use std::{
    io::{self, Write},
    collections::HashMap,
    borrow::{Borrow, Cow},
    u32,
};

use byteorder::{WriteBytesExt, LittleEndian, BigEndian};
use lz4::{Encoder, EncoderBuilder};
use rbx_tree::{RbxTree, RootedRbxInstance, RbxId, RbxValue};

static FILE_HEADER: &[u8] = b"<roblox!\x89\xff\x0d\x0a\x1a\x0a\x00\x00";
static FILE_FOOTER: &[u8] = b"END\x00\x00\x00\x00\x00\x09\x00\x00\x00\x00\x00\x00\x00</roblox>";

/// Serialize the instances denoted by `ids` from `tree` to XML.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], mut output: W) -> io::Result<()> {
    let relevant_instances = gather_instances(tree, ids);
    let type_infos = generate_type_infos(&relevant_instances);
    let referents = generate_referents(&relevant_instances);

    output.write_all(FILE_HEADER)?;

    output.write_u32::<LittleEndian>(type_infos.len() as u32)?;
    output.write_u32::<LittleEndian>(relevant_instances.len() as u32)?;
    output.write_u64::<LittleEndian>(0)?;

    let encoder_builder = EncoderBuilder::new();

    // Type data
    for (type_name, type_info) in &type_infos {
        encode_chunk(&mut output, b"INST", |mut encoder| {
            encoder.write_u32::<LittleEndian>(type_info.id)?;
            encode_string(&mut encoder, type_name)?;

            encoder.write_u8(0)?; // Flag that no additional data is attached

            encoder.write_u32::<LittleEndian>(type_info.object_ids.len() as u32)?;

            let type_referents = type_info.object_ids
                .iter()
                .map(|id| *referents.get(id).unwrap());
            encode_transformed_interleaved_u32_array(&mut encoder, type_referents)?;

            Ok(())
        })?;
    }

    // Property data
    for (type_name, type_info) in &type_infos {
        for prop_info in &type_info.properties {
            encode_chunk(&mut output, b"PROP", |mut encoder| {
                encoder.write_u32::<LittleEndian>(type_info.id)?;
                encode_string(&mut encoder, &prop_info.name)?;

                let data_type = match prop_info.kind {
                    PropKind::String => 0x1,
                };

                encoder.write_u8(data_type)?;

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
                        RbxValue::String { value } => encode_string(&mut encoder, value)?,
                        _ => unimplemented!(),
                    }
                }

                Ok(())
            })?;
        }
    }

    encode_chunk(&mut output, b"PRNT", |mut encoder| {
        encoder.write_u8(0)?; // Unknown byte
        encoder.write_u32::<LittleEndian>(relevant_instances.len() as u32)?;

        let ids = relevant_instances
            .keys()
            .map(|id| *referents.get(id).unwrap());

        let parent_ids = relevant_instances
            .keys()
            .map(|id| {
                let instance = relevant_instances.get(id).unwrap();
                match instance.get_parent_id() {
                    Some(parent_id) => *referents.get(&parent_id).unwrap(),
                    None => u32::MAX,
                }
            });

        encode_transformed_interleaved_u32_array(&mut encoder, ids)?;
        encode_transformed_interleaved_u32_array(&mut encoder, parent_ids)?;

        Ok(())
    })?;

    output.write_all(FILE_FOOTER)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropKind {
    String,
}

impl PropKind {
    fn from_value(value: &RbxValue) -> PropKind {
        match value {
            RbxValue::String { .. } => PropKind::String,
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

fn generate_referents(instances: &HashMap<RbxId, &RootedRbxInstance>) -> HashMap<RbxId, u32> {
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

fn encode_string<W: Write>(mut output: W, value: &str) -> io::Result<()> {
    output.write_u32::<LittleEndian>(value.len() as u32)?;
    write!(output, "{}", value)?;
    Ok(())
}

fn transform_u32(value: u32) -> u32 {
    (value << 1) ^ (value >> 31)
}

fn encode_chunk<W: Write, F>(mut output: W, chunk_name: &[u8], body: F) -> io::Result<()>
    where F: Fn(Encoder<W>) -> io::Result<()>
{
    output.write_all(chunk_name)?;

    let encoder = EncoderBuilder::new()
        .build(output)?;

    body(encoder)
}

fn encode_transformed_interleaved_u32_array<W: Write, I>(mut output: W, values: I) -> io::Result<()>
    where I: Iterator<Item = u32> + Clone
{
    for shift in &[24, 16, 8, 0] {
        for value in values.clone() {
            let encoded = transform_u32(value) >> shift;
            output.write_u8(encoded as u8)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use std::fs::File;
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

        let mut output = File::create("test-output.rbxm").unwrap();
        encode(&tree, &[tree.get_root_id()], &mut output).unwrap();
    }
}