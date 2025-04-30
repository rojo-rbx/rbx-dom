use std::{borrow::Cow, collections::VecDeque, convert::TryInto, io::Read};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rbx_dom_weak::{
    types::{
        Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
        ColorSequenceKeypoint, Content, ContentId, CustomPhysicalProperties, Enum, Faces, Font,
        FontStyle, FontWeight, MaterialColors, Matrix3, NumberRange, NumberSequence,
        NumberSequenceKeypoint, PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities,
        SharedString, SmoothGrid, Tags, UDim, UDim2, UniqueId, Variant, VariantType, Vector2,
        Vector3, Vector3int16,
    },
    InstanceBuilder, Ustr, WeakDom,
};
use rbx_reflection::{DataType, PropertyKind, PropertySerialization, ReflectionDatabase};

use crate::{
    chunk::Chunk,
    core::{find_property_descriptors, RbxReadExt},
    types::Type,
};

use super::{error::InnerError, header::FileHeader, Deserializer};

pub(super) struct DeserializerState<'db, R> {
    /// The user-provided configuration that we should use.
    deserializer: &'db Deserializer<'db>,

    /// The input data encoded as a binary model.
    input: R,

    /// The tree that instances should be written into. Eventually returned to
    /// the user.
    tree: WeakDom,

    /// The metadata contained in the file, which affects how some constructs
    /// are interpreted by Roblox.
    metadata: HashMap<String, String>,

    /// The SharedStrings contained in the file, if any, in the order that they
    /// appear in the file.
    shared_strings: Vec<SharedString>,

    /// All of the instance types described by the file so far.
    type_infos: HashMap<u32, TypeInfo>,

    /// All of the instances known by the deserializer.
    instances_by_ref: HashMap<i32, Instance>,

    /// Referents for all of the instances with no parent, in order they appear
    /// in the file.
    root_instance_refs: Vec<i32>,

    /// Contains a set of unknown type IDs that we've encountered so far while
    /// deserializing this file. We use this map in order to ensure we only
    /// print one warning per unknown type ID when deserializing a file.
    unknown_type_ids: HashSet<u8>,
}

/// Represents a unique instance class. Binary models define all their instance
/// types up front and give them a short u32 identifier.
struct TypeInfo {
    /// The ID given to this type by the current file we're deserializing. This
    /// ID can be different for different files.
    type_id: u32,

    /// The common name for this type like `Folder` or `UserInputService`.
    type_name: Ustr,

    /// A list of the instances described by this file that are this type.
    referents: Vec<i32>,
}

/// Contains all the information we need to gather in order to construct an
/// instance. Incrementally built up by the deserializer as we decode different
/// chunks.
struct Instance {
    /// A work-in-progress builder that will be used to construct this instance.
    builder: InstanceBuilder,

    /// Document-defined IDs for the children of this instance.
    children: Vec<i32>,
}

/// Properties may be serialized under different names or types than
/// they ultimately should have in the DOM. CanonicalProperty
/// represents the "proper" name and type of a property, and possibly
/// contains a migration for some properties Roblox has replaced with
/// others (like Font, which has been superceded by FontFace).
#[derive(Debug)]
struct CanonicalProperty<'db> {
    name: Ustr,
    ty: VariantType,
    migration: Option<&'db PropertySerialization<'db>>,
}

fn find_canonical_property<'de>(
    database: &'de ReflectionDatabase,
    binary_type: Type,
    class_name: Ustr,
    prop_name: Ustr,
) -> Option<CanonicalProperty<'de>> {
    match find_property_descriptors(database, class_name, prop_name) {
        Some(descriptors) => {
            // If this descriptor is known but wasn't supposed to be
            // serialized, we should skip it.
            //
            // On 2021-09-07 (v494), BasePart.MaterialVariant was added as a
            // serializable Referent property. It was removed soon after, on
            // 2021-10-12 (v499). Any models saved during that period have
            // BasePart.MaterialVariant present.
            //
            // On 2022-03-08 (v517), BasePart.MaterialVariant was
            // reintroduced as a string property that does not serialize. It
            // serializes as MaterialVariantSerialized.
            //
            // In case we run into a model serialized during that period, or
            // this happens again, we need to make sure that the name we
            // found is the one that's supposed to serialize.
            if let PropertyKind::Canonical { serialization } = &descriptors.canonical.kind {
                if matches!(serialization, PropertySerialization::DoesNotSerialize) {
                    log::debug!(
                        "Skipping property {} as it is canonical and should not serialize.",
                        descriptors.canonical.name
                    );
                    return None;
                }
            }

            // TODO: Do we need an additional fix here?
            let canonical_name = &descriptors.canonical.name;
            let canonical_type = match &descriptors.canonical.data_type {
                DataType::Value(ty) => *ty,
                DataType::Enum(_) => VariantType::Enum,
                _ => {
                    // TODO: Configurable handling of unknown types?
                    return None;
                }
            };
            let migration = match &descriptors.canonical.kind {
                PropertyKind::Canonical {
                    serialization: migration @ PropertySerialization::Migrate(_),
                } => Some(migration),
                _ => None,
            };

            log::trace!(
                "Known prop, canonical name {} and type {:?}, with {:?} migration",
                canonical_name,
                canonical_type,
                migration,
            );

            Some(CanonicalProperty {
                name: canonical_name.as_ref().into(),
                ty: canonical_type,
                migration,
            })
        }
        None => {
            let canonical_type = match binary_type.to_default_rbx_type() {
                Some(rbx_type) => rbx_type,
                None => {
                    log::warn!("Unsupported prop type {:?}, skipping property", binary_type);
                    return None;
                }
            };

            log::trace!("Unknown prop, using type {:?}", canonical_type);

            Some(CanonicalProperty {
                name: prop_name,
                ty: canonical_type,
                migration: None,
            })
        }
    }
}

