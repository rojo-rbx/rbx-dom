use std::{
    borrow::{Borrow, Cow},
    collections::{BTreeMap, HashMap, VecDeque},
    convert::TryInto,
    io::{self, Write},
    u32,
};

use byteorder::{LittleEndian, WriteBytesExt};
use rbx_dom_weak::{
    types::{BinaryString, Color3, Ref, UDim, UDim2, Variant, VariantType, Vector2},
    WeakDom,
};
use rbx_reflection::{ClassDescriptor, ClassTag, DataType};
use thiserror::Error;

use crate::{
    chunk::{ChunkBuilder, ChunkCompression},
    core::{
        find_serialized_property_descriptor, RbxWriteExt, FILE_MAGIC_HEADER, FILE_SIGNATURE,
        FILE_VERSION,
    },
    types::Type,
};

static FILE_FOOTER: &[u8] = b"</roblox>";

/// Represents an error that occurred during serialization.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self {
            source: Box::new(inner),
        }
    }
}

#[derive(Debug, Error)]
enum InnerError {
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },

    #[error(
        "Property type mismatch: Expected {type_name}.{prop_name} to be of type {valid_type_names}, \
        but it was of type {actual_type_name} on instance {instance_full_name}",
    )]
    PropTypeMismatch {
        type_name: String,
        prop_name: String,
        valid_type_names: &'static str,
        actual_type_name: String,
        instance_full_name: String,
    },

    #[error("Unsupported property type: {type_name}.{prop_name} is of type {prop_type}")]
    UnsupportedPropType {
        type_name: String,
        prop_name: String,
        prop_type: String,
    },

    #[error("The instance with referent {referent:?} was not present in the dom.")]
    InvalidInstanceId { referent: Ref },
}

/// Serializes instances from an `WeakDom` into a writer in Roblox's binary
/// model format.
pub fn encode<W: Write>(dom: &WeakDom, refs: &[Ref], writer: W) -> Result<(), Error> {
    let mut serializer = BinarySerializer::new(dom, writer);

    serializer.add_instances(refs)?;

    log::debug!("Type info discovered: {:#?}", serializer.type_infos);

    serializer.generate_referents();

    log::trace!("Referents constructed: {:#?}", serializer.id_to_referent);

    serializer.write_header()?;
    serializer.serialize_metadata()?;
    serializer.serialize_instances()?;
    serializer.serialize_properties()?;
    serializer.serialize_parents()?;
    serializer.serialize_end()?;

    Ok(())
}

/// Represents all of the state during a single serialization session. A new
/// `BinarySerializer` object should be created every time we want to serialize
/// a binary model file.
struct BinarySerializer<'a, W> {
    /// The dom containing all of the instances that we're serializing.
    dom: &'a WeakDom,

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
    ///
    /// These are stored sorted so that we naturally iterate over them in order
    /// and improve our chances of being deterministic.
    type_infos: BTreeMap<String, TypeInfo>,

    /// The next type ID that should be assigned if a type is discovered and
    /// added to the serializer.
    next_type_id: u32,
}

/// An instance class that our serializer knows about. We should have one struct
/// per unique ClassName.
#[derive(Debug)]
struct TypeInfo {
    /// The ID that this serializer will use to refer to this type of instance.
    type_id: u32,

    /// Whether this type is considered a service. Only one copy of a given
    /// service can exist for a given ServiceProvider. DataModel is the only
    /// ServiceProvider in most projects.
    is_service: bool,

    /// The IDs of all of the instances of this type.
    object_refs: Vec<Ref>,

    /// All of the defined properties for this type found on any instance of
    /// this type.
    ///
    /// Stored in a sorted map to try to ensure that we write out properties in
    /// a deterministic order.
    properties: BTreeMap<String, PropInfo>,

    /// A reference to the type's class descriptor from rbx_reflection, if this
    /// is a known class.
    class_descriptor: Option<&'static ClassDescriptor<'static>>,
}

#[derive(Debug)]
struct PropInfo {
    prop_type: Type,
    default_value: Cow<'static, Variant>,
}

