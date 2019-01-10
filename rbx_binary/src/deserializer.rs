use std::{
    io::{self, Cursor, Read},
    collections::HashMap,
    borrow::Cow,
    fmt,
    str,
};

use log::trace;
use byteorder::{ReadBytesExt, LittleEndian};
use rbx_tree::{RbxTree, RbxId, RbxValue};

use crate::{
    core::{
        FILE_MAGIC_HEADER,
        FILE_SIGNATURE,
        FILE_VERSION,
    },
    types::{
        decode_string,
        decode_string_array,
        decode_referent_array,
        decode_bool_array,
    },
};

#[derive(Debug)]
pub enum DecodeError {
    MissingMagicFileHeader,
    UnknownVersion,
    IoError(io::Error),
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> DecodeError {
        DecodeError::IoError(error)
    }
}

/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(tree: &mut RbxTree, parent_id: RbxId, mut source: R) -> Result<(), DecodeError> {
    let header = decode_file_header(&mut source)?;
    trace!("Number of types: {}", header.num_instance_types);
    trace!("Number of instances: {}", header.num_instances);

    let mut chunk_buffer = Vec::new();
    let mut metadata: HashMap<String, String> = HashMap::new();
    let mut instance_types: HashMap<u32, InstanceType> = HashMap::new();
    let mut instance_props: HashMap<i32, InstanceProps> = HashMap::new();
    let mut instance_parents: HashMap<i32, i32> = HashMap::new();

    loop {
        let header = decode_chunk(&mut source, &mut chunk_buffer)?;
        let cursor = Cursor::new(&chunk_buffer);

        match &header.name {
            b"META" => {
                decode_metadata_chunk(cursor, &mut metadata)?;
            },
            b"INST" => {
                decode_inst_chunk(cursor, &mut instance_types)?;
            },
            b"PROP" => {
                decode_prop_chunk(cursor, &instance_types, &mut instance_props)?;
            },
            b"PRNT" => {
                decode_prnt_chunk(cursor, &mut instance_parents)?;
            },
            b"END\0" => break,
            _ => {
                // Unknown chunk
            },
        }

        chunk_buffer.clear();
    }

    trace!("Instance types: {:#?}", instance_types);
    trace!("Instance props: {:#?}", instance_props);
    trace!("Instance parents: {:#?}", instance_parents);

    Ok(())
}

struct FileHeader {
    pub num_instance_types: u32,
    pub num_instances: u32,
}

fn decode_file_header<R: Read>(mut source: R) -> Result<FileHeader, DecodeError> {
    let mut magic_header = [0; 8];
    source.read_exact(&mut magic_header)?;

    if &magic_header != FILE_MAGIC_HEADER {
        return Err(DecodeError::MissingMagicFileHeader);
    }

    let mut signature = [0; 6];
    source.read_exact(&mut signature)?;

    if &signature != FILE_SIGNATURE {
        return Err(DecodeError::MissingMagicFileHeader);
    }

    let version = source.read_u16::<LittleEndian>()?;

    if version != FILE_VERSION {
        return Err(DecodeError::UnknownVersion);
    }

    let num_instance_types = source.read_u32::<LittleEndian>()?;
    let num_instances = source.read_u32::<LittleEndian>()?;

    let mut reserved = [0; 8];
    source.read_exact(&mut reserved)?;

    Ok(FileHeader {
        num_instance_types,
        num_instances,
    })
}

#[derive(Debug)]
struct ChunkHeader {
    pub name: [u8; 4],
    pub compressed_len: u32,
    pub len: u32,
    pub reserved: u32,
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let name = if let Ok(name) = str::from_utf8(&self.name) {
            Cow::Borrowed(name)
        } else {
            Cow::Owned(format!("{:?}", self.name))
        };

        write!(output, "Chunk \"{}\" (compressed: {}, len: {}, reserved: {})",
            name, self.compressed_len, self.len, self.reserved)
    }
}

fn decode_chunk_header<R: Read>(mut source: R) -> io::Result<ChunkHeader> {
    let mut name = [0; 4];
    source.read_exact(&mut name)?;

    let compressed_len = source.read_u32::<LittleEndian>()?;
    let len = source.read_u32::<LittleEndian>()?;
    let reserved = source.read_u32::<LittleEndian>()?;

    Ok(ChunkHeader {
        name,
        compressed_len,
        len,
        reserved,
    })
}

fn decode_chunk<R: Read>(mut source: R, output: &mut Vec<u8>) -> io::Result<ChunkHeader> {
    let header = decode_chunk_header(&mut source)?;

    trace!("{}", header);

    if header.compressed_len == 0 {
        (&mut source).take(header.len as u64).read_to_end(output)?;
    } else {
        let mut compressed_data = Vec::new();
        (&mut source).take(header.compressed_len as u64).read_to_end(&mut compressed_data)?;

        let data = lz4::block::decompress(&compressed_data, Some(header.len as i32))?;
        output.extend_from_slice(&data);
    }

    assert_eq!(output.len(), header.len as usize);

    Ok(header)
}

