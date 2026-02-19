use std::{
    borrow::Cow,
    collections::{btree_map, BTreeMap},
    io::Write,
};

use ahash::{HashMap, HashMapExt};
use rbx_dom_weak::{
    types::{
        Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
        ColorSequenceKeypoint, Content, ContentId, ContentType, Enum, EnumItem, Faces, Font,
        MaterialColors, Matrix3, NetAssetRef, NumberRange, NumberSequence, NumberSequenceKeypoint,
        PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities, SharedString, Tags, UDim, UDim2,
        UniqueId, Variant, VariantType, Vector2, Vector3, Vector3int16,
    },
    Instance, Ustr, UstrMap, WeakDom,
};

use rbx_reflection::{
    ClassDescriptor, ClassTag, PropertyKind, PropertyMigration, PropertySerialization,
    ReflectionDatabase,
};

use crate::{
    chunk::ChunkBuilder,
    core::{
        find_property_descriptors, PropertyDescriptors, RbxWriteExt, FILE_MAGIC_HEADER,
        FILE_SIGNATURE, FILE_VERSION,
    },
    types::Type,
    Serializer,
};

use super::error::InnerError;
use super::CompressionType;

static FILE_FOOTER: &[u8] = b"</roblox>";

/// Represents all of the state during a single serialization session. A new
/// `BinarySerializer` object should be created every time we want to serialize
/// a binary model file.
pub(super) struct SerializerState<'dom, 'db, W> {
    serializer: &'db Serializer<'db>,

    /// The dom containing all of the instances that we're serializing.
    dom: &'dom WeakDom,

    /// Where the binary output should be written.
    output: W,

    /// All of the instances, in a deterministic order, that we're going to be
    /// serializing.
    relevant_instances: Vec<Ref>,

    /// A map from rbx-dom's unique instance ID (Ref) to the ID space used in
    /// the binary model format, signed integers.
    id_to_referent: HashMap<Ref, i32>,

    /// All of the types of instance discovered by our serializer that we'll be
    /// writing into the output.
    type_infos: TypeInfos<'dom, 'db>,

    /// All of the SharedStrings in the DOM, in the order they'll be written
    // in.
    shared_strings: Vec<SharedString>,

    /// A map of SharedStrings to where it is in the SSTR chunk. This is used
    /// for writing PROP chunks.
    shared_string_ids: HashMap<SharedString, u32>,
}

/// An instance class that our serializer knows about. We should have one struct
/// per unique ClassName.
#[derive(Debug)]
struct TypeInfo<'dom, 'db> {
    /// The ID that this serializer will use to refer to this type of instance.
    type_id: u32,

    /// Whether this type is considered a service. Only one copy of a given
    /// service can exist for a given ServiceProvider. DataModel is the only
    /// ServiceProvider in most projects.
    is_service: bool,

    /// All of the instances referenced by this type.
    instances: Vec<&'dom Instance>,

    /// All of the defined properties for this type found on any instance of
    /// this type. Only one entry should be present for each logical property.
    ///
    /// Sorted by canonical name just before serialization to ensure that
    /// we write out properties in a deterministic order.
    properties: Vec<PropInfo<'dom>>,

    /// A reference to the type's class descriptor from rbx_reflection, if this
    /// is a known class.
    class_descriptor: Option<&'db ClassDescriptor<'db>>,

    /// A set containing the properties that we have seen so far in the file and
    /// processed. This helps us avoid traversing the reflection database
    /// multiple times if there are many copies of the same kind of instance.
    /// This acts as the key to `self.properties`.  It is None for properties
    /// that do not serialize.
    properties_visited: UstrMap<Option<usize>>,
}

/// A property on a specific class that our serializer knows about.
///
/// We should have one `PropInfo` per logical property per class that is used in
/// the document we are serializing. This means that even if `BasePart.Size` and
/// `BasePart.size` are present in the same document, they should share a
/// `PropInfo` as they are the same logical property.
#[derive(Debug)]
struct PropInfo<'dom> {
    /// The binary format type ID that will be use to serialize this property.
    /// This type is related to the type of the serialized form of the logical
    /// property, but is not 1:1.
    ///
    /// For example, a property marked to serialize as a
    /// `VariantType::BinaryString` will serialize as `Type::String`, the same
    /// as the `Content` and `String` variants do.
    prop_type: Type,

    /// The canonical name for this property. This is used to sort the
    /// logical property list just before serialization.
    canonical_name: Ustr,

    /// The serialized name for this property. This is the name that is actually
    /// written as part of the PROP chunk and may not line up with the canonical
    /// name for the property.
    serialized_name: Ustr,

    /// References to logical property values.  May be collected from multiple
    /// property names like `BasePart.Size` and `BasePart.size`.
    values: Vec<&'dom Variant>,

    /// The default value for this property that should be used if any instances
    /// are missing this property.
    ///
    /// With the exception of newly-added properties, Roblox Studio will create
    /// files with instances that contain every property. When mixing old and
    /// newly-saved instances, or mixing instances generated from other tools,
    /// some properties may be missing. They will be populated from this value.
    ///
    /// Default values are first populated from the reflection database, if
    /// present, followed by an educated guess based on the type of the value.
    default_value: &'dom Variant,

    /// If a logical property has a migration associated with it (i.e. BrickColor ->
    /// Color, Font -> FontFace), this field contains Some(PropertyMigration). Otherwise,
    /// it is None.
    migration: Option<&'dom PropertyMigration>,
}
impl<'dom> PropInfo<'dom> {
    /// This function extends `self.values` with `self.default_value` values.
    /// Previous instances may not have traversed all properties, but
    /// all `PropInfo.values` must have the same length as
    /// `TypeInfo.instances.len()` to serialize PROP chunks correctly.
    fn extend_with_default(&mut self, desired_len: usize) {
        let current_len = self.values.len();
        let Some(additional) = desired_len.checked_sub(current_len) else {
            panic!(
                "desired_len ({}) must be greater than or equal to current_len ({})",
                desired_len, current_len
            );
        };
        self.values
            .extend(core::iter::repeat_n(self.default_value, additional));
    }
    /// Set the migration
    fn set_migration(&mut self, m_new: &'dom PropertyMigration) {
        if let Some(m_old) = self.migration {
            // Check that migrations do not conflict if one already exists
            assert_eq!(m_old, m_new, "Migration must not change after being set");
        } else {
            self.migration = Some(m_new);
        }
    }

    /// Helper function for `TypeInfo::get_or_create_logical_property`.
    /// This is separated out to reduce the amount of things in the function body.
    fn new(
        SerializationInfo {
            migration,
            canonical_name,
            serialized_name,
            serialized_ty,
        }: SerializationInfo<'dom>,
        class_descriptor: Option<&'dom ClassDescriptor<'dom>>,
        push_sstr: &mut impl FnMut(&Variant),
        database: &'dom ReflectionDatabase<'dom>,
        type_name: Ustr,
    ) -> Result<PropInfo<'dom>, InnerError> {
        let Some(prop_type) = Type::from_rbx_type(serialized_ty) else {
            // This is a known value type, but rbx_binary doesn't have a
            // binary type value for it. rbx_binary might be out of
            // date?
            return Err(InnerError::UnsupportedPropType {
                type_name: type_name.to_string(),
                prop_name: serialized_name.to_string(),
                prop_type: format!("{:?}", serialized_ty),
            });
        };

