use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
    rc::Rc,
    str,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree, RbxValue};

use crate::{
    chunk::Chunk,
    core::{RbxReadExt, FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION},
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

    /// For each known referent in the file, tracks which referents are
    /// considered its children.
    ///
    /// This map is used to construct the final tree from top to bottom. The
    /// entries for the entry `-1`, the null referent, are treated as the
    /// top-level instances in the file.
    children_from_ref: HashMap<i32, Vec<i32>>,

    instances: HashMap<i32, Instance>,
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

struct Instance {
    /// The file-defined referent for this instance.
    referent: i32,

    /// The type of this instance, given as a type ID defined in the file.
    type_id: u32,

    /// Referents for the children of this instance.
    children: Vec<i32>,

    /// The properties found for this instance so far from the PROP chunk. Using
    /// a Vec preserves order in the unlikely event of a collision and is also
    /// compact storage since we don't need to look up properties by key.
    properties: Vec<(Rc<String>, RbxValue)>,
}

impl<R: Read> BinaryDeserializer<R> {
    fn new(mut input: R) -> io::Result<Self> {
        let tree = make_temp_output_tree();

        let header = FileHeader::decode(&mut input)?;

        let type_infos = HashMap::with_capacity(header.num_types as usize);
        let children_from_ref = HashMap::with_capacity(header.num_instances as usize);
        let instances = HashMap::with_capacity(header.num_instances as usize);

        Ok(BinaryDeserializer {
            input,
            tree,
            metadata: HashMap::new(),
            type_infos,
            children_from_ref,
            instances,
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
        let data_type = chunk.read_u8()?;

        // TODO: Gracefully handle error instead of panic
        let type_info = &self.type_infos[&type_id];

        log::trace!(
            "PROP chunk ({}.{}, (instance type {}) prop type {:x?}",
            type_info.type_name,
            prop_name,
            type_info.type_id,
            type_id
        );

        match data_type {
            0x01 => { /* String, ProtectedString, Content, BinaryString, SharedString */ }
            0x02 => { /* Bool */ }
            0x03 => { /* i32 */ }
            0x04 => { /* f32 */ }
            0x05 => { /* f64 */ }
            0x06 => { /* UDim */ }
            0x07 => { /* UDim2 */ }
            0x08 => { /* Ray */ }
            0x09 => { /* Faces */ }
            0x0A => { /* Axis */ }
            0x0B => { /* BrickColor */ }
            0x0C => { /* Color3 */ }
            0x0D => { /* Vector2 */ }
            0x0E => { /* Vector3 */ }
            0x10 => { /* CFrame */ }
            0x12 => { /* Enum */ }
            0x13 => { /* Referent */ }
            0x14 => { /* Vector3int16 */ }
            0x15 => { /* NumberSequence */ }
            0x16 => { /* ColorSequence */ }
            0x17 => { /* NumberRange */ }
            0x18 => { /* Rect2D */ }
            0x19 => { /* PhysicalProperties */ }
            0x1A => { /* Color3uint8 */ }
            0x1B => { /* Int64 */ }
            _ => {
                log::info!(
                    "Unknown prop type {:x?} on property named {}",
                    data_type,
                    prop_name
                );
            }
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
            self.children_from_ref
                .entry(id)
                .or_default()
                .push(parent_id);
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
        // Track all the instances we need to construct. Order of construction
        // is important to preserve for both determinism and sometimes
        // functionality of models we handle.
        let mut instances_to_construct = VecDeque::new();

        // Any instance with a parent of -1 will be at the top level of the
        // tree. Because of the way rbx_dom_weak generally works, we need to
        // start at the top to begin construction.
        if let Some(root_referents) = self.children_from_ref.get(&-1) {
            let root_id = self.tree.get_root_id();

            for &referent in root_referents {
                instances_to_construct.push_back((referent, root_id));
            }
        } else {
            log::info!("File defined no root referents. It might be malformed.");
        }

        while let Some((referent, parent_id)) = instances_to_construct.pop_front() {
            let id = self.construct_and_insert_instance(referent, parent_id);

            if let Some(children) = self.children_from_ref.get(&referent) {
                for &referent in children {
                    instances_to_construct.push_back((referent, id));
                }
            }
        }
    }

    fn construct_and_insert_instance(&mut self, referent: i32, parent_id: RbxId) -> RbxId {
        let properties = RbxInstanceProperties {
            name: "TEMP".to_string(),
            class_name: "TEMP".to_string(),
            properties: HashMap::new(),
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