fn decode_metadata_chunk<R: Read>(mut source: R, output: &mut HashMap<String, String>) -> io::Result<()> {
    let len = source.read_u32::<LittleEndian>()?;

    for _ in 0..len {
        let key = decode_string(&mut source)?;
        let value = decode_string(&mut source)?;

        output.insert(key, value);
    }

    Ok(())
}

#[derive(Debug)]
struct InstanceType {
    type_id: u32,
    type_name: String,
    referents: Vec<i32>,
}

fn decode_inst_chunk<R: Read>(mut source: R, instance_types: &mut HashMap<u32, InstanceType>) -> io::Result<()> {
    let type_id = source.read_u32::<LittleEndian>()?;
    let type_name = decode_string(&mut source)?;
    let additional_data = source.read_u8()?;
    let number_instances = source.read_u32::<LittleEndian>()?;

    let mut referents = vec![0; number_instances as usize];
    decode_referent_array(&mut source, &mut referents)?;

    trace!("{} instances of type ID {} ({})", number_instances, type_id, type_name);
    trace!("Referents found: {:?}", referents);

    instance_types.insert(type_id, InstanceType {
        type_id,
        type_name,
        referents,
    });

    Ok(())
}

#[derive(Debug)]
struct InstanceProps {
    type_id: u32,
    referent: i32,
    properties: HashMap<String, RbxValue>,
}

fn decode_prop_chunk<R: Read>(
    mut source: R,
    instance_types: &HashMap<u32, InstanceType>,
    instance_props: &mut HashMap<i32, InstanceProps>,
) -> io::Result<()> {
    let type_id = source.read_u32::<LittleEndian>()?;
    let prop_name = decode_string(&mut source)?;
    let data_type = source.read_u8()?;

    trace!("Set prop (type {}) {}.{}", data_type, type_id, prop_name);

    // TODO: Convert to new error type instead of panic
    let instance_type = instance_types.get(&type_id)
        .expect("Could not find instance type!");

    match data_type {
        0x01 => {
            let values = decode_string_array(&mut source, instance_type.referents.len())?;

            for (index, value) in values.iter().enumerate() {
                let referent = instance_type.referents[index];
                let prop_data = instance_props
                    .entry(referent)
                    .or_insert(InstanceProps {
                        type_id,
                        referent,
                        properties: HashMap::new(),
                    });

                prop_data.properties.insert(prop_name.clone(), RbxValue::String { value: value.clone() });
            }
        },
        0x02 => {
            let values = decode_bool_array(&mut source, instance_type.referents.len())?;

            for (index, &value) in values.iter().enumerate() {
                let referent = instance_type.referents[index];
                let prop_data = instance_props
                    .entry(referent)
                    .or_insert(InstanceProps {
                        type_id,
                        referent,
                        properties: HashMap::new(),
                    });

                prop_data.properties.insert(prop_name.clone(), RbxValue::Bool { value });
            }
        },
        0x03 => { /* i32 array */ },
        0x04 => { /* f32 array */ },
        0x05 => { /* f64 array */ },
        0x06 => { /* UDim array? */ },
        0x07 => { /* UDim2 array */ },
        0x08 => { /* Ray array */ },
        0x09 => { /* Faces array */ },
        0x0A => { /* Axis array */ },
        0x0B => { /* BrickColor array */ },
        0x0C => { /* Color3 array */ },
        0x0D => { /* Vector2 array */ },
        0x0E => { /* Vector3 array */ },
        0x10 => { /* CFrame array */ },
        0x12 => { /* Enum array */ },
        0x13 => { /* Referent array */ },
        _ => {
            trace!("Unknown prop type {} named {}", data_type, prop_name);
        },
    }

    Ok(())
}

fn decode_prnt_chunk<R: Read>(mut source: R, instance_parents: &mut HashMap<i32, i32>) -> io::Result<()> {
    let version = source.read_u8()?;

    if version != 0 {
        // TODO: Warn for version mismatch?
        return Ok(());
    }

    let number_objects = source.read_u32::<LittleEndian>()?;

    trace!("{} objects with parents", number_objects);

    let mut instance_ids = vec![0; number_objects as usize];
    let mut parent_ids = vec![0; number_objects as usize];

    decode_referent_array(&mut source, &mut instance_ids)?;
    decode_referent_array(&mut source, &mut parent_ids)?;

    for (id, parent_id) in instance_ids.iter().zip(&parent_ids) {
        instance_parents.insert(*id, *parent_id);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    use rbx_tree::RbxInstance;

    static MODEL_A: &[u8] = include_bytes!("../test-files/model-a.rbxm");
    static MODEL_B: &[u8] = include_bytes!("../test-files/model-b.rbxm");
    static MODEL_C: &[u8] = include_bytes!("../test-files/model-c.rbxm");

    fn new_test_tree() -> RbxTree {
        let root = RbxInstance {
            name: "Folder".to_string(),
            class_name: "Folder".to_string(),
            properties: HashMap::new(),
        };

        RbxTree::new(root)
    }

    #[test]
    fn test_decode() {
        let _ = env_logger::try_init();

        for model_source in &[MODEL_A, MODEL_B, MODEL_C] {
            let mut tree = new_test_tree();
            let root_id = tree.get_root_id();

            print!("\n");
            trace!("Model:");
            decode(&mut tree, root_id, *model_source).unwrap();
        }
    }
}