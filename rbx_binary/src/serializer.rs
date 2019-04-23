use std::{
    io::{self, Write},
    collections::HashMap,
    borrow::{Borrow, Cow},
    u32,
};

use byteorder::{WriteBytesExt, LittleEndian};
use rbx_dom_weak::{RbxTree, RbxInstance, RbxId, RbxValue, ExtractOwned, ExtractBorrowed};

use crate::{
    chunks::{encode_chunk, ChunkCompression},
    core::{
        BinaryType,
        FILE_MAGIC_HEADER,
        FILE_SIGNATURE,
        FILE_VERSION,
    },
    types::{
        BoolType,
        StringType,
        encode_referent_array,
    },
};

static FILE_FOOTER: &[u8] = b"</roblox>";

#[derive(Debug)]
pub enum EncodeError {
    IoError(io::Error),
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> EncodeError {
        EncodeError::IoError(error)
    }
}

/// Serialize the instances denoted by `ids` from `tree` to Roblox's binary
/// format.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], mut output: W) -> Result<(), EncodeError> {
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
        encode_chunk(&mut output, b"INST", ChunkCompression::Compressed, |mut output| {
            output.write_u32::<LittleEndian>(type_info.id)?;
            StringType::write_one(&mut output, type_name)?;

            // TODO: Set this flag for services?
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
    // TODO: This should become an iterator using encode_*_array instead of
    // individual encode methods to properly support interleaved data.
    for (_type_name, type_info) in &type_infos {
        for prop_info in &type_info.properties {
            encode_chunk(&mut output, b"PROP", ChunkCompression::Compressed, |mut output| {
                output.write_u32::<LittleEndian>(type_info.id)?;
                StringType::write_one(&mut output, &prop_info.name)?;
                output.write_u8(prop_info.kind.id())?;

                match prop_info.kind {
                    PropKind::String => {
                        let values = collect_values::<String>(&relevant_instances, prop_info, &type_info.object_ids);
                    },
                    PropKind::Bool => {
                        let values = collect_values_owned::<bool>(&relevant_instances, prop_info, &type_info.object_ids);
                    },
                }

                for instance_id in &type_info.object_ids {
                    let instance = relevant_instances.get(instance_id).unwrap();
                    let value = match prop_info.name.as_str() {
                        "Name" => Cow::Owned(RbxValue::String {
                            value: instance.name.clone()
                        }),
                        _ => {
                            // TODO: This is way wrong; we need type information
                            // to fall back to the correct default value.
                            let value = instance.properties.get(&prop_info.name)
                                .map(Cow::Borrowed)
                                .unwrap_or_else(|| Cow::Borrowed(prop_info.kind.default_value()));

                            // For now, we ensure that every instance of a given
                            // type pinky-promises to have the correct type.
                            // TODO: Turn this into a real error
                            assert_eq!(PropKind::from_value(&value), prop_info.kind);

                            value
                        },
                    };

                    assert_eq!(PropKind::from_value(&value), prop_info.kind);

                    match value.borrow() {
                        RbxValue::String { value } => StringType::write_one(&mut output, value)?,
                        RbxValue::Bool { value } => BoolType::write_one(&mut output, value)?,
                        _ => unimplemented!(),
                    }
                }

                Ok(())
            })?;
        }
    }

    encode_chunk(&mut output, b"PRNT", ChunkCompression::Compressed, |mut output| {
        output.write_u8(0)?; // Parent chunk data, version 0
        output.write_u32::<LittleEndian>(relevant_instances.len() as u32)?;

        let ids = relevant_instances
            .keys()
            .map(|id| *referents.get(id).unwrap());

        let parent_ids = relevant_instances
            .values()
            .map(|instance| {
                match instance.get_parent_id() {
                    Some(parent_id) => *referents.get(&parent_id).unwrap_or(&-1),
                    None => -1,
                }
            });

        encode_referent_array(&mut output, ids)?;
        encode_referent_array(&mut output, parent_ids)?;

        Ok(())
    })?;

    encode_chunk(&mut output, b"END\0", ChunkCompression::Uncompressed, |mut output| {
        output.write_all(FILE_FOOTER)
    })?;

    Ok(())
}

fn collect_values<'a, T>(
    relevant_instances: &'a HashMap<RbxId, &RbxInstance>,
    prop_info: &PropInfo,
    object_ids: &[RbxId],
) -> Vec<&'a T>
where
    RbxValue: ExtractBorrowed<T>,
{
    let mut output = Vec::with_capacity(object_ids.len());

    for instance_id in object_ids {
        let instance = relevant_instances.get(instance_id).unwrap();

        // TODO: This is way wrong; we need type information
        // to fall back to the correct default value.
        let value = instance.properties.get(&prop_info.name)
            .unwrap_or_else(|| prop_info.kind.default_value());

        assert_eq!(PropKind::from_value(&value), prop_info.kind);

        output.push(RbxValue::extract_borrowed(value).unwrap());
    }

    output
}

