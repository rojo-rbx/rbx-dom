use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    io::{self, Read},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rbx_dom_weak::{
    types::{Ref, Variant, VariantType},
    InstanceBuilder, WeakDom,
};
use rbx_reflection::DataType;
use snafu::{ResultExt, Snafu};

use crate::{
    chunk::Chunk,
    core::{
        find_canonical_property_descriptor, RbxReadExt, FILE_MAGIC_HEADER, FILE_SIGNATURE,
        FILE_VERSION,
    },
    types::Type,
};

/// Represents an error that occurred during deserialization.
#[derive(Debug, Snafu)]
pub struct Error(Box<InnerError>);

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self(Box::new(inner))
    }
}

#[derive(Debug, Snafu)]
pub(crate) enum InnerError {
    /// A general I/O error occurred.
    #[snafu(display("{}", source))]
    Io { source: io::Error },

    #[snafu(display("Invalid file header"))]
    BadHeader,

    #[snafu(display("Unknown file version {}. Known versions are: 0", version))]
    UnknownFileVersion { version: u16 },

    #[snafu(display("Unknown version {} for chunk {}", version, chunk_name))]
    UnknownChunkVersion {
        chunk_name: &'static str,
        version: u8,
    },

    #[snafu(display(
        "Type mismatch: Property {}.{} should be {}, but it was {}",
        type_name,
        prop_name,
        valid_type_names,
        actual_type_name
    ))]
    PropTypeMismatch {
        type_name: String,
        prop_name: String,
        valid_type_names: &'static str,
        actual_type_name: String,
    },

    #[snafu(display("File referred to type ID {}, which was not declared", type_id))]
    InvalidTypeId { type_id: u32 },
}

impl From<io::Error> for InnerError {
    fn from(source: io::Error) -> Self {
        InnerError::Io { source }
    }
}

/// Deserializes instances from a reader containing Roblox's binary model
/// format.
pub(crate) fn decode<R: Read>(reader: R) -> Result<WeakDom, Error> {
    let mut deserializer = BinaryDeserializer::new(reader)?;

    loop {
        let chunk = Chunk::decode(&mut deserializer.input).context(Io)?;

        match &chunk.name {
            b"META" => deserializer.decode_meta_chunk(&chunk.data)?,
            b"INST" => deserializer.decode_inst_chunk(&chunk.data)?,
            b"PROP" => deserializer.decode_prop_chunk(&chunk.data)?,
            b"PRNT" => deserializer.decode_prnt_chunk(&chunk.data)?,
            b"END\0" => {
                deserializer.decode_end_chunk(&chunk.data)?;
                break;
            }
            _ => match str::from_utf8(&chunk.name) {
                Ok(name) => log::info!("Unknown binary chunk name {}", name),
                Err(_) => log::info!("Unknown binary chunk name {:?}", chunk.name),
            },
        }
    }

    deserializer.construct_tree();

    Ok(deserializer.finish())
}

struct BinaryDeserializer<R> {
    /// The input data encoded as a binary model.
    input: R,

    /// The tree that instances should be written into. Eventually returned to
    /// the user.
    tree: WeakDom,

    /// The metadata contained in the file, which affects how some constructs
    /// are interpreted by Roblox.
    metadata: HashMap<String, String>,

    /// All of the instance types described by the file so far.
    type_infos: HashMap<u32, TypeInfo>,

    /// All of the instances known by the deserializer.
    instances_by_ref: HashMap<i32, Instance>,

    /// Referents for all of the instances with no parent, in order they appear
    /// in the file.
    root_instance_refs: Vec<i32>,
}

/// All the information contained in the header before any chunks are read from
/// the file.
pub(crate) struct FileHeader {
    /// The number of instance types (represented for us as `TypeInfo`) that are
    /// in this file. Generally useful to pre-size some containers before
    /// reading the file.
    pub(crate) num_types: u32,

    /// The total number of instances described by this file.
    pub(crate) num_instances: u32,
}

/// Represents a unique instance class. Binary models define all their instance
/// types up front and give them a short u32 identifier.
struct TypeInfo {
    /// The ID given to this type by the current file we're deserializing. This
    /// ID can be different for different files.
    type_id: u32,

    /// The common name for this type like `Folder` or `UserInputService`.
    type_name: String,

    /// A list of the instances described by this file that are this type.
    referents: Vec<i32>,
}

