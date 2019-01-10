use std::{
    io::{self, Cursor, Write},
    collections::HashMap,
    borrow::{Borrow, Cow},
    u32,
};

use byteorder::{WriteBytesExt, LittleEndian};
use rbx_tree::{RbxTree, RootedRbxInstance, RbxId, RbxValue};

use crate::{
    core::{
        FILE_MAGIC_HEADER,
        FILE_SIGNATURE,
        FILE_VERSION,
    },
    types::{
        encode_referent_array,
        encode_string,
        encode_bool,
    },
};

static FILE_FOOTER: &[u8] = b"</roblox>";

/// Serialize the instances denoted by `ids` from `tree` to XML.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], mut output: W) -> io::Result<()> {
    let relevant_instances = gather_instances(tree, ids);
    let type_infos = generate_type_infos(&relevant_instances);
    let referents = generate_referents(&relevant_instances);

    output.write_all(FILE_MAGIC_HEADER)?;
    output.write_all(FILE_SIGNATURE)?;
    output.write_u16::<LittleEndian>(FILE_VERSION)?;

    output.write_u32::<LittleEndian>(type_infos.len() as u32)?;
    output.write_u32::<LittleEndian>(relevant_instances.len() as u32)?;
    output.write_u64::<LittleEndian>(0)?;

    // Type data
    for (type_name, type_info) in &type_infos {
        encode_chunk(&mut output, b"INST", Compression::Compressed, |mut output| {
            output.write_u32::<LittleEndian>(type_info.id)?;
            encode_string(&mut output, type_name)?;

            output.write_u8(0)?; // Flag that no additional data is attached

            output.write_u32::<LittleEndian>(type_info.object_ids.len() as u32)?;

            let type_referents = type_info.object_ids
                .iter()
                .map(|id| *referents.get(id).unwrap());
            encode_referent_array(&mut output, type_referents)?;

            Ok(())
        })?;
    }

    // Property data
    for (_type_name, type_info) in &type_infos {
        for prop_info in &type_info.properties {
            encode_chunk(&mut output, b"PROP", Compression::Compressed, |mut output| {
                output.write_u32::<LittleEndian>(type_info.id)?;
                encode_string(&mut output, &prop_info.name)?;

                let data_type = match prop_info.kind {
                    PropKind::String => 0x1,
                    PropKind::Bool => 0x2,
                };

                output.write_u8(data_type)?;

                for id in &type_info.object_ids {
                    let instance = relevant_instances.get(id).unwrap();
                    let value = match prop_info.name.as_str() {
                        "Name" => Cow::Owned(RbxValue::String {
                            value: instance.name.clone()
                        }),
                        _ => {
                            // TODO: String/type validation nonsense?
                            Cow::Borrowed(instance.properties.get(&prop_info.name).unwrap())
                        },
                    };

                    assert_eq!(PropKind::from_value(&value), prop_info.kind);

                    match value.borrow() {
                        RbxValue::String { value } => encode_string(&mut output, value)?,
                        RbxValue::Bool { value } => encode_bool(&mut output, *value)?,
                        _ => unimplemented!(),
                    }
                }

                Ok(())
            })?;
        }
    }

    encode_chunk(&mut output, b"PRNT", Compression::Compressed, |mut output| {
        output.write_u8(0)?; // Parent chunk data, version 0
        output.write_u32::<LittleEndian>(relevant_instances.len() as u32)?;

        let ids = relevant_instances
            .keys()
            .map(|id| *referents.get(id).unwrap());

        let parent_ids = relevant_instances
            .keys()
            .map(|id| {
                let instance = relevant_instances.get(id).unwrap();
                match instance.get_parent_id() {
                    Some(parent_id) => *referents.get(&parent_id).unwrap_or(&-1),
                    None => -1,
                }
            });

        encode_referent_array(&mut output, ids)?;
        encode_referent_array(&mut output, parent_ids)?;

        Ok(())
    })?;

    encode_chunk(&mut output, b"END\0", Compression::Uncompressed, |mut output| {
        output.write_all(FILE_FOOTER)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropKind {
    String,
    Bool,
}

impl PropKind {
    fn from_value(value: &RbxValue) -> PropKind {
        match value {
            RbxValue::String { .. } => PropKind::String,
            RbxValue::Bool { .. } => PropKind::Bool,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct PropInfo {
    name: String,
    kind: PropKind,
}

#[derive(Debug)]
struct TypeInfo {
    id: u32,
    object_ids: Vec<RbxId>,
    properties: Vec<PropInfo>,
}

fn generate_type_infos<'a>(instances: &HashMap<RbxId, &'a RootedRbxInstance>) -> HashMap<&'a str, TypeInfo> {
    let mut type_infos = HashMap::new();
    let mut next_type_id = 0;

    for instance in instances.values() {
        let class_name = instance.class_name.as_str();

        let info = match type_infos.get_mut(class_name) {
            Some(info) => info,
            None => {
                let info = TypeInfo {
                    id: next_type_id,
                    object_ids: Vec::new(),
                    properties: vec![
                        PropInfo {
                            name: "Name".to_string(),
                            kind: PropKind::String,
                        },
                    ],
                };
                next_type_id += 1;

                type_infos.insert(class_name, info);
                type_infos.get_mut(class_name).unwrap()
            },
        };

        info.object_ids.push(instance.get_id());

        for (prop_name, prop_value) in &instance.properties {
            if info.properties.iter().find(|prop| &prop.name == prop_name).is_none() {
                let prop_info = PropInfo {
                    name: prop_name.clone(),
                    kind: PropKind::from_value(prop_value),
                };

                info.properties.push(prop_info);
            }
        }
    }

    type_infos
}

fn generate_referents(instances: &HashMap<RbxId, &RootedRbxInstance>) -> HashMap<RbxId, i32> {
    let mut referents = HashMap::new();
    let mut next_referent = 0;

    for instance in instances.values() {
        referents.insert(instance.get_id(), next_referent);
        next_referent += 1;
    }

    referents
}

fn gather_instances<'a>(tree: &'a RbxTree, ids: &[RbxId]) -> HashMap<RbxId, &'a RootedRbxInstance> {
    let mut output = HashMap::new();

    for id in ids {
        for descendant in tree.descendants(*id) {
            output.insert(descendant.get_id(), descendant);
        }
    }

    output
}

enum Compression {
    Compressed,
    Uncompressed,
}

fn encode_chunk<W: Write, F>(mut output: W, chunk_name: &[u8], compression: Compression, body: F) -> io::Result<()>
    where F: Fn(Cursor<&mut Vec<u8>>) -> io::Result<()>
{
    output.write_all(chunk_name)?;

    let mut buffer = Vec::new();
    body(Cursor::new(&mut buffer))?;

    match compression {
        Compression::Compressed => {
            let compressed = lz4::block::compress(&buffer, None, false)?;

            output.write_u32::<LittleEndian>(compressed.len() as u32)?;
            output.write_u32::<LittleEndian>(buffer.len() as u32)?;
            output.write_u32::<LittleEndian>(0)?;

            output.write_all(&compressed)?;
        },
        Compression::Uncompressed => {
            output.write_u32::<LittleEndian>(0)?;
            output.write_u32::<LittleEndian>(buffer.len() as u32)?;
            output.write_u32::<LittleEndian>(0)?;

            output.write_all(&buffer)?;
        },
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use rbx_tree::RbxInstance;

    fn new_test_tree() -> RbxTree {
        let instance = RbxInstance {
            name: "Folder".to_string(),
            class_name: "Folder".to_string(),
            properties: HashMap::new(),
        };

        RbxTree::new(instance)
    }

    #[test]
    fn test_encode() {
        let tree = new_test_tree();

        let output = Vec::new();
        encode(&tree, &[tree.get_root_id()], Cursor::new(output)).unwrap();
    }
}