fn collect_values_owned<T>(
    relevant_instances: &HashMap<RbxId, &RbxInstance>,
    prop_info: &PropInfo,
    object_ids: &[RbxId],
) -> Vec<T>
where
    RbxValue: ExtractOwned<T>,
    T: Clone,
{
    let mut output = Vec::with_capacity(object_ids.len());

    for instance_id in object_ids {
        let instance = relevant_instances.get(instance_id).unwrap();

        // TODO: This is way wrong; we need type information
        // to fall back to the correct default value.
        let value = instance.properties.get(&prop_info.name)
            .unwrap_or_else(|| prop_info.kind.default_value());

        assert_eq!(PropKind::from_value(&value), prop_info.kind);

        output.push(RbxValue::extract_owned(value).unwrap());
    }

    output
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

    // This function requires type information to implement correctly!
    fn default_value(&self) -> &'static RbxValue {
        lazy_static::lazy_static! {
            static ref DEFAULT_STRING: RbxValue = RbxValue::String {
                value: String::new(),
            };

            static ref DEFAULT_BOOL: RbxValue = RbxValue::Bool {
                value: false,
            };
        }

        match self {
            PropKind::String => &DEFAULT_STRING,
            PropKind::Bool => &DEFAULT_BOOL,
        }
    }

    fn id(&self) -> u8 {
        match self {
            PropKind::String => 0x1,
            PropKind::Bool => 0x2,
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

fn generate_type_infos<'a>(instances: &HashMap<RbxId, &'a RbxInstance>) -> HashMap<&'a str, TypeInfo> {
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

fn generate_referents(instances: &HashMap<RbxId, &RbxInstance>) -> HashMap<RbxId, i32> {
    let mut referents = HashMap::new();
    let mut next_referent = 0;

    for instance in instances.values() {
        referents.insert(instance.get_id(), next_referent);
        next_referent += 1;
    }

    referents
}

fn gather_instances<'a>(tree: &'a RbxTree, ids: &[RbxId]) -> HashMap<RbxId, &'a RbxInstance> {
    let mut output = HashMap::new();

    for id in ids {
        for descendant in tree.descendants(*id) {
            output.insert(descendant.get_id(), descendant);
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    use std::{
        collections::HashMap,
        io::Cursor,
    };
    use rbx_dom_weak::RbxInstanceProperties;

    fn new_test_tree() -> RbxTree {
        let instance = RbxInstanceProperties {
            name: "Folder".to_string(),
            class_name: "Folder".to_string(),
            properties: HashMap::new(),
        };

        RbxTree::new(instance)
    }

    #[test]
    fn test_encode() {
        let _ = env_logger::try_init();
        let tree = new_test_tree();

        let output = Vec::new();
        encode(&tree, &[tree.get_root_id()], Cursor::new(output)).unwrap();
    }
}