/// Contains all the information we need to gather in order to construct an
/// instance. Incrementally built up by the deserializer as we decode different
/// chunks.
struct Instance {
    /// The type of this instance, given as a type ID defined in the file.
    type_id: u32,

    /// Referents for the children of this instance.
    children: Vec<i32>,

    /// The properties found for this instance so far from the PROP chunk. Using
    /// a Vec preserves order in the unlikely event of a collision and is also
    /// compact storage since we don't need to look up properties by key.
    properties: Vec<(String, Variant)>,
}

impl<R: Read> BinaryDeserializer<R> {
    fn new(mut input: R) -> Result<Self, InnerError> {
        let tree = WeakDom::new(InstanceBuilder::new("DataModel"));

        let header = FileHeader::decode(&mut input)?;

        let type_infos = HashMap::with_capacity(header.num_types as usize);
        let instances_by_ref = HashMap::with_capacity(1 + header.num_instances as usize);

        Ok(BinaryDeserializer {
            input,
            tree,
            metadata: HashMap::new(),
            type_infos,
            instances_by_ref,
            root_instance_refs: Vec::new(),
        })
    }

    fn decode_meta_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let len = chunk.read_u32::<LittleEndian>()?;
        self.metadata.reserve(len as usize);

        for _ in 0..len {
            let key = chunk.read_string()?;
            let value = chunk.read_string()?;

            self.metadata.insert(key, value);
        }