impl<'a, W: Write> BinarySerializer<'a, W> {
    fn new(dom: &'a WeakDom, output: W) -> Self {
        BinarySerializer {
            dom,
            output,
            relevant_instances: Vec::new(),
            id_to_referent: HashMap::new(),
            type_infos: BTreeMap::new(),
            next_type_id: 0,
        }
    }

    /// Mark the given instance IDs and all of their descendants as intended for
    /// serialization with this serializer.
    fn add_instances(&mut self, referents: &[Ref]) -> Result<(), InnerError> {
        let mut to_visit = VecDeque::new();
        to_visit.extend(referents);

        while let Some(referent) = to_visit.pop_front() {
            self.relevant_instances.push(referent);
            self.collect_type_info(referent)?;

            // TODO: Turn into error
            let instance = self.dom.get_by_ref(referent).unwrap();
            to_visit.extend(instance.children());
        }

        Ok(())
    }

    /// Collect information about all the different types of instance and their
    /// properties.
    fn collect_type_info(&mut self, referent: Ref) -> Result<(), InnerError> {
        let instance = self
            .dom
            .get_by_ref(referent)
            .ok_or_else(|| InnerError::InvalidInstanceId { referent })?;

        let type_info = self.get_or_create_type_info(&instance.class);
        type_info.object_refs.push(referent);

        for (prop_name, prop_value) in &instance.properties {
            let ser_name;
            let ser_rbx_type;

            match find_serialized_property_descriptor(&instance.class, prop_name) {
                Some(descriptor) => {
                    ser_name = descriptor.name.as_ref();
                    ser_rbx_type = match &descriptor.data_type {
                        DataType::Value(ty) => *ty,
                        DataType::Enum(_) => VariantType::EnumValue,
                        _ => {
                            // rbx_binary is not new enough to handle this kind
                            // of property, whatever it is.
                            panic!("Unsupported property type");
                        }
                    };
                }
                None => {
                    ser_name = prop_name.as_str();
                    ser_rbx_type = prop_value.ty();
                }
            }

            if !type_info.properties.contains_key(ser_name) {
                let default_value = type_info
                    .class_descriptor
                    .and_then(|class| {
                        class
                            .default_properties
                            .get(prop_name.as_str())
                            .map(Cow::Borrowed)
                    })
                    .or_else(|| Self::fallback_default_value(ser_rbx_type).map(Cow::Owned))
                    .ok_or_else(|| {
                        // Since we don't know how to generate the default value for
                        // this property, we consider it unsupported.
                        InnerError::UnsupportedPropType {
                            type_name: instance.class.clone(),
                            prop_name: prop_name.clone(),
                            prop_type: format!("{:?}", ser_rbx_type),
                        }
                    })?;

                let ser_type = Type::from_rbx_type(ser_rbx_type).ok_or_else(|| {
                    // This is a known value type, but rbx_binary doesn't have a
                    // binary type value for it. rbx_binary might be out of
                    // date?
                    InnerError::UnsupportedPropType {
                        type_name: instance.class.clone(),
                        prop_name: prop_name.clone(),
                        prop_type: format!("{:?}", ser_rbx_type),
                    }
                })?;

                type_info.properties.insert(
                    ser_name.to_owned(),
                    PropInfo {
                        prop_type: ser_type,
                        default_value,
                    },
                );
            }
        }

        Ok(())
    }

    /// Finds the type info from the given class name if it exists, or creates
    /// one and returns a reference to it if not.
    fn get_or_create_type_info(&mut self, class: &str) -> &mut TypeInfo {
        if !self.type_infos.contains_key(class) {
            let type_id = self.next_type_id;
            self.next_type_id += 1;

            let class_descriptor = rbx_reflection_database::get().classes.get(class);

            let is_service;
            if let Some(descriptor) = &class_descriptor {
                is_service = descriptor.tags.contains(&ClassTag::Service);
            } else {
                log::info!("The class {} is not known to rbx_binary", class);
                is_service = false;
            };

            let mut properties = BTreeMap::new();

            // Every instance has a property named Name. Even though
            // rbx_dom_weak encodes the name property specially, we still insert
            // this property into the type info and handle it like a regular
            // property during encoding.
            //
            // We can use a dummy default_value here because instances from
            // rbx_dom_weak always have a name set.
            properties.insert(
                "Name".to_owned(),
                PropInfo {
                    prop_type: Type::String,
                    default_value: Cow::Owned(Variant::String(String::new())),
                },
            );

            self.type_infos.insert(
                class.to_owned(),
                TypeInfo {
                    type_id,
                    is_service,
                    object_refs: Vec::new(),
                    properties,
                    class_descriptor,
                },
            );
        }

        // This unwrap will not panic because we always insert this key into
        // type_infos in this function.
        self.type_infos.get_mut(class).unwrap()
    }