fn add_property(instance: &mut Instance, canonical_property: &CanonicalProperty, value: Variant) {
    if let Some(PropertySerialization::Migrate(migration)) = canonical_property.migration {
        let new_property_name = &migration.new_property_name;
        let old_property_name = canonical_property.name;

        if !instance.builder.has_property(new_property_name) {
            log::trace!(
                "Attempting to migrate property {old_property_name} to {new_property_name}"
            );
            match migration.perform(&value) {
                Ok(new_value) => {
                    instance.builder.add_property(new_property_name, new_value);
                    log::trace!(
                        "Successfully migrated property {old_property_name} to {new_property_name}"
                    );
                }
                Err(e) => {
                    log::warn!("Failed to migrate property {old_property_name} to {new_property_name} because: {e}");
                }
            };
        }
    } else {
        instance
            .builder
            .add_property(canonical_property.name, value)
    }
}

impl<'db, R: Read> DeserializerState<'db, R> {
    pub(super) fn new(
        deserializer: &'db Deserializer<'db>,
        mut input: R,
    ) -> Result<Self, InnerError> {
        let mut tree = WeakDom::new(InstanceBuilder::new("DataModel"));

        let header = FileHeader::decode(&mut input)?;

        let type_infos = HashMap::with_capacity(header.num_types as usize);
        let instances_by_ref = HashMap::with_capacity(1 + header.num_instances as usize);

        tree.reserve(header.num_instances as usize);

        Ok(DeserializerState {
            deserializer,
            input,
            tree,
            metadata: HashMap::new(),
            shared_strings: Vec::new(),
            type_infos,
            instances_by_ref,
            root_instance_refs: Vec::new(),
            unknown_type_ids: HashSet::new(),
        })
    }

    pub(super) fn next_chunk(&mut self) -> Result<Chunk, InnerError> {
        Ok(Chunk::decode(&mut self.input)?)
    }

    #[profiling::function]
    pub(super) fn decode_meta_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let len = chunk.read_le_u32()?;
        self.metadata.reserve(len as usize);

        for _ in 0..len {
            let key = chunk.read_string()?;
            let value = chunk.read_string()?;

            self.metadata.insert(key, value);
        }

