use std::{
    io::{self, Cursor, Read},
    collections::HashMap,
    borrow::Cow,
    mem,
    fmt,
    str,
};

use byteorder::{ReadBytesExt, LittleEndian};
use rbx_tree::{RbxTree, RbxId};

use crate::core::{
    FILE_MAGIC_HEADER,
    FILE_SIGNATURE,
    FILE_VERSION,
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
    println!("Number of types: {}", header.num_instance_types);
    println!("Number of instances: {}", header.num_instances);

    let mut chunk_buffer = Vec::new();
    let mut metadata = HashMap::new();

    loop {
        let header = decode_chunk(&mut source, &mut chunk_buffer)?;

        match &header.name {
            b"META" => {
                decode_metadata_chunk(&chunk_buffer, &mut metadata)?;
            },
            b"INST" => {
                decode_inst_chunk(&chunk_buffer)?;
            },
            b"PROP" => {
                decode_prop_chunk(&chunk_buffer)?;
            },
            b"PRNT" => {
                decode_prnt_chunk(&chunk_buffer)?;
            },
            b"END\0" => break,
            _ => {
                // Unknown chunk
            },
        }

        chunk_buffer.clear();
    }

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

    println!("{}", header);

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

fn decode_metadata_chunk(buffer: &[u8], output: &mut HashMap<String, String>) -> io::Result<()> {
    let mut source = Cursor::new(buffer);
    let len = source.read_u32::<LittleEndian>()?;

    for _ in 0..len {
        let key = decode_string(&mut source)?;
        let value = decode_string(&mut source)?;

        output.insert(key, value);
    }

    Ok(())
}

fn decode_inst_chunk(buffer: &[u8]) -> io::Result<()> {
    let mut source = Cursor::new(buffer);
    let type_id = source.read_u32::<LittleEndian>()?;
    let type_name = decode_string(&mut source)?;
    let additional_data = source.read_u8()?;
    let number_instances = source.read_u32::<LittleEndian>()?;

    let mut referents = vec![0; number_instances as usize];
    decode_id_array(&mut source, &mut referents)?;

    println!("{} instances of type ID {} ({})", number_instances, type_id, type_name);
    println!("Referents found: {:?}", referents);

    Ok(())
}

fn decode_prop_chunk(buffer: &[u8]) -> io::Result<()> {
    let mut source = Cursor::new(buffer);
    let type_id = source.read_u32::<LittleEndian>()?;
    let prop_name = decode_string(&mut source)?;
    let data_type = source.read_u8()?;

    // TODO: Read data

    println!("Set prop {}.{}", type_id, prop_name);

    Ok(())
}

fn decode_prnt_chunk(buffer: &[u8]) -> io::Result<()> {
    let mut source = Cursor::new(buffer);
    source.read_u8()?; // Reserved
    let number_objects = source.read_u32::<LittleEndian>()?;

    println!("There are {} objects with parents.", number_objects);

    let mut instance_ids = vec![0; number_objects as usize];
    let mut parent_ids = vec![0; number_objects as usize];

    decode_id_array(&mut source, &mut instance_ids)?;
    decode_id_array(&mut source, &mut parent_ids)?;

    for (id, parent_id) in instance_ids.iter().zip(&parent_ids) {
        println!("Parent of {} is {}", id, parent_id);
    }

    Ok(())
}

fn decode_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

fn decode_id_array<R: Read>(source: R, output: &mut [i32]) -> io::Result<()> {
    decode_i32_array(source, output)?;
    let mut last = 0;

    for i in 0..output.len() {
        output[i] += last;
        last = output[i];
    }

    Ok(())
}

fn decode_i32_array<R: Read>(mut source: R, output: &mut [i32]) -> io::Result<()> {
    let mut buffer = vec![0; output.len() * mem::size_of::<i32>()];
    source.read_exact(&mut buffer)?;

    decode_i32_array_from_buffer(&buffer, output);

    Ok(())
}

fn decode_i32_array_from_buffer(buffer: &[u8], output: &mut [i32]) {
    for i in 0..output.len() {
        let v0 = buffer[i] as i32;
        let v1 = buffer[i + output.len()] as i32;
        let v2 = buffer[i + output.len() * 2] as i32;
        let v3 = buffer[i + output.len() * 3] as i32;

        output[i] = decode_i32((v0 << 24) | (v1 << 16) | (v2 << 8) | v3);
    }
}

fn decode_string<R: Read>(mut source: R) -> io::Result<String> {
    let length = source.read_u32::<LittleEndian>()?;

    let mut value = String::new();
    (&mut source).take(length as u64).read_to_string(&mut value)?;

    Ok(value)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    use rbx_tree::RbxInstance;

    static MODEL_A: &[u8] = include_bytes!("../test-files/model-a.rbxm");
    static MODEL_B: &[u8] = include_bytes!("../test-files/model-b.rbxm");

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
        for model_source in &[MODEL_A, MODEL_B] {
            let mut tree = new_test_tree();
            let root_id = tree.get_root_id();

            print!("\n");
            println!("Model:");
            decode(&mut tree, root_id, *model_source).unwrap();
        }
    }
}