    /// Populate the map from rbx-dom's instance ID space to the IDs that we'll
    /// be serializing to the model.
    fn generate_referents(&mut self) {
        self.id_to_referent.reserve(self.relevant_instances.len());

        for (next_referent, id) in self.relevant_instances.iter().enumerate() {
            self.id_to_referent.insert(*id, next_referent.try_into().unwrap());
        }
    }

    fn write_header(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing header");

        self.output.write_all(FILE_MAGIC_HEADER)?;
        self.output.write_all(FILE_SIGNATURE)?;
        self.output.write_u16::<LittleEndian>(FILE_VERSION)?;

        self.output
            .write_u32::<LittleEndian>(self.type_infos.len() as u32)?;
        self.output
            .write_u32::<LittleEndian>(self.relevant_instances.len() as u32)?;
        self.output.write_u64::<LittleEndian>(0)?;

        Ok(())
    }

    /// Write out any metadata about this file, stored in a chunk named META.
    fn serialize_metadata(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing metadata (currently no-op)");
        // TODO: There is no concept of metadata in a dom yet.
        Ok(())
    }

    /// Write out the declarations of all instances, stored in a series of
    /// chunks named INST.
    fn serialize_instances(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing instance chunks");

        for (type_name, type_info) in &self.type_infos {
            log::trace!(
                "Writing chunk for {} ({} instances)",
                type_name,
                type_info.object_refs.len()
            );

            let mut chunk = ChunkBuilder::new(b"INST", ChunkCompression::Compressed);

            chunk.write_u32::<LittleEndian>(type_info.type_id)?;
            chunk.write_string(type_name)?;

            // It's possible that this integer will be expanded in the future to
            // be a general version/format field instead of just service vs
            // non-service.
            //
            // At that point, we'll start thinking about it like it's a u8
            // instead of a bool.
            chunk.write_bool(type_info.is_service)?;

            chunk.write_u32::<LittleEndian>(type_info.object_refs.len() as u32)?;

            chunk.write_referents(
                type_info
                    .object_refs
                    .iter()
                    .map(|id| self.id_to_referent[id]),
            )?;

            if type_info.is_service {
                // It's unclear what this byte is used for, but when the type is
                // a service (like Workspace, Lighting, etc), we need to write
                // the value `1` for every instance in our file of that type.
                //
                // In 99.9% of cases, there's only going to be one copy of a
                // given service, so we're not worried about doing this super
                // efficiently.
                for _ in 0..type_info.object_refs.len() {
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
    fn serialize_properties(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing properties");

        for (type_name, type_info) in &self.type_infos {
            for (prop_name, prop_info) in &type_info.properties {
                log::trace!(
                    "Writing property {}.{} (type {:?})",
                    type_name,
                    prop_name,
                    prop_info.prop_type
                );

                let mut chunk = ChunkBuilder::new(b"PROP", ChunkCompression::Compressed);

                chunk.write_u32::<LittleEndian>(type_info.type_id)?;
                chunk.write_string(&prop_name)?;
                chunk.write_u8(prop_info.prop_type as u8)?;

                let dom = &self.dom;
                let values = type_info
                    .object_refs
                    .iter()
                    .map(|id| {
                        // This unwrap will not panic because we uphold the
                        // invariant that any ID in object_refs must be part of
                        // this dom.
                        let instance = dom.get_by_ref(*id).unwrap();

                        // We store the Name property in a different field for
                        // convenience, but when serializing to the binary model
                        // format we need to handle it just like other properties.
                        if prop_name == "Name" {
                            Cow::Owned(Variant::String(instance.name.clone()))
                        } else {
                            instance
                                .properties
                                .get(prop_name)
                                .map(Cow::Borrowed)
                                .unwrap_or_else(|| Cow::Borrowed(prop_info.default_value.borrow()))
                        }
                    })
                    .enumerate();

                // Helper to generate a type mismatch error with context from
                // this chunk.
                let type_mismatch =
                    |i: usize, bad_value: &Variant, valid_type_names: &'static str| {
                        Err(InnerError::PropTypeMismatch {
                            type_name: type_name.clone(),
                            prop_name: prop_name.clone(),
                            valid_type_names,
                            actual_type_name: format!("{:?}", bad_value.ty()),
                            instance_full_name: self.full_name_for(type_info.object_refs[i]),
                        })
                    };

                match prop_info.prop_type {
                    Type::String => {
                        for (i, rbx_value) in values {
                            match rbx_value.as_ref() {
                                Variant::String(value) => {
                                    chunk.write_string(&value)?;
                                }
                                Variant::Content(value) => {
                                    chunk.write_string(value.as_ref())?;
                                }
                                Variant::BinaryString(value) => {
                                    chunk.write_binary_string(value.as_ref())?;
                                }
                                _ => {
                                    return type_mismatch(
                                        i,
                                        &rbx_value,
                                        "String, Content, or BinaryString",
                                    );
                                }
                            }
                        }
                    }
                    Type::Bool => {
                        for (i, rbx_value) in values {
                            match rbx_value.as_ref() {
                                Variant::Bool(value) => {
                                    chunk.write_bool(*value)?;
                                }
                                _ => {
                                    return type_mismatch(i, &rbx_value, "Bool");
                                }
                            }
                        }
                    }
                    Type::Int32 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Int32(value) = rbx_value.as_ref() {
                                buf.push(*value);
                            } else {
                                return type_mismatch(i, &rbx_value, "Int32");
                            }
                        }

                        chunk.write_interleaved_i32_array(buf.into_iter())?;
                    }
                    Type::Float32 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Float32(value) = rbx_value.as_ref() {
                                buf.push(*value);
                            } else {
                                return type_mismatch(i, &rbx_value, "Float32");
                            }
                        }

                        chunk.write_interleaved_f32_array(buf.into_iter())?;
                    }
                    Type::Float64 => {
                        for (i, rbx_value) in values {
                            if let Variant::Float64(value) = rbx_value.as_ref() {
                                chunk.write_f64::<LittleEndian>(*value)?;
                            } else {
                                return type_mismatch(i, &rbx_value, "Float64");
                            }
                        }
                    }
                    Type::UDim => {
                        let mut scale = Vec::with_capacity(values.len());
                        let mut offset = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::UDim(value) = rbx_value.as_ref() {
                                scale.push(value.scale);
                                offset.push(value.offset);
                            } else {
                                return type_mismatch(i, &rbx_value, "UDim");
                            }
                        }

                        chunk.write_interleaved_f32_array(scale.into_iter())?;
                        chunk.write_interleaved_i32_array(offset.into_iter())?;
                    }
                    Type::UDim2 => {
                        let mut scale_x = Vec::with_capacity(values.len());
                        let mut scale_y = Vec::with_capacity(values.len());
                        let mut offset_x = Vec::with_capacity(values.len());
                        let mut offset_y = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::UDim2(value) = rbx_value.as_ref() {
                                scale_x.push(value.x.scale);
                                scale_y.push(value.y.scale);
                                offset_x.push(value.x.offset);
                                offset_y.push(value.y.offset);
                            } else {
                                return type_mismatch(i, &rbx_value, "UDim2");
                            }
                        }

                        chunk.write_interleaved_f32_array(scale_x.into_iter())?;
                        chunk.write_interleaved_f32_array(scale_y.into_iter())?;
                        chunk.write_interleaved_i32_array(offset_x.into_iter())?;
                        chunk.write_interleaved_i32_array(offset_y.into_iter())?;
                    }
                    Type::Color3 => {
                        let mut r = Vec::with_capacity(values.len());
                        let mut g = Vec::with_capacity(values.len());
                        let mut b = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Color3(value) = rbx_value.as_ref() {
                                r.push(value.r);
                                g.push(value.g);
                                b.push(value.b);
                            } else {
                                return type_mismatch(i, &rbx_value, "Color3");
                            }
                        }

                        chunk.write_interleaved_f32_array(r.into_iter())?;
                        chunk.write_interleaved_f32_array(g.into_iter())?;
                        chunk.write_interleaved_f32_array(b.into_iter())?;
                    }
                    Type::Vector2 => {
                        let mut x = Vec::with_capacity(values.len());
                        let mut y = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Vector2(value) = rbx_value.as_ref() {
                                x.push(value.x);
                                y.push(value.y)
                            } else {
                                return type_mismatch(i, &rbx_value, "Vector2");
                            }
                        }