        Ok(())
    }

    #[profiling::function]
    pub(super) fn decode_sstr_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let version = chunk.read_le_u32()?;

        if version != 0 {
            return Err(InnerError::UnknownChunkVersion {
                chunk_name: "SSTR",
                version,
            });
        }

        let num_entries = chunk.read_le_u32()?;

        for _ in 0..num_entries {
            chunk.read_exact(&mut [0; 16])?; // We don't do anything with the hash.
            let data = chunk.read_binary_string()?;
            self.shared_strings.push(SharedString::new(data));
        }

        Ok(())
    }

    #[profiling::function]
    pub(super) fn decode_inst_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let type_id = chunk.read_le_u32()?;
        let type_name = chunk.read_string()?;
        let object_format = chunk.read_u8()?;
        let number_instances = chunk.read_le_u32()?;

        log::trace!(
            "INST chunk (type ID {}, type name {}, format {}, {} instances)",
            type_id,
            type_name,
            object_format,
            number_instances,
        );

        let mut referents = vec![0; number_instances as usize];
        chunk.read_referent_array(&mut referents)?;

        let prop_capacity = self
            .deserializer
            .database
            .classes
            .get(type_name.as_str())
            .map(|class| class.default_properties.len())
            .unwrap_or(0);

        // TODO: Check object_format and check for service markers if it's 1?

        for &referent in &referents {
            self.instances_by_ref.insert(
                referent,
                Instance {
                    builder: InstanceBuilder::with_property_capacity(
                        type_name.as_str(),
                        prop_capacity,
                    ),
                    children: Vec::new(),
                },
            );
        }

        self.type_infos.insert(
            type_id,
            TypeInfo {
                type_id,
                type_name: type_name.into(),
                referents,
            },
        );

        Ok(())
    }

    #[profiling::function]
    pub(super) fn decode_prop_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let type_id = chunk.read_le_u32()?;
        let prop_name = chunk.read_string()?;

        let type_info = self
            .type_infos
            .get(&type_id)
            .ok_or(InnerError::InvalidTypeId { type_id })?;

        // PROP chunks that contain no type byte are ignored by Roblox. This can
        // happen when a new type is introduced.
        //
        // On 2021-04-08, OptionalCoordinateFrame was introduced, but its
        // serialized format was just a type ID followed by the prop name. This
        // leads us to believe that Roblox will silently ignore any PROP chunks
        // that end immediately after the prop name, so we do the same.
        let binary_type_byte = match chunk.read_u8() {
            Ok(byte) => byte,
            Err(_) => return Ok(()),
        };

        let binary_type: Type = match binary_type_byte.try_into() {
            Ok(ty) => ty,
            Err(_) => {
                if self.unknown_type_ids.insert(binary_type_byte) {
                    log::warn!(
                        "Unknown value type ID {byte:#04x} ({byte}) in Roblox \
                         binary model file. Found in property {class}.{prop}.",
                        byte = binary_type_byte,
                        class = type_info.type_name,
                        prop = prop_name,
                    );
                }

                return Ok(());
            }
        };

        log::trace!(
            "PROP chunk ({}.{}, instance type {}, prop type {}",
            type_info.type_name,
            prop_name,
            type_info.type_id,
            type_id
        );

        // The `Name` prop is special and is routed to a different spot for
        // rbx_dom_weak, so we handle it specially here.
        if prop_name == "Name" {
            // TODO: If an instance is never assigned a name through this code
            // path, we should use the reflection database to figure out its
            // default name. This should be rare: effectively never!

            for referent in &type_info.referents {
                let instance = self.instances_by_ref.get_mut(referent).unwrap();
                let binary_string = chunk.read_binary_string()?;
                let value = match std::str::from_utf8(&binary_string) {
                    Ok(value) => Cow::Borrowed(value),
                    Err(_) => {
                        log::warn!(
                            "Performing lossy string conversion on property {}.{} because it did not contain UTF-8.
This may cause unexpected or broken behavior in your final results if you rely on this property being non UTF-8.",
                            type_info.type_name,
                            prop_name
                        );

                        String::from_utf8_lossy(binary_string.as_ref())
                    }
                };
                instance.builder.set_name(value);
            }

            return Ok(());
        }

        let property = if let Some(property) = find_canonical_property(
            self.deserializer.database,
            binary_type,
            type_info.type_name,
            prop_name.as_str().into(),
        ) {
            property
        } else {
            return Ok(());
        };

        let canonical_type = property.ty;

        match binary_type {
            Type::String => match canonical_type {
                VariantType::String => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let binary_string = chunk.read_binary_string()?;
                        let value = match std::str::from_utf8(&binary_string) {
                            Ok(value) => Cow::Borrowed(value),
                            Err(_) => {
                                log::warn!(
                            "Performing lossy string conversion on property {}.{} because it did not contain UTF-8.
This may cause unexpected or broken behavior in your final results if you rely on this property being non UTF-8.",
                                    type_info.type_name,
                                    property.name
                                );

                                String::from_utf8_lossy(&binary_string)
                            }
                        };

                        add_property(instance, &property, value.as_ref().into());
                    }
                }
                VariantType::ContentId => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        add_property(instance, &property, ContentId::from(value).into());
                    }
                }
                VariantType::BinaryString => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value: BinaryString = chunk.read_binary_string()?.into();
                        add_property(instance, &property, value.into());
                    }
                }
                VariantType::Tags => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let buffer = chunk.read_binary_string()?;

                        let value = Tags::decode(buffer.as_ref()).map_err(|_| {
                            InnerError::InvalidPropData {
                                type_name: type_info.type_name.to_string(),
                                prop_name: prop_name.clone(),
                                valid_value: "a list of valid null-delimited UTF-8 strings",
                                actual_value: "invalid UTF-8".to_string(),
                            }
                        })?;

                        add_property(instance, &property, value.into());
                    }
                }
                VariantType::Attributes => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let buffer = chunk.read_binary_string()?;

                        match Attributes::from_reader(buffer.as_slice()) {
                            Ok(value) => {
                                add_property(instance, &property, value.into());
                            }
                            Err(err) => {
                                log::warn!(
                                    "Failed to parse Attributes on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                                    type_info.type_name,
                                    err
                                );

                                add_property(
                                    instance,
                                    &property,
                                    BinaryString::from(buffer).into(),
                                );
                            }
                        }
                    }
                }
                VariantType::MaterialColors => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let buffer = chunk.read_binary_string()?;
                        match MaterialColors::decode(&buffer) {
                            Ok(value) => add_property(instance, &property, value.into()),
                            Err(err) => {
                                log::warn!(
                                    "Failed to parse MaterialColors on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                                    type_info.type_name,
                                    err
                                );

                                add_property(
                                    instance,
                                    &property,
                                    BinaryString::from(buffer).into(),
                                );
                            }
                        }
                    }
                }
                VariantType::SmoothGrid => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let buffer = chunk.read_binary_string()?;
                        match SmoothGrid::decode(&buffer) {
                            Ok(value) => add_property(instance, &property, value.into()),
                            Err(err) => {
                                log::warn!(
                                    "Failed to parse SmoothGrid on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                                    type_info.type_name,
                                    err
                                );

                                add_property(
                                    instance,
                                    &property,
                                    BinaryString::from(buffer).into(),
                                );
                            }
                        }
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names:
                            "String, ContentId, Content, Tags, Attributes, or BinaryString",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Bool => match canonical_type {
                VariantType::Bool => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_bool()?;
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Bool",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Int32 => match canonical_type {
                VariantType::Int32 => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_i32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                // This branch allows values serialized as Int32 to be converted to Int64 when we expect a Int64
                // Basically, we convert Int32 to Int64 when we expect a Int64 but read a Int32
                // See: #301
                VariantType::Int64 => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_i32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value_converted = i64::from(value);
                        add_property(instance, &property, value_converted.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Int32",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Float32 => match canonical_type {
                VariantType::Float32 => {
                    let mut values = vec![0.0; type_info.referents.len()];
                    chunk.read_interleaved_f32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Float32",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Float64 => match canonical_type {
                VariantType::Float64 => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_le_f64()?;
                        add_property(instance, &property, value.into());
                    }
                }
                // This branch allows values serialized as Float32 to be converted to Float64 when we expect a Float64
                // Basically, we convert Float32 to Float64 when we expect a Float64 but read a Float32
                // See: #301
                VariantType::Float32 => {
                    let mut values = vec![0.0; type_info.referents.len()];
                    chunk.read_interleaved_f32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let converted_value = f64::from(value);
                        add_property(instance, &property, converted_value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Float64",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::UDim => match canonical_type {
                VariantType::UDim => {
                    let mut scales = vec![0.0; type_info.referents.len()];
                    let mut offsets = vec![0; type_info.referents.len()];

                    chunk.read_interleaved_f32_array(&mut scales)?;
                    chunk.read_interleaved_i32_array(&mut offsets)?;

                    let values = scales
                        .into_iter()
                        .zip(offsets)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    for (value, referent) in values.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "UDim",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::UDim2 => match canonical_type {
                VariantType::UDim2 => {
                    let prop_count = type_info.referents.len();
                    let mut scale_x = vec![0.0; prop_count];
                    let mut scale_y = vec![0.0; prop_count];
                    let mut offset_x = vec![0; prop_count];
                    let mut offset_y = vec![0; prop_count];

                    chunk.read_interleaved_f32_array(&mut scale_x)?;
                    chunk.read_interleaved_f32_array(&mut scale_y)?;
                    chunk.read_interleaved_i32_array(&mut offset_x)?;
                    chunk.read_interleaved_i32_array(&mut offset_y)?;

                    let x = scale_x
                        .into_iter()
                        .zip(offset_x)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    let y = scale_y
                        .into_iter()
                        .zip(offset_y)
                        .map(|(scale, offset)| UDim::new(scale, offset));

                    let values = x.zip(y).map(|(x, y)| UDim2::new(x, y));

                    for (value, referent) in values.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "UDim2",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Ray => match canonical_type {
                VariantType::Ray => {
                    for referent in &type_info.referents {
                        let origin_x = chunk.read_le_f32()?;
                        let origin_y = chunk.read_le_f32()?;
                        let origin_z = chunk.read_le_f32()?;
                        let direction_x = chunk.read_le_f32()?;
                        let direction_y = chunk.read_le_f32()?;
                        let direction_z = chunk.read_le_f32()?;

                        let instance = self.instances_by_ref.get_mut(referent).unwrap();

                        add_property(
                            instance,
                            &property,
                            Ray::new(
                                Vector3::new(origin_x, origin_y, origin_z),
                                Vector3::new(direction_x, direction_y, direction_z),
                            )
                            .into(),
                        );
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Ray",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Faces => match canonical_type {
                VariantType::Faces => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_u8()?;
                        let faces =
                            Faces::from_bits(value).ok_or_else(|| InnerError::InvalidPropData {
                                type_name: type_info.type_name.to_string(),
                                prop_name: prop_name.clone(),
                                valid_value: "less than 63",
                                actual_value: value.to_string(),
                            })?;

                        add_property(instance, &property, faces.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Faces",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Axes => match canonical_type {
                VariantType::Axes => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_u8()?;

                        let axes =
                            Axes::from_bits(value).ok_or_else(|| InnerError::InvalidPropData {
                                type_name: type_info.type_name.to_string(),
                                prop_name: prop_name.clone(),
                                valid_value: "less than 7",
                                actual_value: value.to_string(),
                            })?;

                        add_property(instance, &property, axes.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Axes",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::BrickColor => match canonical_type {
                VariantType::BrickColor => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_u32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let color = value
                            .try_into()
                            .ok()
                            .and_then(BrickColor::from_number)
                            .ok_or_else(|| InnerError::InvalidPropData {
                                type_name: type_info.type_name.to_string(),
                                prop_name: prop_name.clone(),
                                valid_value: "a valid BrickColor",
                                actual_value: value.to_string(),
                            })?;

                        add_property(instance, &property, color.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "BrickColor",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Color3 => match canonical_type {
                VariantType::Color3 => {
                    let mut r = vec![0.0; type_info.referents.len()];
                    let mut g = vec![0.0; type_info.referents.len()];
                    let mut b = vec![0.0; type_info.referents.len()];

                    chunk.read_interleaved_f32_array(&mut r)?;
                    chunk.read_interleaved_f32_array(&mut g)?;
                    chunk.read_interleaved_f32_array(&mut b)?;

                    let colors = r
                        .into_iter()
                        .zip(g)
                        .zip(b)
                        .map(|((r, g), b)| Color3::new(r, g, b));

                    for (color, referent) in colors.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, color.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Color3",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Vector2 => match canonical_type {
                VariantType::Vector2 => {
                    let mut x = vec![0.0; type_info.referents.len()];
                    let mut y = vec![0.0; type_info.referents.len()];

                    chunk.read_interleaved_f32_array(&mut x)?;
                    chunk.read_interleaved_f32_array(&mut y)?;

                    let values = x.into_iter().zip(y).map(|(x, y)| Vector2::new(x, y));

                    for (value, referent) in values.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Vector2",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Vector3 => match canonical_type {
                VariantType::Vector3 => {
                    let mut x = vec![0.0; type_info.referents.len()];
                    let mut y = vec![0.0; type_info.referents.len()];
                    let mut z = vec![0.0; type_info.referents.len()];

                    chunk.read_interleaved_f32_array(&mut x)?;
                    chunk.read_interleaved_f32_array(&mut y)?;
                    chunk.read_interleaved_f32_array(&mut z)?;

                    let values = x
                        .into_iter()
                        .zip(y)
                        .zip(z)
                        .map(|((x, y), z)| Vector3::new(x, y, z));

                    for (value, referent) in values.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Vector3",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::CFrame => match canonical_type {
                VariantType::CFrame => {
                    let referents = &type_info.referents;
                    let mut rotations = Vec::with_capacity(referents.len());

                    for _ in 0..referents.len() {
                        let id = chunk.read_u8()?;
                        if id == 0 {
                            rotations.push(Matrix3::new(
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));
                        } else if let Ok(basic_rotation) = Matrix3::from_basic_rotation_id(id) {
                            rotations.push(basic_rotation);
                        } else {
                            return Err(InnerError::BadRotationId {
                                type_name: type_info.type_name.to_string(),
                                prop_name,
                                id,
                            });
                        }
                    }

                    let mut x = vec![0.0; referents.len()];
                    let mut y = vec![0.0; referents.len()];
                    let mut z = vec![0.0; referents.len()];

                    chunk.read_interleaved_f32_array(&mut x)?;
                    chunk.read_interleaved_f32_array(&mut y)?;
                    chunk.read_interleaved_f32_array(&mut z)?;

                    let values = x
                        .into_iter()
                        .zip(y)
                        .zip(z)
                        .map(|((x, y), z)| Vector3::new(x, y, z))
                        .zip(rotations)
                        .map(|(position, rotation)| CFrame::new(position, rotation));

                    for (cframe, referent) in values.zip(referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, cframe.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "CFrame",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Enum => match canonical_type {
                VariantType::Enum => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_u32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, Enum::from_u32(value).into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Enum",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Ref => match canonical_type {
                VariantType::Ref => {
                    let mut refs = vec![0; type_info.referents.len()];
                    chunk.read_referent_array(&mut refs)?;

                    for (value, referent) in refs.into_iter().zip(&type_info.referents) {
                        let rbx_value = if let Some(instance) = self.instances_by_ref.get(&value) {
                            instance.builder.referent()
                        } else {
                            Ref::none()
                        };

                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, rbx_value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Ref",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Vector3int16 => match canonical_type {
                VariantType::Vector3int16 => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(
                            instance,
                            &property,
                            Vector3int16::new(
                                chunk.read_le_i16()?,
                                chunk.read_le_i16()?,
                                chunk.read_le_i16()?,
                            )
                            .into(),
                        )
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Vector3int16",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Font => match canonical_type {
                VariantType::Font => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();

                        let family = chunk.read_string()?;
                        let weight = FontWeight::from_u16(chunk.read_le_u16()?).unwrap_or_default();
                        let style = FontStyle::from_u8(chunk.read_u8()?).unwrap_or_default();
                        let cached_face_id = chunk.read_string()?;

                        let cached_face_id = if cached_face_id.is_empty() {
                            None
                        } else {
                            Some(cached_face_id)
                        };

                        add_property(
                            instance,
                            &property,
                            Font {
                                family,
                                weight,
                                style,
                                cached_face_id,
                            }
                            .into(),
                        );
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Font",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::NumberSequence => match canonical_type {
                VariantType::NumberSequence => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let keypoint_count = chunk.read_le_u32()?;
                        let mut keypoints = Vec::with_capacity(keypoint_count as usize);

                        for _ in 0..keypoint_count {
                            keypoints.push(NumberSequenceKeypoint::new(
                                chunk.read_le_f32()?,
                                chunk.read_le_f32()?,
                                chunk.read_le_f32()?,
                            ))
                        }

                        add_property(instance, &property, NumberSequence { keypoints }.into())
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "NumberSequence",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::ColorSequence => match canonical_type {
                VariantType::ColorSequence => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let keypoint_count = chunk.read_le_u32()? as usize;
                        let mut keypoints = Vec::with_capacity(keypoint_count);

                        for _ in 0..keypoint_count {
                            keypoints.push(ColorSequenceKeypoint::new(
                                chunk.read_le_f32()?,
                                Color3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));

                            // envelope is serialized but doesn't do anything; don't do anything with it
                            chunk.read_le_f32()?;
                        }

                        add_property(instance, &property, ColorSequence { keypoints }.into())
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "ColorSequence",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::NumberRange => match canonical_type {
                VariantType::NumberRange => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(
                            instance,
                            &property,
                            NumberRange::new(chunk.read_le_f32()?, chunk.read_le_f32()?).into(),
                        )
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "NumberRange",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Rect => match canonical_type {
                VariantType::Rect => {
                    let len = type_info.referents.len();
                    let mut x_min = vec![0.0; len];
                    let mut y_min = vec![0.0; len];
                    let mut x_max = vec![0.0; len];
                    let mut y_max = vec![0.0; len];

                    chunk.read_interleaved_f32_array(&mut x_min)?;
                    chunk.read_interleaved_f32_array(&mut y_min)?;
                    chunk.read_interleaved_f32_array(&mut x_max)?;
                    chunk.read_interleaved_f32_array(&mut y_max)?;

                    let values = x_min.into_iter().zip(y_min).zip(x_max).zip(y_max).map(
                        |(((x_min, y_min), x_max), y_max)| {
                            Rect::new(Vector2::new(x_min, y_min), Vector2::new(x_max, y_max))
                        },
                    );

                    for (value, referent) in values.zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into())
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Rect",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::PhysicalProperties => match canonical_type {
                VariantType::PhysicalProperties => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = if chunk.read_u8()? == 1 {
                            Variant::PhysicalProperties(PhysicalProperties::Custom(
                                CustomPhysicalProperties {
                                    density: chunk.read_le_f32()?,
                                    friction: chunk.read_le_f32()?,
                                    elasticity: chunk.read_le_f32()?,
                                    friction_weight: chunk.read_le_f32()?,
                                    elasticity_weight: chunk.read_le_f32()?,
                                },
                            ))
                        } else {
                            Variant::PhysicalProperties(PhysicalProperties::Default)
                        };

                        add_property(instance, &property, value);
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "PhysicalProperties",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Color3uint8 => match canonical_type {
                VariantType::Color3 => {
                    let len = type_info.referents.len();
                    let mut r = vec![0; len];
                    let mut g = vec![0; len];
                    let mut b = vec![0; len];

                    chunk.read_exact(r.as_mut_slice())?;
                    chunk.read_exact(g.as_mut_slice())?;
                    chunk.read_exact(b.as_mut_slice())?;

                    let colors = r
                        .into_iter()
                        .zip(g)
                        .zip(b)
                        .map(|((r, g), b)| Color3uint8::new(r, g, b));

                    for (color, referent) in colors.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, color.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Color3",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Int64 => match canonical_type {
                VariantType::Int64 => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_i64_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Int64",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::SharedString => match canonical_type {
                VariantType::SharedString => {
                    let mut values = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_u32_array(&mut values)?;

                    for (value, referent) in values.into_iter().zip(&type_info.referents) {
                        let shared_string =
                            self.shared_strings.get(value as usize).ok_or_else(|| {
                                InnerError::InvalidPropData {
                                    type_name: type_info.type_name.to_string(),
                                    prop_name: prop_name.clone(),
                                    valid_value: "a valid SharedString",
                                    actual_value: format!("{:?}", value),
                                }
                            })?;

                        let instance = self.instances_by_ref.get_mut(referent).unwrap();

                        add_property(instance, &property, shared_string.clone().into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "SharedString",
                        actual_type_name: format!("{:?}", invalid_type),
                    })
                }
            },
            Type::OptionalCFrame => match canonical_type {
                VariantType::OptionalCFrame => {
                    let referents = &type_info.referents;
                    let mut rotations = Vec::with_capacity(referents.len());

                    // Roblox writes a type marker for CFrame here that we don't
                    // need to use. We explicitly check for this right now just
                    // in case we're wrong and we do need it!
                    let actual_type_id = chunk.read_u8()?;
                    if actual_type_id != Type::CFrame as u8 {
                        return Err(InnerError::BadOptionalCFrameFormat {
                            expected_type_name: String::from("CFrame"),
                            expected_type_id: Type::CFrame as u8,
                            actual_type_id,
                        });
                    }

                    for _ in 0..referents.len() {
                        let id = chunk.read_u8()?;
                        if id == 0 {
                            rotations.push(Matrix3::new(
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                                Vector3::new(
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                    chunk.read_le_f32()?,
                                ),
                            ));
                        } else if let Ok(basic_rotation) = Matrix3::from_basic_rotation_id(id) {
                            rotations.push(basic_rotation);
                        } else {
                            return Err(InnerError::BadRotationId {
                                type_name: type_info.type_name.to_string(),
                                prop_name,
                                id,
                            });
                        }
                    }

                    let mut x = vec![0.0; referents.len()];
                    let mut y = vec![0.0; referents.len()];
                    let mut z = vec![0.0; referents.len()];

                    chunk.read_interleaved_f32_array(&mut x)?;
                    chunk.read_interleaved_f32_array(&mut y)?;
                    chunk.read_interleaved_f32_array(&mut z)?;

                    // Roblox writes a type marker for Bool here that we don't
                    // need to use. We explicitly check for this right now just
                    // in case we're wrong and we do need it!
                    let actual_type_id = chunk.read_u8()?;
                    if actual_type_id != Type::Bool as u8 {
                        return Err(InnerError::BadOptionalCFrameFormat {
                            expected_type_name: String::from("Bool"),
                            expected_type_id: Type::Bool as u8,
                            actual_type_id,
                        });
                    }

                    let values = x
                        .into_iter()
                        .zip(y)
                        .zip(z)
                        .map(|((x, y), z)| Vector3::new(x, y, z))
                        .zip(rotations)
                        .map(|(position, rotation)| {
                            if chunk.read_u8().ok()? == 0 {
                                None
                            } else {
                                Some(CFrame::new(position, rotation))
                            }
                        });

                    for (cframe, referent) in values.zip(referents) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, cframe.into());
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "OptionalCFrame",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::UniqueId => match canonical_type {
                VariantType::UniqueId => {
                    let n = type_info.referents.len();
                    let mut values = vec![[0; 16]; n];
                    chunk.read_interleaved_bytes::<16>(&mut values)?;

                    for (i, referent) in type_info.referents.iter().enumerate() {
                        let mut value = values[i].as_slice();
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(
                            instance,
                            &property,
                            UniqueId::new(
                                value.read_be_u32()?,
                                value.read_be_u32()?,
                                value.read_be_i64()?.rotate_right(1),
                            )
                            .into(),
                        )
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "UniqueId",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::SecurityCapabilities => match canonical_type {
                VariantType::SecurityCapabilities => {
                    let mut values = vec![0; type_info.referents.len()];

                    chunk.read_interleaved_i64_array(values.as_mut_slice())?;

                    let values: Vec<SecurityCapabilities> = values
                        .into_iter()
                        .map(|value| SecurityCapabilities::from_bits(value as u64))
                        .collect();

                    for (referent, value) in type_info.referents.iter().zip(values) {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into())
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "SecurityCapabilities",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
            Type::Content => match canonical_type {
                VariantType::Content => {
                    let mut source_types = vec![0; type_info.referents.len()];
                    chunk.read_interleaved_i32_array(&mut source_types)?;

                    let uri_count = chunk.read_le_u32()? as usize;
                    let mut uris = VecDeque::with_capacity(uri_count);
                    for _ in 0..uri_count {
                        uris.push_front(chunk.read_string()?);
                    }

                    let object_count = chunk.read_le_u32()? as usize;
                    let mut objects: VecDeque<i32> = vec![0; object_count].into();
                    chunk.read_referent_array(objects.make_contiguous())?;

                    let external_count = chunk.read_le_u32().unwrap() as usize;
                    // We are advised by Roblox to just ignore this, as it's
                    // meant for internal use. If we want to use it in the
                    // future, it's a referent array.
                    let mut bytes = vec![0; external_count * 4];
                    chunk.read_to_end(&mut bytes)?;

                    for (referent, ty) in type_info.referents.iter().zip(source_types) {
                        let value = match ty {
                            0 => Content::none(),
                            1 => Content::from_uri(uris.pop_back().unwrap()),
                            2 => {
                                let read_value = objects.pop_back().unwrap();
                                Content::from_referent(
                                    if let Some(instance) = self.instances_by_ref.get(&read_value) {
                                        instance.builder.referent()
                                    } else {
                                        Ref::none()
                                    },
                                )
                            }
                            n => return Err(InnerError::BadContentType(n)),
                        };
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        add_property(instance, &property, value.into())
                    }
                }
                invalid_type => {
                    return Err(InnerError::PropTypeMismatch {
                        type_name: type_info.type_name.to_string(),
                        prop_name,
                        valid_type_names: "Content",
                        actual_type_name: format!("{:?}", invalid_type),
                    });
                }
            },
        }

        Ok(())
    }

    #[profiling::function]
    pub(super) fn decode_prnt_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let version = chunk.read_u8()?;

        if version != 0 {
            return Err(InnerError::UnknownChunkVersion {
                chunk_name: "PRNT",
                version: version as u32,
            });
        }

        let number_objects = chunk.read_le_u32()?;

        log::trace!("PRNT chunk ({} instances)", number_objects);

        let mut subjects = vec![0; number_objects as usize];
        let mut parents = vec![0; number_objects as usize];

        chunk.read_referent_array(&mut subjects)?;
        chunk.read_referent_array(&mut parents)?;

        for (id, parent_ref) in subjects.iter().copied().zip(parents.iter().copied()) {
            if parent_ref == -1 {
                self.root_instance_refs.push(id);
            } else {
                let instance = self.instances_by_ref.get_mut(&parent_ref).unwrap();
                instance.children.push(id);
            }
        }

        Ok(())
    }

    #[profiling::function]
    pub(super) fn decode_end_chunk(&mut self, _chunk: &[u8]) -> Result<(), InnerError> {
        log::trace!("END chunk");

        // We don't do any validation on the END chunk. There's no useful
        // information for us here as it just signals that the file hasn't been
        // truncated.

        Ok(())
    }

    /// Combines together all the decoded information to build and emplace
    /// instances in our tree.
    #[profiling::function]
    pub(super) fn finish(mut self) -> WeakDom {
        log::trace!("Constructing tree from deserialized data");

        // Track all the instances we need to construct. Order of construction
        // is important to preserve for both determinism and sometimes
        // functionality of models we handle.
        let mut instances_to_construct = VecDeque::new();

        // Any instance with a parent of -1 will be at the top level of the
        // tree. Because of the way rbx_dom_weak generally works, we need to
        // start at the top of the tree to begin construction.
        let root_ref = self.tree.root_ref();
        for &referent in &self.root_instance_refs {
            instances_to_construct.push_back((referent, root_ref));
        }

        while let Some((referent, parent_ref)) = instances_to_construct.pop_front() {
            let instance = self.instances_by_ref.remove(&referent).unwrap();
            let id = self.tree.insert(parent_ref, instance.builder);

            for referent in instance.children {
                instances_to_construct.push_back((referent, id));
            }
        }

        self.tree
    }
}