        let default_value = class_descriptor
            .and_then(|class| database.find_default_property(class, &canonical_name))
            .or_else(|| fallback_default_value(serialized_ty))
            .ok_or_else(|| {
                // Since we don't know how to generate the default value
                // for this property, we consider it unsupported.
                InnerError::UnsupportedPropType {
                    type_name: type_name.to_string(),
                    prop_name: canonical_name.to_string(),
                    prop_type: format!("{:?}", serialized_ty),
                }
            })?;

        // There's no assurance that the default SharedString value
        // will actually get serialized inside of the SSTR chunk, so we
        // check here just to make sure.
        push_sstr(default_value);

        Ok(PropInfo {
            prop_type,
            canonical_name,
            serialized_name,
            values: Vec::new(),
            default_value,
            migration,
        })
    }
}

/// Contains all of the `TypeInfo` objects known to the serializer so far. This
/// struct was broken out to help encapsulate the behavior here and to ease
/// self-borrowing issues from BinarySerializer getting too large.
#[derive(Debug)]
struct TypeInfos<'dom, 'db> {
    database: &'db ReflectionDatabase<'db>,
    /// A map containing one entry for each unique ClassName discovered in the
    /// DOM.
    ///
    /// These are stored sorted so that we naturally iterate over them in order
    /// and improve our chances of being deterministic.
    values: BTreeMap<Ustr, TypeInfo<'dom, 'db>>,

    /// The next type ID that should be assigned if a type is discovered and
    /// added to the serializer.
    next_type_id: u32,
}

impl<'dom, 'db> TypeInfos<'dom, 'db> {
    fn new(database: &'db ReflectionDatabase<'db>) -> Self {
        Self {
            database,
            values: BTreeMap::new(),
            next_type_id: 0,
        }
    }

    /// Finds the type info from the given ClassName if it exists, or creates
    /// one and returns a reference to it if not.
    fn get_or_create(&mut self, class: Ustr) -> &mut TypeInfo<'dom, 'db> {
        if let btree_map::Entry::Vacant(entry) = self.values.entry(class) {
            let type_id = self.next_type_id;
            self.next_type_id += 1;

            let class_descriptor = self.database.classes.get(class.as_str());

            let is_service = if let Some(descriptor) = &class_descriptor {
                descriptor.tags.contains(&ClassTag::Service)
            } else {
                log::info!("The class {class} is not known to rbx_binary");
                false
            };

            entry.insert(TypeInfo {
                type_id,
                is_service,
                instances: Vec::new(),
                properties: Vec::new(),
                class_descriptor,
                properties_visited: UstrMap::new(),
            });
        }

        // This unwrap will not panic because we always insert this key into
        // type_infos in this function.
        self.values.get_mut(&class).unwrap()
    }
}

struct SerializationInfo<'db> {
    migration: Option<&'db PropertyMigration>,
    canonical_name: Ustr,
    serialized_name: Ustr,
    serialized_ty: VariantType,
}

impl<'db> SerializationInfo<'db> {
    /// Helper function for `TypeInfo::get_or_create_logical_property`.
    /// This is separated out to utilize `return`.
    fn new(
        class_descriptor: Option<&'db ClassDescriptor<'db>>,
        database: &'db ReflectionDatabase<'db>,
        prop_name: Ustr,
        sample_value: &Variant,
    ) -> Option<SerializationInfo<'db>> {
        let canonical_name;
        let serialized_name;
        let serialized_ty;
        let mut migration = None;

        match find_property_descriptors(database, class_descriptor, &prop_name) {
            Some((superclass_descriptor, descriptors)) => {
                // For any properties that do not serialize, we can skip
                // adding them to the set of type_infos.
                let serialized = match descriptors.serialized {
                    Some(descriptor) => {
                        if let PropertyKind::Canonical {
                            serialization: PropertySerialization::Migrate(prop_migration),
                        } = &descriptor.kind
                        {
                            // If the property migrates, we need to look up the
                            // property it should migrate to and use the reflection
                            // information of the new property instead of the old
                            // property, because migrated properties should not
                            // serialize
                            //
                            // Assume that the migration will always be
                            // directed to a property on the same class.
                            // This avoids re-walking the superclasses.
                            let new_descriptors = superclass_descriptor
                                .properties
                                .get(prop_migration.new_property_name.as_str())
                                .and_then(|prop| {
                                    PropertyDescriptors::new(superclass_descriptor, prop)
                                });

                            migration = Some(prop_migration);

                            match new_descriptors {
                                Some(descriptor) => match descriptor.serialized {
                                    Some(serialized) => {
                                        canonical_name = descriptor.canonical.name.as_ref().into();
                                        serialized
                                    }
                                    None => return None,
                                },
                                None => return None,
                            }
                        } else {
                            canonical_name = descriptors.canonical.name.as_ref().into();
                            descriptor
                        }
                    }
                    None => return None,
                };

                serialized_name = serialized.name.as_ref().into();

                serialized_ty = serialized.data_type.ty();
            }

            None => {
                canonical_name = prop_name;
                serialized_name = prop_name;
                serialized_ty = sample_value.ty();
            }
        }

        Some(SerializationInfo {
            migration,
            canonical_name,
            serialized_name,
            serialized_ty,
        })
    }
}

impl<'dom, 'db: 'dom> TypeInfo<'dom, 'db> {
    /// Helper function for `TypeInfo::get_or_create_logical_property`.
    /// Push a new PropInfo into properties and add a list of aliases which point to it.
    fn push_prop_info_with_names(
        &mut self,
        prop_info: PropInfo<'dom>,
        prop_names: impl IntoIterator<Item = Ustr>,
    ) -> &mut PropInfo<'dom> {
        let logical_index = self.properties.len();
        self.properties.push(prop_info);
        let prop_info = &mut self.properties[logical_index];
        for prop_name in prop_names {
            self.properties_visited
                .insert(prop_name, Some(logical_index));
        }
        prop_info
    }

    /// Get or create a logical property from a visited property.
    fn get_or_create_logical_property<'a>(
        &'a mut self,
        push_sstr: &mut impl FnMut(&Variant),
        database: &'db ReflectionDatabase<'db>,
        type_name: Ustr,
        prop_name: Ustr,
        sample_value: &Variant,
    ) -> Result<Option<&'a mut PropInfo<'dom>>, InnerError> {
        let class = self.class_descriptor;

        // Check if prop_name is already in properties_visited, return it
        if let Some(&logical_index) = self.properties_visited.get(&prop_name) {
            let prop_info = match logical_index {
                Some(logical_index) => Some(&mut self.properties[logical_index]),
                // Does not serialize, no logical property
                None => None,
            };
            return Ok(prop_info);
        }

        let Some(ser_info) = SerializationInfo::new(class, database, prop_name, sample_value)
        else {
            // Remember that this visited property does not serialize
            self.properties_visited.insert(prop_name, None);
            return Ok(None);
        };

        // Is this property the canonical representation?
        let canonical_name = ser_info.canonical_name;
        if canonical_name != prop_name {
            // Check if canonical name is already in properties_visited, return it
            if let Some(&logical_index) = self.properties_visited.get(&canonical_name) {
                // Add the new alias
                self.properties_visited.insert(prop_name, logical_index);

                let prop_info = match logical_index {
                    Some(logical_index) => {
                        let prop_info = &mut self.properties[logical_index];
                        // The visited property may contain a migration that
                        // the logical property has not been made aware of yet.
                        if let Some(migration) = ser_info.migration {
                            prop_info.set_migration(migration);
                        }
                        Some(prop_info)
                    }
                    // Does not serialize, no logical property
                    None => None,
                };

                return Ok(prop_info);
            }
            // Create logical property with two new names
            let prop_info = PropInfo::new(ser_info, class, push_sstr, database, type_name)?;
            let prop_info = self.push_prop_info_with_names(prop_info, [canonical_name, prop_name]);
            Ok(Some(prop_info))
        } else {
            // Create logical property with one new name
            let prop_info = PropInfo::new(ser_info, class, push_sstr, database, type_name)?;
            let prop_info = self.push_prop_info_with_names(prop_info, [prop_name]);
            Ok(Some(prop_info))
        }
    }
}

