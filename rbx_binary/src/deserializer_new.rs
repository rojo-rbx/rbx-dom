use std::{
    collections::HashMap,
    io::{self, Read},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt};
use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree};

use crate::{
    chunk::Chunk,
    core::{FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION},
};

/// A compatibility shim to expose the new deserializer with the API of the old
/// deserializer.
pub fn decode_compat<R: Read>(
    tree: &mut RbxTree,
    parent_id: RbxId,
    mut source: R,
) -> io::Result<()> {
    let mut temp_tree = decode(source)?;
    let root_instance = temp_tree.get_instance(temp_tree.get_root_id()).unwrap();
    let root_children = root_instance.get_children_ids().to_vec();

    for id in root_children {
        temp_tree.move_instance(id, tree, parent_id);
    }

    Ok(())
}

pub fn decode<R: Read>(mut input: R) -> io::Result<RbxTree> {
    let mut deserializer = BinaryDeserializer::new(input)?;

    loop {
        let chunk = Chunk::decode(&mut deserializer.input)?;

        match &chunk.name {
            b"META" => deserializer.decode_meta_chunk(chunk)?,
            b"INST" => deserializer.decode_inst_chunk(chunk)?,
            b"PROP" => deserializer.decode_prop_chunk(chunk)?,
            b"PRNT" => deserializer.decode_prnt_chunk(chunk)?,
            b"END\0" => break,
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

    /// The data about the file contained in the header.
    header: FileHeader,

    /// Currently unused. The metadata contained in the file, which affects how
    /// some constructs are interpreted by Roblox.
    metadata: HashMap<String, String>,
}

struct FileHeader {
    num_instance_types: u32,
    num_instances: u32,
}

impl<R: Read> BinaryDeserializer<R> {
    fn new(mut input: R) -> io::Result<Self> {
        let tree = make_temp_output_tree();

        let header = FileHeader::decode(&mut input)?;

        Ok(BinaryDeserializer {
            input,
            tree,
            header,
            metadata: HashMap::new(),
        })
    }

    fn decode_meta_chunk(&mut self, chunk: Chunk) -> io::Result<()> {
        Ok(())
    }

    fn decode_inst_chunk(&mut self, chunk: Chunk) -> io::Result<()> {
        Ok(())
    }

    fn decode_prop_chunk(&mut self, chunk: Chunk) -> io::Result<()> {
        Ok(())
    }

    fn decode_prnt_chunk(&mut self, chunk: Chunk) -> io::Result<()> {
        Ok(())
    }

    fn decode_end_chunk(&mut self, chunk: Chunk) -> io::Result<()> {
        Ok(())
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

        let num_instance_types = source.read_u32::<LittleEndian>()?;
        let num_instances = source.read_u32::<LittleEndian>()?;

        let mut reserved = [0; 8];
        source.read_exact(&mut reserved)?;

        if reserved != [0; 8] {
            panic!("Invalid reserved bytes");
        }

        Ok(Self {
            num_instance_types,
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
