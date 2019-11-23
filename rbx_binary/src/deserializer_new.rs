use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    io::{self, Read},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree, RbxValue, RbxValueType};
use rbx_reflection::RbxPropertyTypeDescriptor;

use crate::{
    chunk::Chunk,
    core::{
        find_canonical_property_descriptor, RbxReadExt, FILE_MAGIC_HEADER, FILE_SIGNATURE,
        FILE_VERSION,
    },
    types_new::Type,
};

/// A compatibility shim to expose the new deserializer with the API of the old
/// deserializer.
pub fn decode_compat<R: Read>(tree: &mut RbxTree, parent_id: RbxId, source: R) -> io::Result<()> {
    let mut temp_tree = decode(source)?;
    let root_instance = temp_tree.get_instance(temp_tree.get_root_id()).unwrap();
    let root_children = root_instance.get_children_ids().to_vec();

    for id in root_children {
        temp_tree.move_instance(id, tree, parent_id);
    }

    Ok(())
}

pub fn decode<R: Read>(input: R) -> io::Result<RbxTree> {
    let mut deserializer = BinaryDeserializer::new(input)?;

    loop {
        let chunk = Chunk::decode(&mut deserializer.input)?;

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
    tree: RbxTree,

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
struct FileHeader {
    /// The number of instance types (represented for us as `TypeInfo`) that are
    /// in this file. Generally useful to pre-size some containers before
    /// reading the file.
    num_types: u32,

    /// The total number of instances described by this file.
    num_instances: u32,
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
    // TODO: Put class descriptor reference for this type here?
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
    properties: Vec<(String, RbxValue)>,
}

impl<R: Read> BinaryDeserializer<R> {
    fn new(mut input: R) -> io::Result<Self> {
        let tree = make_temp_output_tree();

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

    fn decode_meta_chunk(&mut self, mut chunk: &[u8]) -> io::Result<()> {
        let len = chunk.read_u32::<LittleEndian>()?;
        self.metadata.reserve(len as usize);

        for _ in 0..len {
            let key = chunk.read_string()?;
            let value = chunk.read_string()?;

            self.metadata.insert(key, value);
        }

        Ok(())
    }

    fn decode_inst_chunk(&mut self, mut chunk: &[u8]) -> io::Result<()> {
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

    fn decode_prop_chunk(&mut self, mut chunk: &[u8]) -> io::Result<()> {
        let type_id = chunk.read_u32::<LittleEndian>()?;
        let prop_name = chunk.read_string()?;
        let data_type: Type = chunk.read_u8()?.try_into().unwrap();

        // TODO: Gracefully handle error instead of panic
        let type_info = &self.type_infos[&type_id];

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
                canonical_name = descriptor.name().to_owned();
                canonical_type = match descriptor.property_type() {
                    RbxPropertyTypeDescriptor::Data(ty) => *ty,
                    RbxPropertyTypeDescriptor::Enum(_) => RbxValueType::Enum,
                    RbxPropertyTypeDescriptor::UnimplementedType(name) => {
                        log::info!("Unimplemented data type {}", name);

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
                canonical_name = prop_name;
                canonical_type = data_type.to_default_rbx_type();

                log::trace!("Unknown prop, using type {:?}", canonical_type);
            }
        }

        match data_type {
            Type::String => match canonical_type {
                RbxValueType::String => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        let rbx_value = RbxValue::String { value };
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                RbxValueType::Content => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        let rbx_value = RbxValue::String { value };
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                RbxValueType::BinaryString => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_string()?;
                        let rbx_value = RbxValue::String { value };
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                _ => panic!("type mismatch"),
            },
            Type::Bool => match canonical_type {
                RbxValueType::Bool => {
                    for referent in &type_info.referents {
                        let instance = self.instances_by_ref.get_mut(referent).unwrap();
                        let value = chunk.read_bool()?;
                        let rbx_value = RbxValue::Bool { value };
                        instance
                            .properties
                            .push((canonical_name.clone(), rbx_value));
                    }
                }
                _ => panic!("type mismatch"),
            },
            Type::Int32 => {}
            Type::Float32 => {}
            Type::Float64 => {}
            Type::UDim => {}
            Type::UDim2 => {}
            Type::Ray => {}
            // Type::Faces => {}
            // Type::Axis => {}
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

    fn decode_prnt_chunk(&mut self, mut chunk: &[u8]) -> io::Result<()> {
        let version = chunk.read_u8()?;

        if version != 0 {
            panic!("Unrecognized PRNT chunk version {}, expected 0", version);
        }

        let number_objects = chunk.read_u32::<LittleEndian>()?;

        log::trace!("PRNT chunk ({} instances)", number_objects);

        let mut subjects = vec![0; number_objects as usize];
        let mut parents = vec![0; number_objects as usize];

        chunk.read_referent_array(&mut subjects)?;
        chunk.read_referent_array(&mut parents)?;

        for (id, parent_id) in subjects.iter().copied().zip(parents.iter().copied()) {
            if parent_id == -1 {
                self.root_instance_refs.push(id);
            } else {
                let instance = self.instances_by_ref.get_mut(&parent_id).unwrap();
                instance.children.push(id);
            }
        }

        Ok(())
    }

    fn decode_end_chunk(&mut self, _chunk: &[u8]) -> io::Result<()> {
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
        let root_id = self.tree.get_root_id();
        for &referent in &self.root_instance_refs {
            instances_to_construct.push_back((referent, root_id));
        }

        while let Some((referent, parent_id)) = instances_to_construct.pop_front() {
            let id = self.construct_and_insert_instance(referent, parent_id);

            if let Some(instance) = self.instances_by_ref.get(&referent) {
                for &referent in &instance.children {
                    instances_to_construct.push_back((referent, id));
                }
            }
        }
    }

    fn construct_and_insert_instance(&mut self, referent: i32, parent_id: RbxId) -> RbxId {
        let instance = self.instances_by_ref.get_mut(&referent).unwrap();
        let type_info = &self.type_infos[&instance.type_id];

        let class_name = type_info.type_name.clone();
        let mut name = None;
        let mut properties = HashMap::new();

        for (prop_key, prop_value) in instance.properties.drain(..) {
            if prop_key.as_str() == "Name" {
                if let RbxValue::String { value } = prop_value {
                    name = Some(value);
                } else {
                    panic!("Name property was defined as a non-string type.");
                }
            } else {
                properties.insert(prop_key, prop_value);
            }
        }

        // TODO: Look up default instance name from class descriptor and then
        // fall back to ClassName if the Name property or whole class descriptor
        // is unknown.
        let name = name.unwrap_or_else(|| class_name.clone());

        let properties = RbxInstanceProperties {
            class_name,
            name,
            properties,
        };

        self.tree.insert_instance(properties, parent_id)
    }

    fn finish(self) -> RbxTree {
        self.tree
    }
}

impl FileHeader {
    fn decode<R: Read>(mut source: R) -> io::Result<Self> {
        let mut magic_header = [0; 8];
        source.read_exact(&mut magic_header)?;

        if &magic_header != FILE_MAGIC_HEADER {
            panic!("Mismatched magic header");
        }

        let mut signature = [0; 6];
        source.read_exact(&mut signature)?;

        if &signature != FILE_SIGNATURE {
            panic!("Mismatched file signature");
        }

        let version = source.read_u16::<LittleEndian>()?;

        if version != FILE_VERSION {
            panic!("Unknown file version");
        }

        let num_types = source.read_u32::<LittleEndian>()?;
        let num_instances = source.read_u32::<LittleEndian>()?;

        let mut reserved = [0; 8];
        source.read_exact(&mut reserved)?;

        if reserved != [0; 8] {
            panic!("Invalid reserved bytes");
        }

        Ok(Self {
            num_types,
            num_instances,
        })
    }
}

fn make_temp_output_tree() -> RbxTree {
    RbxTree::new(RbxInstanceProperties {
        name: "ROOT".to_owned(),
        class_name: "DataModel".to_owned(),
        properties: HashMap::new(),
    })
}