        Ok(())
    }

    fn decode_inst_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let type_id = chunk.read_u32::<LittleEndian>()?;
        let type_name = chunk.read_string()?;
        let object_format = chunk.read_u8()?;
        let number_instances = chunk.read_u32::<LittleEndian>()?;

        log::trace!(
            "INST chunk (type ID {}, type name {}, format {}, {} instances)",
            type_id,
            type_name,
            object_format,
            number_instances,
        );

        let mut referents = vec![0; number_instances as usize];
        chunk.read_referent_array(&mut referents)?;

        // TODO: Check object_format and check for service markers if it's 1?

        for &referent in &referents {
            self.instances_by_ref.insert(
                referent,
                Instance {
                    type_id,
                    children: Vec::new(),
                    properties: Vec::new(),
                },
            );
        }

        self.type_infos.insert(
            type_id,
            TypeInfo {
                type_id,
                type_name,
                referents,
            },
        );

        Ok(())
    }

    fn decode_prop_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let type_id = chunk.read_u32::<LittleEndian>()?;
        let prop_name = chunk.read_string()?;
        let binary_type: Type = chunk.read_u8()?.try_into().unwrap();

        let type_info = self
            .type_infos
            .get(&type_id)
            .ok_or(InnerError::InvalidTypeId { type_id })?;

        log::trace!(
            "PROP chunk ({}.{}, instance type {}, prop type {}",
            type_info.type_name,
            prop_name,
            type_info.type_id,
            type_id
        );

        let canonical_name;
        let canonical_type;

        match find_canonical_property_descriptor(&type_info.type_name, &prop_name) {
            Some(descriptor) => {
                canonical_name = descriptor.name.clone().into_owned();
                canonical_type = match &descriptor.data_type {
                    DataType::Value(ty) => *ty,
                    DataType::Enum(_) => VariantType::EnumValue,
                    _ => {
                        // TODO: Configurable handling of unknown types?
                        return Ok(());
                    }
                };

                log::trace!(
                    "Known prop, canonical name {} and type {:?}",
                    canonical_name,
                    canonical_type
                );
            }
            None => {
                canonical_name = prop_name.clone();

                match binary_type.to_default_rbx_type() {
                    Some(rbx_type) => canonical_type = rbx_type,
                    None => {
                        log::warn!("Unsupported prop type {:?}, skipping property", binary_type);

                        return Ok(());
                    }
                }

                log::trace!("Unknown prop, using type {:?}", canonical_type);
            }
        }

        match binary_type {
            Type::String => match canonical_type {
                VariantType::String => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        let rbx_value = Variant::String(value);
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                VariantType::Content => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        let rbx_value = Variant::String(value);
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                VariantType::BinaryString => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_binary_string()?;
                        let rbx_value = Variant::BinaryString(value.into());
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                invalid_type => {
                    PropTypeMismatch {
                        type_name: type_info.type_name.clone(),
                        prop_name,
                        valid_type_names: "String, Content, or BinaryString",
                        actual_type_name: format!("{:?}", invalid_type),
                    }
                    .fail()?;
                }
            },
            Type::Bool => match canonical_type {
                VariantType::Bool => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_bool()?;
                        let rbx_value = Variant::Bool(value);
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                invalid_type => {
                    PropTypeMismatch {
                        type_name: type_info.type_name.clone(),
                        prop_name,
                        valid_type_names: "Bool",
                        actual_type_name: format!("{:?}", invalid_type),
                    }
                    .fail()?;
                }
            },
            Type::Int32 => {}
            Type::Float32 => {}
            Type::Float64 => {}
            Type::UDim => {}
            Type::UDim2 => {}
            Type::Ray => {}
            Type::Faces => {}
            Type::Axis => {}
            Type::BrickColor => {}
            Type::Color3 => {}
            Type::Vector2 => {}
            Type::Vector3 => {}
            Type::CFrame => {}
            Type::Enum => {}
            Type::Ref => {}
            Type::Vector3int16 => {}
            Type::NumberSequence => {}
            Type::ColorSequence => {}
            Type::NumberRange => {}
            Type::Rect => {}
            Type::PhysicalProperties => {}
            Type::Color3uint8 => {}
            Type::Int64 => {}
        }

        Ok(())
    }

    fn decode_prnt_chunk(&mut self, mut chunk: &[u8]) -> Result<(), InnerError> {
        let version = chunk.read_u8()?;

        if version != 0 {
            return Err(InnerError::UnknownChunkVersion {
                chunk_name: "PRNT",
                version,
            });
        }

        let number_objects = chunk.read_u32::<LittleEndian>()?;

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

    fn decode_end_chunk(&mut self, _chunk: &[u8]) -> Result<(), InnerError> {
        log::trace!("END chunk");

        // We don't do any validation on the END chunk. There's no useful
        // information for us here as it just signals that the file hasn't been
        // truncated.

        Ok(())
    }

    /// Combines together all the decoded information to build and emplace
    /// instances in our tree.
    fn construct_tree(&mut self) {
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
            let id = self.construct_and_insert_instance(referent, parent_ref);

            if let Some(instance) = self.instances_by_ref.get(&referent) {
                for &referent in &instance.children {
                    instances_to_construct.push_back((referent, id));
                }
            }
        }
    }

    fn construct_and_insert_instance(&mut self, referent: i32, parent_ref: Ref) -> Ref {
        let instance = self.instances_by_ref.get_mut(&referent).unwrap();
        let type_info = &self.type_infos[&instance.type_id];

        let mut builder = InstanceBuilder::new(&type_info.type_name);

        for (prop_key, prop_value) in instance.properties.drain(..) {
            if prop_key.as_str() == "Name" {
                if let Variant::String(value) = prop_value {
                    builder = builder.with_name(value);
                } else {
                    panic!("Name property was defined as a non-string type.");
                }
            } else {
                builder = builder.with_property(prop_key, prop_value);
            }
        }

        // TODO: Look up default instance name from class descriptor and then
        // fall back to ClassName if the Name property or whole class descriptor
        // is unknown. This isn't super important since binary files with
        // instances that have no Name generally don't exist.

        self.tree.insert(parent_ref, builder)
    }

    fn finish(self) -> WeakDom {
        self.tree
    }
}

impl FileHeader {
    pub(crate) fn decode<R: Read>(mut source: R) -> Result<Self, InnerError> {
        let mut magic_header = [0; 8];
        source.read_exact(&mut magic_header)?;

        if &magic_header != FILE_MAGIC_HEADER {
            return Err(InnerError::BadHeader);
        }

        let mut signature = [0; 6];
        source.read_exact(&mut signature)?;

        if &signature != FILE_SIGNATURE {
            return Err(InnerError::BadHeader);
        }

        let version = source.read_u16::<LittleEndian>()?;

        if version != FILE_VERSION {
            return Err(InnerError::UnknownFileVersion { version });
        }

        let num_types = source.read_u32::<LittleEndian>()?;
        let num_instances = source.read_u32::<LittleEndian>()?;

        let mut reserved = [0; 8];
        source.read_exact(&mut reserved)?;

        if reserved != [0; 8] {
            return Err(InnerError::BadHeader);
        }

        Ok(Self {
            num_types,
            num_instances,
        })
    }
}