impl<'dom, 'db: 'dom, W: Write> SerializerState<'dom, 'db, W> {
    pub fn new(serializer: &'db Serializer<'db>, dom: &'dom WeakDom, output: W) -> Self {
        SerializerState {
            serializer,
            dom,
            output,
            relevant_instances: Vec::new(),
            id_to_referent: HashMap::new(),
            type_infos: TypeInfos::new(serializer.database),
            shared_strings: Vec::new(),
            shared_string_ids: HashMap::new(),
        }
    }

    /// Mark the given instance IDs and all of their descendants as intended for
    /// serialization with this serializer.
    #[profiling::function]
    pub fn add_instances(&mut self, referents: &[Ref]) -> Result<(), InnerError> {
        // Populate relevant_instances with a depth-first post-order traversal over the
        // tree(s). This is important to ensure that the order of the PRNT chunk (later
        // written by SerializerState::serialize_parents) is correct.

        // The implementation here slightly deviates from Roblox. Roblox writes the PRNT
        // in depth-first post-order, numbers referents in depth-first pre-order, and
        // generates type infos in lexical order by class name. See
        // https://github.com/rojo-rbx/rbx-dom/pull/411#issuecomment-2103713517

        // Since it seems only the PRNT chunk has important semantics related to its
        // ordering, we do one tree traversal in this function, thereby numbering
        // referents, generating type infos, and writing the PRNT chunk all in depth-first
        // post-order.
        let mut to_visit = Vec::new();
        let mut last_visited_child = None;

        to_visit.extend(referents.iter().rev());

        while let Some(referent) = to_visit.last() {
            let instance = self
                .dom
                .get_by_ref(*referent)
                .ok_or(InnerError::InvalidInstanceId {
                    referent: *referent,
                })?;

            to_visit.extend(instance.children().iter().rev());

            while let Some(referent) = to_visit.last() {
                let instance =
                    self.dom
                        .get_by_ref(*referent)
                        .ok_or(InnerError::InvalidInstanceId {
                            referent: *referent,
                        })?;

                if !instance.children().is_empty()
                    && instance.children().last() != last_visited_child.as_ref()
                {
                    break;
                }

                self.relevant_instances.push(*referent);
                self.collect_type_info(instance)?;
                last_visited_child = to_visit.pop();
            }
        }

        // Sort shared_strings by their hash, to ensure they are deterministically added
        // into the SSTR chunk, then assign them corresponding ids
        self.shared_strings.sort_by_key(SharedString::hash);
        for (id, shared_string) in self.shared_strings.iter().cloned().enumerate() {
            self.shared_string_ids.insert(shared_string, id as u32);
        }

        log::debug!(
            "Discovered {} unique TypeInfos",
            self.type_infos.values.len()
        );

        Ok(())
    }

    /// Collect information about all the different types of instance and their
    /// properties.
    // Using the entry API here, as Clippy suggests, would require us to
    // clone canonical_name in a cold branch. We don't want to do that.
    #[allow(clippy::map_entry)]
    #[profiling::function]
    pub fn collect_type_info(&mut self, instance: &'dom Instance) -> Result<(), InnerError> {
        let SerializerState {
            serializer: Serializer { database, .. },
            type_infos,
            shared_strings,
            shared_string_ids,
            ..
        } = self;

        let type_info = type_infos.get_or_create(instance.class);
        // The desired length of all PropInfo.values in this TypeInfo.
        // Some instances may have missing properties, meaning the
        // corresponding PropInfo is never visited and no value is inserted.
        //
        // Used in the loop below (extend_with_default)
        let desired_len = type_info.instances.len();
        type_info.instances.push(instance);

        // Helper to track a SharedString Variant
        let mut push_sstr = |variant: &Variant| {
            if let Variant::SharedString(sstr) = variant {
                if !shared_string_ids.contains_key(sstr) {
                    shared_string_ids.insert(sstr.clone(), 0);
                    shared_strings.push(sstr.clone());
                }
            } else if let Variant::NetAssetRef(net) = variant {
                // NetAssetRef is serialized identically as `SharedString` and
                // uses the same repository, so we just treat them all the same
                let sstr_ref = net.as_ref();
                if !shared_string_ids.contains_key(sstr_ref) {
                    shared_string_ids.insert(sstr_ref.clone(), 0);
                    shared_strings.push(sstr_ref.clone())
                }
            }
        };

        for (prop_name, prop_value) in &instance.properties {
            let Some(logical_property) = type_info.get_or_create_logical_property(
                &mut push_sstr,
                database,
                instance.class,
                *prop_name,
                prop_value,
            )?
            else {
                continue;
            };

            // Discover and track any shared strings we come across.
            push_sstr(prop_value);

            // Add default values until the desired len is reached.
            // This happens when instances of the same class have different
            // sets of properties.  This is cheaper than checking
            // for missing properties using set differences.
            logical_property.extend_with_default(desired_len);

            // Append value reference to prop_info.values.  This avoids
            // getting all properties of all instances again in `serialize_properties`.
            logical_property.values.push(prop_value);
        }

        Ok(())
    }

    /// Populate the map from rbx-dom's instance ID space to the IDs that we'll
    /// be serializing to the model.
    #[profiling::function]
    pub fn generate_referents(&mut self) {
        // This performs the same check as next_referent.try_into().unwrap() but only once.
        assert!(self.relevant_instances.len() <= i32::MAX as usize);

        self.id_to_referent.extend(
            self.relevant_instances
                .iter()
                .enumerate()
                .map(|(next_referent, id)| (*id, next_referent as i32)),
        );

        log::debug!("Collected {} referents", self.id_to_referent.len());
    }

    pub fn write_header(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing header");

        self.output.write_all(FILE_MAGIC_HEADER)?;
        self.output.write_all(FILE_SIGNATURE)?;
        self.output.write_le_u16(FILE_VERSION)?;

        self.output
            .write_le_u32(self.type_infos.values.len() as u32)?;
        self.output
            .write_le_u32(self.relevant_instances.len() as u32)?;
        self.output.write_all(&[0; 8])?;

        Ok(())
    }

    /// Write out any metadata about this file, stored in a chunk named META.
    pub fn serialize_metadata(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing metadata (currently no-op)");
        // TODO: There is no concept of metadata in a dom yet.
        Ok(())
    }

    /// Write out all of the SharedStrings in this file, if any exist,
    /// stored in a chunk named SSTR.
    #[profiling::function]
    pub fn serialize_shared_strings(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing shared string chunk");

        if self.shared_strings.is_empty() {
            return Ok(());
        }

        let mut chunk = ChunkBuilder::new(b"SSTR", self.serializer.compression);

        chunk.write_le_u32(0)?; // SSTR version number
        chunk.write_le_u32(self.shared_strings.len() as u32)?;

        for shared_string in &self.shared_strings {
            // Better to write nothing than write half a hash
            chunk.write_all(&[0; 16])?;
            chunk.write_binary_string(shared_string.data())?;
        }

        chunk.dump(&mut self.output)?;

        Ok(())
    }

    /// Write out the declarations of all instances, stored in a series of
    /// chunks named INST.
    #[profiling::function]
    pub fn serialize_instances(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing instance chunks");

        for (type_name, type_info) in &self.type_infos.values {
            log::trace!(
                "Writing chunk for {} ({} instances)",
                type_name,
                type_info.instances.len()
            );

            let mut chunk = ChunkBuilder::new(b"INST", self.serializer.compression);

            chunk.write_le_u32(type_info.type_id)?;
            chunk.write_string(type_name)?;

            // It's possible that this integer will be expanded in the future to
            // be a general version/format field instead of just service vs
            // non-service.
            //
            // At that point, we'll start thinking about it like it's a u8
            // instead of a bool.
            chunk.write_bool(type_info.is_service)?;

            chunk.write_le_u32(type_info.instances.len() as u32)?;

            chunk.write_referent_array(
                type_info
                    .instances
                    .iter()
                    .map(|instance| self.id_to_referent[&instance.referent()]),
            )?;

            if type_info.is_service {
                // It's unclear what this byte is used for, but when the type is
                // a service (like Workspace, Lighting, etc), we need to write
                // the value `1` for every instance in our file of that type.
                //
                // In 99.9% of cases, there's only going to be one copy of a
                // given service, so we're not worried about doing this super
                // efficiently.
                for _ in 0..type_info.instances.len() {
                    chunk.write_u8(1)?;
                }
            }

            chunk.dump(&mut self.output)?;
        }

        Ok(())
    }

    /// Write out batch declarations of property values for the instances
    /// previously defined in the INST chunks. Property data is contained in
    /// chunks named PROP.
    #[profiling::function]
    pub fn serialize_properties(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing properties");

        let name_ustr = rbx_dom_weak::ustr("Name");
        for (type_name, type_info) in &mut self.type_infos.values {
            // Sort logical properties by canonical name
            type_info
                .properties
                .sort_by_key(|prop_info| prop_info.canonical_name);

            // Locate the index where "Name" could be inserted
            let name_location = type_info
                .properties
                .binary_search_by_key(&name_ustr, |prop_info| prop_info.canonical_name);

            let (properties_before_name, properties_after_name) = match name_location {
                // Split properties at the sort location of "Name"
                Err(name_insert_index) => type_info.properties.split_at_mut(name_insert_index),
                // "Name" logical property exists.  Ignore it.
                Ok(name_index) => {
                    log::warn!("Name property should not exist in Instance.properties. Use Instance.name instead. Property was ignored.");

                    let (properties_before_name, properties_after_name) =
                        type_info.properties.split_at_mut(name_index);

                    // Skip "Name" logical property
                    (properties_before_name, &mut properties_after_name[1..])
                }
            };

            for prop_info in properties_before_name {
                let mut chunk = ChunkBuilder::new(b"PROP", self.serializer.compression);

                chunk.write_le_u32(type_info.type_id)?;
                chunk.write_string(&prop_info.serialized_name)?;
                chunk.write_u8(prop_info.prop_type as u8)?;

                write_prop_info(
                    prop_info,
                    &mut chunk,
                    self.dom,
                    &self.id_to_referent,
                    &self.shared_string_ids,
                    &type_info.instances,
                    type_name,
                )?;

                chunk.dump(&mut self.output)?;
            }

            // Write name properties as a special case
            {
                let mut chunk = ChunkBuilder::new(b"PROP", self.serializer.compression);

                chunk.write_le_u32(type_info.type_id)?;
                chunk.write_string("Name")?;
                chunk.write_u8(Type::String as u8)?;

                for &instance in &type_info.instances {
                    chunk.write_string(&instance.name)?;
                }

                chunk.dump(&mut self.output)?;
            }

            for prop_info in properties_after_name {
                let mut chunk = ChunkBuilder::new(b"PROP", self.serializer.compression);

                chunk.write_le_u32(type_info.type_id)?;
                chunk.write_string(&prop_info.serialized_name)?;
                chunk.write_u8(prop_info.prop_type as u8)?;

                write_prop_info(
                    prop_info,
                    &mut chunk,
                    self.dom,
                    &self.id_to_referent,
                    &self.shared_string_ids,
                    &type_info.instances,
                    type_name,
                )?;

                chunk.dump(&mut self.output)?;
            }

            fn write_prop_info<'dom>(
                prop_info: &mut PropInfo<'dom>,
                chunk: &mut ChunkBuilder,
                dom: &'dom WeakDom,
                id_to_referent: &HashMap<Ref, i32>,
                shared_string_ids: &HashMap<SharedString, u32>,
                instances: &[&Instance],
                type_name: &str,
            ) -> Result<(), InnerError> {
                profiling::scope!("serialize property", &prop_info.canonical_name);
                log::trace!(
                    "Writing property {}.{} (type {:?})",
                    type_name,
                    prop_info.canonical_name,
                    prop_info.prop_type
                );

                // Ensure the number of values matches the number of referents.
                // This happens when instances of the same class have different
                // sets of properties.
                prop_info.extend_with_default(instances.len());

                // Helper to generate a type mismatch error with context from
                // this chunk.
                let type_mismatch =
                    |i: usize, bad_value: &Variant, valid_type_names: &'static str| {
                        Err(InnerError::PropTypeMismatch {
                            type_name: type_name.to_string(),
                            prop_name: prop_info.canonical_name.to_string(),
                            valid_type_names,
                            actual_type_name: format!("{:?}", bad_value.ty()),
                            instance_full_name: dom.full_path_of(instances[i].referent(), "."),
                        })
                    };

                let invalid_value = |i: usize, bad_value: &Variant| InnerError::InvalidPropValue {
                    instance_full_name: dom.full_path_of(instances[i].referent(), "."),
                    type_name: type_name.to_string(),
                    prop_name: prop_info.canonical_name.to_string(),
                    prop_type: format!("{:?}", bad_value.ty()),
                };

                if let Some(property_migration) = prop_info.migration {
                    let migrated_values: Vec<_> = prop_info
                        .values
                        .iter()
                        .map(|&value| {
                            property_migration
                                .perform(value)
                                // Take original if migration failed
                                .map_or(Cow::Borrowed(value), Cow::Owned)
                        })
                        .collect();

                    write_prop_values(
                        chunk,
                        id_to_referent,
                        shared_string_ids,
                        prop_info.prop_type,
                        migrated_values.iter().map(Cow::as_ref).enumerate(),
                        type_mismatch,
                        invalid_value,
                    )?;
                } else {
                    write_prop_values(
                        chunk,
                        id_to_referent,
                        shared_string_ids,
                        prop_info.prop_type,
                        prop_info.values.iter().copied().enumerate(),
                        type_mismatch,
                        invalid_value,
                    )?;
                };

                Ok(())
            }
            fn write_prop_values<'a, I, TypeMismatch, InvalidValue>(
                chunk: &mut ChunkBuilder,
                id_to_referent: &HashMap<Ref, i32>,
                shared_string_ids: &HashMap<SharedString, u32>,
                prop_type: Type,
                values: I,
                type_mismatch: TypeMismatch,
                invalid_value: InvalidValue,
            ) -> Result<(), InnerError>
            where
                I: IntoIterator<Item = (usize, &'a Variant)>,
                I: ExactSizeIterator,
                TypeMismatch: Fn(usize, &Variant, &'static str) -> Result<(), InnerError>,
                InvalidValue: Fn(usize, &Variant) -> InnerError,
            {
                match prop_type {
                    Type::String => {
                        for (i, rbx_value) in values {
                            match rbx_value {
                                Variant::String(value) => {
                                    chunk.write_string(value)?;
                                }
                                Variant::ContentId(value) => {
                                    chunk.write_string(value.as_ref())?;
                                }
                                Variant::BinaryString(value) => {
                                    chunk.write_binary_string(value.as_ref())?;
                                }
                                Variant::Tags(value) => {
                                    let buf = value.encode();
                                    chunk.write_binary_string(&buf)?;
                                }
                                Variant::Attributes(value) => {
                                    let mut buf = Vec::new();

                                    value
                                        .to_writer(&mut buf)
                                        .map_err(|_| invalid_value(i, rbx_value))?;

                                    chunk.write_binary_string(&buf)?;
                                }
                                Variant::MaterialColors(value) => {
                                    chunk.write_binary_string(&value.encode())?;
                                }
                                _ => {
                                    return type_mismatch(
                                        i,
                                        rbx_value,
                                        "String, ContentId, Tags, Attributes, MaterialColors, or BinaryString",
                                    );
                                }
                            }
                        }
                    }
                    Type::Bool => {
                        for (i, rbx_value) in values {
                            if let Variant::Bool(value) = rbx_value {
                                chunk.write_bool(*value)?;
                            } else {
                                return type_mismatch(i, rbx_value, "Bool");
                            }
                        }
                    }
                    Type::Int32 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Int32(value) = rbx_value {
                                buf.push(*value);
                            } else {
                                return type_mismatch(i, rbx_value, "Int32");
                            }
                        }

                        chunk.write_interleaved_i32_array(buf)?;
                    }
                    Type::Float32 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Float32(value) = rbx_value {
                                buf.push(*value);
                            } else {
                                return type_mismatch(i, rbx_value, "Float32");
                            }
                        }

                        chunk.write_interleaved_f32_array(buf)?;
                    }
                    Type::Float64 => {
                        for (i, rbx_value) in values {
                            match rbx_value {
                                Variant::Float64(value) => {
                                    chunk.write_le_f64(*value)?;
                                }
                                Variant::Float32(value) => {
                                    chunk.write_le_f64(*value as f64)?;
                                }
                                _ => return type_mismatch(i, rbx_value, "Float64"),
                            }
                        }
                    }
                    Type::UDim => {
                        let mut scale = Vec::with_capacity(values.len());
                        let mut offset = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::UDim(value) = rbx_value {
                                scale.push(value.scale);
                                offset.push(value.offset);
                            } else {
                                return type_mismatch(i, rbx_value, "UDim");
                            }
                        }

                        chunk.write_interleaved_f32_array(scale)?;
                        chunk.write_interleaved_i32_array(offset)?;
                    }
                    Type::UDim2 => {
                        let mut scale_x = Vec::with_capacity(values.len());
                        let mut scale_y = Vec::with_capacity(values.len());
                        let mut offset_x = Vec::with_capacity(values.len());
                        let mut offset_y = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::UDim2(value) = rbx_value {
                                scale_x.push(value.x.scale);
                                scale_y.push(value.y.scale);
                                offset_x.push(value.x.offset);
                                offset_y.push(value.y.offset);
                            } else {
                                return type_mismatch(i, rbx_value, "UDim2");
                            }
                        }

                        chunk.write_interleaved_f32_array(scale_x)?;
                        chunk.write_interleaved_f32_array(scale_y)?;
                        chunk.write_interleaved_i32_array(offset_x)?;
                        chunk.write_interleaved_i32_array(offset_y)?;
                    }
                    Type::Font => {
                        for (i, rbx_value) in values {
                            if let Variant::Font(value) = rbx_value {
                                chunk.write_string(&value.family)?;
                                chunk.write_le_u16(value.weight.as_u16())?;
                                chunk.write_u8(value.style.as_u8())?;
                                chunk.write_string(
                                    value.cached_face_id.as_deref().unwrap_or_default(),
                                )?;
                            } else {
                                return type_mismatch(i, rbx_value, "Font");
                            }
                        }
                    }
                    Type::Ray => {
                        for (i, rbx_value) in values {
                            if let Variant::Ray(value) = rbx_value {
                                chunk.write_le_f32(value.origin.x)?;
                                chunk.write_le_f32(value.origin.y)?;
                                chunk.write_le_f32(value.origin.z)?;
                                chunk.write_le_f32(value.direction.x)?;
                                chunk.write_le_f32(value.direction.y)?;
                                chunk.write_le_f32(value.direction.x)?;
                            } else {
                                return type_mismatch(i, rbx_value, "Ray");
                            }
                        }
                    }
                    Type::Faces => {
                        for (i, rbx_value) in values {
                            if let Variant::Faces(value) = rbx_value {
                                chunk.write_u8(value.bits())?;
                            } else {
                                return type_mismatch(i, rbx_value, "Faces");
                            }
                        }
                    }
                    Type::Axes => {
                        for (i, rbx_value) in values {
                            if let Variant::Axes(value) = rbx_value {
                                chunk.write_u8(value.bits())?;
                            } else {
                                return type_mismatch(i, rbx_value, "Axes");
                            }
                        }
                    }
                    Type::BrickColor => {
                        let mut numbers = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::BrickColor(value) = rbx_value {
                                numbers.push(*value as u32);
                            } else if let Variant::Int32(value) = rbx_value {
                                numbers.push(*value as u32);
                            } else {
                                return type_mismatch(i, rbx_value, "BrickColor");
                            }
                        }

                        chunk.write_interleaved_u32_array(numbers)?;
                    }
                    Type::Color3 => {
                        let mut r = Vec::with_capacity(values.len());
                        let mut g = Vec::with_capacity(values.len());
                        let mut b = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Color3(value) = rbx_value {
                                r.push(value.r);
                                g.push(value.g);
                                b.push(value.b);
                            } else {
                                return type_mismatch(i, rbx_value, "Color3");
                            }
                        }

                        chunk.write_interleaved_f32_array(r)?;
                        chunk.write_interleaved_f32_array(g)?;
                        chunk.write_interleaved_f32_array(b)?;
                    }
                    Type::Vector2 => {
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Vector2(value) = rbx_value {
                                x.push(value.x);
                                y.push(value.y)
                            } else {
                                return type_mismatch(i, rbx_value, "Vector2");
                            }
                        }

                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                    }
                    Type::Vector3 => {
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        let mut z = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Vector3(value) = rbx_value {
                                x.push(value.x);
                                y.push(value.y);
                                z.push(value.z)
                            } else {
                                return type_mismatch(i, rbx_value, "Vector3");
                            }
                        }

                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                        chunk.write_interleaved_f32_array(z)?;
                    }
                    Type::CFrame => {
                        let mut rotations = Vec::with_capacity(values.len());
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        let mut z = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::CFrame(value) = rbx_value {
                                rotations.push(value.orientation);
                                x.push(value.position.x);
                                y.push(value.position.y);
                                z.push(value.position.z);
                            } else {
                                return type_mismatch(i, rbx_value, "CFrame");
                            }
                        }

                        for matrix in rotations {
                            if let Some(id) = matrix.to_basic_rotation_id() {
                                chunk.write_u8(id)?;
                            } else {
                                chunk.write_u8(0x00)?;

                                chunk.write_le_f32(matrix.x.x)?;
                                chunk.write_le_f32(matrix.x.y)?;
                                chunk.write_le_f32(matrix.x.z)?;

                                chunk.write_le_f32(matrix.y.x)?;
                                chunk.write_le_f32(matrix.y.y)?;
                                chunk.write_le_f32(matrix.y.z)?;

                                chunk.write_le_f32(matrix.z.x)?;
                                chunk.write_le_f32(matrix.z.y)?;
                                chunk.write_le_f32(matrix.z.z)?;
                            }
                        }

                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                        chunk.write_interleaved_f32_array(z)?;
                    }
                    Type::Enum => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            match rbx_value {
                                Variant::Enum(value) => buf.push(value.to_u32()),
                                Variant::EnumItem(EnumItem { value, .. }) => buf.push(*value),
                                _ => return type_mismatch(i, rbx_value, "Enum or EnumItem"),
                            }
                        }

                        chunk.write_interleaved_u32_array(buf)?;
                    }
                    Type::Ref => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Ref(value) = rbx_value {
                                if let Some(id) = id_to_referent.get(value) {
                                    buf.push(*id);
                                } else {
                                    buf.push(-1);
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "Ref");
                            }
                        }

                        chunk.write_referent_array(buf)?;
                    }
                    Type::Vector3int16 => {
                        for (i, rbx_value) in values {
                            if let Variant::Vector3int16(value) = rbx_value {
                                chunk.write_le_i16(value.x)?;
                                chunk.write_le_i16(value.y)?;
                                chunk.write_le_i16(value.z)?;
                            } else {
                                return type_mismatch(i, rbx_value, "Vector3int16");
                            }
                        }
                    }
                    Type::NumberSequence => {
                        for (i, rbx_value) in values {
                            if let Variant::NumberSequence(value) = rbx_value {
                                chunk.write_le_u32(value.keypoints.len() as u32)?;

                                for keypoint in &value.keypoints {
                                    chunk.write_le_f32(keypoint.time)?;
                                    chunk.write_le_f32(keypoint.value)?;
                                    chunk.write_le_f32(keypoint.envelope)?;
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "NumberSequence");
                            }
                        }
                    }
                    Type::ColorSequence => {
                        for (i, rbx_value) in values {
                            if let Variant::ColorSequence(value) = rbx_value {
                                chunk.write_le_u32(value.keypoints.len() as u32)?;

                                for keypoint in &value.keypoints {
                                    chunk.write_le_f32(keypoint.time)?;
                                    chunk.write_le_f32(keypoint.color.r)?;
                                    chunk.write_le_f32(keypoint.color.g)?;
                                    chunk.write_le_f32(keypoint.color.b)?;

                                    // write out a dummy value for envelope, which is serialized but doesn't do anything
                                    chunk.write_le_f32(0.0)?;
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "ColorSequence");
                            }
                        }
                    }
                    Type::NumberRange => {
                        for (i, rbx_value) in values {
                            if let Variant::NumberRange(value) = rbx_value {
                                chunk.write_le_f32(value.min)?;
                                chunk.write_le_f32(value.max)?;
                            } else {
                                return type_mismatch(i, rbx_value, "NumberRange");
                            }
                        }
                    }
                    Type::Rect => {
                        let mut x_min = Vec::with_capacity(values.len());
                        let mut y_min = Vec::with_capacity(values.len());
                        let mut x_max = Vec::with_capacity(values.len());
                        let mut y_max = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Rect(value) = rbx_value {
                                x_min.push(value.min.x);
                                y_min.push(value.min.y);
                                x_max.push(value.max.x);
                                y_max.push(value.max.y);
                            } else {
                                return type_mismatch(i, rbx_value, "Rect");
                            }
                        }

                        chunk.write_interleaved_f32_array(x_min)?;
                        chunk.write_interleaved_f32_array(y_min)?;
                        chunk.write_interleaved_f32_array(x_max)?;
                        chunk.write_interleaved_f32_array(y_max)?;
                    }
                    Type::PhysicalProperties => {
                        for (i, rbx_value) in values {
                            if let Variant::PhysicalProperties(value) = rbx_value {
                                // So that we better roundtrip files, we only
                                // write the AcousticAbsorption bit if we
                                // have to.
                                match value {
                                    PhysicalProperties::Default => {
                                        chunk.write_u8(0b00)?;
                                    }
                                    PhysicalProperties::Custom(props) => {
                                        let skip_acoustics = props.acoustic_absorption() == 1.0;
                                        chunk.write_u8(if skip_acoustics { 0b01 } else { 0b11 })?;
                                        chunk.write_le_f32(props.density())?;
                                        chunk.write_le_f32(props.friction())?;
                                        chunk.write_le_f32(props.elasticity())?;
                                        chunk.write_le_f32(props.friction_weight())?;
                                        chunk.write_le_f32(props.elasticity_weight())?;
                                        if !skip_acoustics {
                                            chunk.write_le_f32(props.acoustic_absorption())?;
                                        }
                                    }
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "PhysicalProperties");
                            }
                        }
                    }
                    Type::Color3uint8 => {
                        let mut r = Vec::with_capacity(values.len());
                        let mut g = Vec::with_capacity(values.len());
                        let mut b = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            match rbx_value {
                                Variant::Color3uint8(value) => {
                                    r.push(value.r);
                                    g.push(value.g);
                                    b.push(value.b);
                                }
                                Variant::Color3(value) => {
                                    let color: Color3uint8 = (*value).into();

                                    r.push(color.r);
                                    g.push(color.g);
                                    b.push(color.b);
                                }
                                _ => return type_mismatch(i, rbx_value, "Color3uint8 or Color3"),
                            }
                        }

                        chunk.write_all(r.as_slice())?;
                        chunk.write_all(g.as_slice())?;
                        chunk.write_all(b.as_slice())?;
                    }
                    Type::Int64 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            match rbx_value {
                                Variant::Int64(value) => {
                                    buf.push(*value);
                                }
                                Variant::Int32(value) => {
                                    buf.push(*value as i64);
                                }
                                _ => return type_mismatch(i, rbx_value, "Int64"),
                            }
                        }

                        chunk.write_interleaved_i64_array(buf)?;
                    }
                    Type::SharedString => {
                        let mut entries = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::SharedString(value) = rbx_value {
                                if let Some(id) = shared_string_ids.get(value) {
                                    entries.push(*id);
                                } else {
                                    panic!(
                                        "SharedString {} was not found during type collection",
                                        value.hash()
                                    )
                                }
                            } else if let Variant::NetAssetRef(value) = rbx_value {
                                let sstr_ref = value.as_ref();
                                if let Some(id) = shared_string_ids.get(sstr_ref) {
                                    entries.push(*id)
                                } else {
                                    panic!(
                                        "NetAssetRef {} was not found during type collection",
                                        value.hash()
                                    );
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "SharedString or NetAssetRef");
                            }
                        }

                        chunk.write_interleaved_u32_array(entries)?;
                    }
                    Type::OptionalCFrame => {
                        let mut rotations = Vec::with_capacity(values.len());
                        let mut bools = Vec::with_capacity(values.len());
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());
                        let mut z = Vec::with_capacity(values.len());

                        chunk.write_u8(Type::CFrame as u8)?;

                        for (i, rbx_value) in values {
                            if let Variant::OptionalCFrame(value) = rbx_value {
                                if let Some(value) = value {
                                    rotations.push(value.orientation);
                                    x.push(value.position.x);
                                    y.push(value.position.y);
                                    z.push(value.position.z);
                                    bools.push(0x01);
                                } else {
                                    rotations.push(Matrix3::identity());
                                    x.push(0.0);
                                    y.push(0.0);
                                    z.push(0.0);
                                    bools.push(0x00);
                                }
                            } else {
                                return type_mismatch(i, rbx_value, "OptionalCFrame");
                            }
                        }

                        for matrix in rotations {
                            if let Some(id) = matrix.to_basic_rotation_id() {
                                chunk.write_u8(id)?;
                            } else {
                                chunk.write_u8(0x00)?;

                                chunk.write_le_f32(matrix.x.x)?;
                                chunk.write_le_f32(matrix.x.y)?;
                                chunk.write_le_f32(matrix.x.z)?;

                                chunk.write_le_f32(matrix.y.x)?;
                                chunk.write_le_f32(matrix.y.y)?;
                                chunk.write_le_f32(matrix.y.z)?;

                                chunk.write_le_f32(matrix.z.x)?;
                                chunk.write_le_f32(matrix.z.y)?;
                                chunk.write_le_f32(matrix.z.z)?;
                            }
                        }

                        chunk.write_interleaved_f32_array(x)?;
                        chunk.write_interleaved_f32_array(y)?;
                        chunk.write_interleaved_f32_array(z)?;

                        chunk.write_u8(Type::Bool as u8)?;
                        chunk.write_all(bools.as_slice())?;
                    }
                    Type::UniqueId => {
                        let mut blobs = Vec::with_capacity(values.len());
                        for (i, rbx_value) in values {
                            if let Variant::UniqueId(value) = rbx_value {
                                let mut blob = [0; 16];
                                // This is maybe not the best solution to this
                                // but we can always change it.
                                blob[0..4].copy_from_slice(&value.index().to_be_bytes());
                                blob[4..8].copy_from_slice(&value.time().to_be_bytes());
                                blob[8..]
                                    .copy_from_slice(&value.random().rotate_left(1).to_be_bytes());
                                blobs.push(blob);
                            } else {
                                return type_mismatch(i, rbx_value, "UniqueId");
                            }
                        }

                        chunk.write_interleaved_bytes::<16, _>(blobs)?;
                    }
                    Type::SecurityCapabilities => {
                        let mut capabilities = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::SecurityCapabilities(value) = rbx_value {
                                capabilities.push(value.bits() as i64)
                            } else {
                                return type_mismatch(i, rbx_value, "SecurityCapabilities");
                            }
                        }

                        chunk.write_interleaved_i64_array(capabilities)?;
                    }
                    Type::Content => {
                        let mut source_types = Vec::with_capacity(values.len());
                        let mut uris = Vec::with_capacity(values.len());
                        let mut objects = Vec::new();
                        for (i, rbx_value) in values {
                            if let Variant::Content(content) = rbx_value {
                                source_types.push(match content.value() {
                                    ContentType::None => 0,
                                    ContentType::Uri(uri) => {
                                        uris.push(uri.as_str());
                                        1
                                    }
                                    ContentType::Object(referent) => {
                                        if let Some(id) = id_to_referent.get(referent) {
                                            objects.push(*id);
                                        } else {
                                            objects.push(-1);
                                        }
                                        2
                                    }
                                    _ => return Err(invalid_value(i, rbx_value)),
                                });
                            } else {
                                return type_mismatch(i, rbx_value, "Content");
                            }
                        }
                        chunk.write_interleaved_i32_array(source_types)?;

                        chunk.write_le_u32(uris.len() as u32)?;
                        for uri in uris {
                            chunk.write_string(uri)?;
                        }
                        chunk.write_le_u32(objects.len() as u32)?;
                        chunk.write_referent_array(objects)?;

                        // If we ever need to support the external referents,
                        // we will need to add it here.
                        chunk.write_le_u32(0)?;
                    }
                }
                Ok(())
            }
        }

        Ok(())
    }

    /// Write out the hierarchical relations between instances, stored in a
    /// chunk named PRNT.
    #[profiling::function]
    pub fn serialize_parents(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing parent relationships");

        let mut chunk = ChunkBuilder::new(b"PRNT", self.serializer.compression);

        chunk.write_u8(0)?; // PRNT version 0
        chunk.write_le_u32(self.relevant_instances.len() as u32)?;

        let object_referents = self
            .relevant_instances
            .iter()
            .map(|id| self.id_to_referent[id]);

        let parent_referents = self.relevant_instances.iter().map(|id| {
            let instance = self.dom.get_by_ref(*id).unwrap();

            // If there's no parent set OR our parent is not one of the
            // instances we're serializing, we use -1 to represent a null
            // parent.
            if instance.parent().is_some() {
                self.id_to_referent
                    .get(&instance.parent())
                    .cloned()
                    .unwrap_or(-1)
            } else {
                -1
            }
        });

        chunk.write_referent_array(object_referents)?;
        chunk.write_referent_array(parent_referents)?;

        chunk.dump(&mut self.output)?;

        Ok(())
    }

    /// Write the fixed, uncompressed end chunk used to verify that the file
    /// hasn't been truncated mistakenly. This chunk is named END\0, with a zero
    /// byte at the end.
    #[profiling::function]
    pub fn serialize_end(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing file end");

        let mut end = ChunkBuilder::new(b"END\0", CompressionType::None);
        end.write_all(FILE_FOOTER)?;
        end.dump(&mut self.output)?;

        Ok(())
    }
}
fn fallback_default_value(rbx_type: VariantType) -> Option<&'static Variant> {
    use std::sync::LazyLock;
    static DEFAULT_STRING: Variant = Variant::String(String::new());
    static DEFAULT_BINARYSTRING: Variant = Variant::BinaryString(BinaryString::new());
    static DEFAULT_BOOL: Variant = Variant::Bool(false);
    static DEFAULT_INT32: Variant = Variant::Int32(0);
    static DEFAULT_FLOAT32: Variant = Variant::Float32(0.0);
    static DEFAULT_FLOAT64: Variant = Variant::Float64(0.0);
    static DEFAULT_UDIM: Variant = Variant::UDim(UDim::new(0.0, 0));
    static DEFAULT_UDIM2: Variant =
        Variant::UDim2(UDim2::new(UDim::new(0.0, 0), UDim::new(0.0, 0)));
    static DEFAULT_RAY: Variant = Variant::Ray(Ray::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
    ));
    static DEFAULT_FACES: Variant = Variant::Faces(Faces::from_bits(0).unwrap());
    static DEFAULT_AXES: Variant = Variant::Axes(Axes::from_bits(0).unwrap());
    static DEFAULT_BRICKCOLOR: Variant = Variant::BrickColor(BrickColor::MediumStoneGrey);
    static DEFAULT_CFRAME: Variant = Variant::CFrame(CFrame::identity());
    static DEFAULT_ENUM: Variant = Variant::Enum(Enum::from_u32(u32::MAX));
    static DEFAULT_COLOR3: Variant = Variant::Color3(Color3::new(0.0, 0.0, 0.0));
    static DEFAULT_VECTOR2: Variant = Variant::Vector2(Vector2::new(0.0, 0.0));
    static DEFAULT_VECTOR3: Variant = Variant::Vector3(Vector3::new(0.0, 0.0, 0.0));
    static DEFAULT_REF: Variant = Variant::Ref(Ref::none());
    static DEFAULT_VECTOR3INT16: Variant = Variant::Vector3int16(Vector3int16::new(0, 0, 0));
    static DEFAULT_NUMBERSEQUENCE: LazyLock<Variant> = LazyLock::new(|| {
        Variant::NumberSequence(NumberSequence {
            keypoints: vec![
                NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
                NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
            ],
        })
    });
    static DEFAULT_COLORSEQUENCE: LazyLock<Variant> = LazyLock::new(|| {
        Variant::ColorSequence(ColorSequence {
            keypoints: vec![
                ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
                ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
            ],
        })
    });
    static DEFAULT_NUMBERRANGE: Variant = Variant::NumberRange(NumberRange::new(0.0, 0.0));
    static DEFAULT_RECT: Variant =
        Variant::Rect(Rect::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)));
    static DEFAULT_PHYSICALPROPERTIES: Variant =
        Variant::PhysicalProperties(PhysicalProperties::Default);
    static DEFAULT_COLOR3UINT8: Variant = Variant::Color3uint8(Color3uint8::new(0, 0, 0));
    static DEFAULT_INT64: Variant = Variant::Int64(0);
    static DEFAULT_SHAREDSTRING: LazyLock<Variant> =
        LazyLock::new(|| Variant::SharedString(SharedString::new(Vec::new())));
    static DEFAULT_OPTIONALCFRAME: Variant = Variant::OptionalCFrame(None);
    static DEFAULT_TAGS: Variant = Variant::Tags(Tags::new());
    static DEFAULT_CONTENTID: Variant = Variant::ContentId(ContentId::new());
    static DEFAULT_ATTRIBUTES: Variant = Variant::Attributes(Attributes::new());
    static DEFAULT_UNIQUEID: Variant = Variant::UniqueId(UniqueId::nil());
    static DEFAULT_FONT: LazyLock<Variant> = LazyLock::new(|| Variant::Font(Font::default()));
    static DEFAULT_MATERIALCOLORS: Variant = Variant::MaterialColors(MaterialColors::new());
    static DEFAULT_SECURITYCAPABILITIES: Variant =
        Variant::SecurityCapabilities(SecurityCapabilities::from_bits(0));
    static DEFAULT_CONTENT: Variant = Variant::Content(Content::none());
    static DEFAULT_NETASSETREF: LazyLock<Variant> =
        LazyLock::new(|| Variant::NetAssetRef(NetAssetRef::new(Vec::new())));
    Some(match rbx_type {
        VariantType::String => &DEFAULT_STRING,
        VariantType::BinaryString => &DEFAULT_BINARYSTRING,
        VariantType::Bool => &DEFAULT_BOOL,
        VariantType::Int32 => &DEFAULT_INT32,
        VariantType::Float32 => &DEFAULT_FLOAT32,
        VariantType::Float64 => &DEFAULT_FLOAT64,
        VariantType::UDim => &DEFAULT_UDIM,
        VariantType::UDim2 => &DEFAULT_UDIM2,
        VariantType::Ray => &DEFAULT_RAY,
        VariantType::Faces => &DEFAULT_FACES,
        VariantType::Axes => &DEFAULT_AXES,
        VariantType::BrickColor => &DEFAULT_BRICKCOLOR,
        VariantType::CFrame => &DEFAULT_CFRAME,
        VariantType::Enum => &DEFAULT_ENUM,
        VariantType::Color3 => &DEFAULT_COLOR3,
        VariantType::Vector2 => &DEFAULT_VECTOR2,
        VariantType::Vector3 => &DEFAULT_VECTOR3,
        VariantType::Ref => &DEFAULT_REF,
        VariantType::Vector3int16 => &DEFAULT_VECTOR3INT16,
        VariantType::NumberSequence => &DEFAULT_NUMBERSEQUENCE,
        VariantType::ColorSequence => &DEFAULT_COLORSEQUENCE,
        VariantType::NumberRange => &DEFAULT_NUMBERRANGE,
        VariantType::Rect => &DEFAULT_RECT,
        VariantType::PhysicalProperties => &DEFAULT_PHYSICALPROPERTIES,
        VariantType::Color3uint8 => &DEFAULT_COLOR3UINT8,
        VariantType::Int64 => &DEFAULT_INT64,
        VariantType::SharedString => &DEFAULT_SHAREDSTRING,
        VariantType::OptionalCFrame => &DEFAULT_OPTIONALCFRAME,
        VariantType::Tags => &DEFAULT_TAGS,
        VariantType::ContentId => &DEFAULT_CONTENTID,
        VariantType::Attributes => &DEFAULT_ATTRIBUTES,
        VariantType::UniqueId => &DEFAULT_UNIQUEID,
        VariantType::Font => &DEFAULT_FONT,
        VariantType::MaterialColors => &DEFAULT_MATERIALCOLORS,
        VariantType::SecurityCapabilities => &DEFAULT_SECURITYCAPABILITIES,
        VariantType::Content => &DEFAULT_CONTENT,
        VariantType::NetAssetRef => &DEFAULT_NETASSETREF,
        _ => return None,
    })
}