                        chunk.write_interleaved_f32_array(x.into_iter())?;
                        chunk.write_interleaved_f32_array(y.into_iter())?;
                    }
                    Type::Int64 => {
                        let mut buf = Vec::with_capacity(values.len());

                        for (i, rbx_value) in values {
                            if let Variant::Int64(value) = rbx_value.as_ref() {
                                buf.push(*value);
                            } else {
                                return type_mismatch(i, &rbx_value, "Int64");
                            }
                        }

                        chunk.write_interleaved_i64_array(buf.into_iter())?;
                    }
                    _ => {
                        return Err(InnerError::UnsupportedPropType {
                            type_name: type_name.clone(),
                            prop_name: prop_name.clone(),
                            prop_type: format!("binary type {:?}", prop_info.prop_type),
                        });
                    }
                }

                chunk.dump(&mut self.output)?;
            }
        }

        Ok(())
    }

    /// Write out the hierarchical relations between instances, stored in a
    /// chunk named PRNT.
    fn serialize_parents(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing parent relationships");

        let mut chunk = ChunkBuilder::new(b"PRNT", ChunkCompression::Compressed);

        chunk.write_u8(0)?; // PRNT version 0
        chunk.write_u32::<LittleEndian>(self.relevant_instances.len() as u32)?;

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

        chunk.write_referents(object_referents)?;
        chunk.write_referents(parent_referents)?;

        chunk.dump(&mut self.output)?;

        Ok(())
    }

    /// Write the fixed, uncompressed end chunk used to verify that the file
    /// hasn't been truncated mistakenly. This chunk is named END\0, with a zero
    /// byte at the end.
    fn serialize_end(&mut self) -> Result<(), InnerError> {
        log::trace!("Writing file end");

        let mut end = ChunkBuilder::new(b"END\0", ChunkCompression::Uncompressed);
        end.write_all(FILE_FOOTER)?;
        end.dump(&mut self.output)?;

        Ok(())
    }

    /// Equivalent to Instance:GetFullName() from Roblox.
    fn full_name_for(&self, subject_ref: Ref) -> String {
        let mut components = Vec::new();
        let mut current_id = subject_ref;

        while current_id.is_some() {
            let instance = self.dom.get_by_ref(current_id).unwrap();
            components.push(instance.name.as_str());
            current_id = instance.parent();
        }

        let mut name = String::new();
        for component in components.iter().rev() {
            name.push_str(component);
            name.push('.');
        }
        name.pop();

        name
    }

    fn fallback_default_value(rbx_type: VariantType) -> Option<Variant> {
        Some(match rbx_type {
            VariantType::String => Variant::String(String::new()),
            VariantType::BinaryString => Variant::BinaryString(BinaryString::new()),
            VariantType::Bool => Variant::Bool(false),
            VariantType::Int32 => Variant::Int32(0),
            VariantType::Float32 => Variant::Float32(0.0),
            VariantType::Float64 => Variant::Float64(0.0),
            VariantType::UDim => Variant::UDim(UDim::new(0.0, 0)),
            VariantType::UDim2 => Variant::UDim2(UDim2::new(UDim::new(0.0, 0), UDim::new(0.0, 0))),
            VariantType::Color3 => Variant::Color3(Color3::new(0.0, 0.0, 0.0)),
            VariantType::Vector2 => Variant::Vector2(Vector2::new(0.0, 0.0)),
            VariantType::Int64 => Variant::Int64(0),
            _ => return None,
        })
    }
}
