use std::{
    collections::HashMap,
    io::{self, Cursor, Read},
    str,
};

use byteorder::{LittleEndian, ReadBytesExt};
use log::trace;
use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree, RbxValue};

use crate::{
    chunk::decode_chunk,
    core::{BinaryType, RbxReadExt, FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION},
    types::{decode_referent_array, BoolType, StringType},
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
pub fn decode<R: Read>(
    tree: &mut RbxTree,
    parent_id: RbxId,
    mut source: R,
) -> Result<(), DecodeError> {
    let header = decode_file_header(&mut source)?;
    trace!("Number of types: {}", header.num_instance_types);
    trace!("Number of instances: {}", header.num_instances);

    let mut metadata: HashMap<String, String> = HashMap::new();
    let mut instance_types: HashMap<u32, InstanceType> = HashMap::new();
    let mut instance_props: HashMap<i32, InstanceProps> = HashMap::new();
    let mut instance_parents: HashMap<i32, i32> = HashMap::new();

    loop {
        let chunk = decode_chunk(&mut source)?;
        let mut cursor = Cursor::new(&chunk.data);

        match &chunk.name {
            b"META" => decode_meta_chunk(&mut cursor, &mut metadata)?,
            b"INST" => decode_inst_chunk(&mut cursor, &mut instance_types)?,
            b"PROP" => decode_prop_chunk(&mut cursor, &instance_types, &mut instance_props)?,
            b"PRNT" => decode_prnt_chunk(&mut cursor, &mut instance_parents)?,
            b"END\0" => break,
            _ => match str::from_utf8(&chunk.name) {
                Ok(name) => trace!("Unknown chunk name {}", name),
                Err(_) => trace!("Unknown chunk name {:?}", chunk.name),
            },
        }
    }

    trace!("Instance types: {:#?}", instance_types);
    trace!("Instance props: {:#?}", instance_props);
    trace!("Instance parents: {:#?}", instance_parents);

    let mut parents_to_children: HashMap<i32, Vec<i32>> = HashMap::new();
    for (referent, parent_referent) in &instance_parents {
        parents_to_children
            .entry(*parent_referent)
            .or_default()
            .push(*referent);
    }

    if let Some(root_referents) = parents_to_children.get(&-1) {
        for referent in root_referents {
            construct_and_parent(
                tree,
                parent_id,
                *referent,
                &parents_to_children,
                &instance_types,
                &instance_props,
            );
        }
    }

    Ok(())
}

fn construct_and_parent(
    tree: &mut RbxTree,
    parent_id: RbxId,
    referent: i32,
    parents_to_children: &HashMap<i32, Vec<i32>>,
    instance_types: &HashMap<u32, InstanceType>,
    instance_props: &HashMap<i32, InstanceProps>,
) {
    let props = instance_props
        .get(&referent)
        .expect("Could not find props for referent listed in PRNT chunk");

    let type_info = instance_types
        .get(&props.type_id)
        .expect("Could not find type information for referent");

    let mut properties = HashMap::new();
    for (key, value) in &props.properties {
        if key != "Name" {
            properties.insert(key.clone(), value.clone());
        }
    }

    let name = props
        .properties
        .get("Name")
        .map(|name| match name {
            RbxValue::String { value } => value.clone(),
            _ => panic!("Invalid non-string type used for 'Name' property"),
        })
        .unwrap_or_else(|| type_info.type_name.clone());

    let instance = RbxInstanceProperties {
        name,
        class_name: type_info.type_name.clone(),
        properties,
    };

    let id = tree.insert_instance(instance, parent_id);

    if let Some(child_referents) = parents_to_children.get(&referent) {
        for child_referent in child_referents {
            construct_and_parent(
                tree,
                id,
                *child_referent,
                parents_to_children,
                instance_types,
                instance_props,
            );
        }
    }
}

struct FileHeader {
    pub num_instance_types: u32,
    pub num_instances: u32,
}

fn decode_file_header<R: Read>(source: &mut R) -> Result<FileHeader, DecodeError> {
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

fn decode_meta_chunk<R: Read>(
    source: &mut R,
    output: &mut HashMap<String, String>,
) -> io::Result<()> {
    let len = source.read_u32::<LittleEndian>()?;

    for _ in 0..len {
        let key = source.read_string()?;
        let value = source.read_string()?;

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

fn decode_inst_chunk<R: Read>(
    source: &mut R,
    instance_types: &mut HashMap<u32, InstanceType>,
) -> io::Result<()> {
    let type_id = source.read_u32::<LittleEndian>()?;
    let type_name = source.read_string()?;
    let _additional_data = source.read_u8()?;
    let number_instances = source.read_u32::<LittleEndian>()?;

    let mut referents = vec![0; number_instances as usize];
    decode_referent_array(source, &mut referents)?;

    trace!(
        "{} instances of type ID {} ({})",
        number_instances,
        type_id,
        type_name
    );
    trace!("Referents found: {:?}", referents);

    instance_types.insert(
        type_id,
        InstanceType {
            type_id,
            type_name,
            referents,
        },
    );

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
    let prop_name = source.read_string()?;
    let data_type = source.read_u8()?;

    trace!("Set prop (type {}) {}.{}", data_type, type_id, prop_name);

    // TODO: Convert to new error type instead of panic
    let instance_type = instance_types
        .get(&type_id)
        .expect("Could not find instance type!");

    match data_type {
        0x01 => {
            let values = StringType::read_array(&mut source, instance_type.referents.len())?;

            for (index, value) in values.into_iter().enumerate() {
                let referent = instance_type.referents[index];
                let prop_data = instance_props.entry(referent).or_insert(InstanceProps {
                    type_id,
                    referent,
                    properties: HashMap::new(),
                });

                prop_data.properties.insert(prop_name.clone(), value);
            }
        }
        0x02 => {
            let values = BoolType::read_array(&mut source, instance_type.referents.len())?;

            for (index, value) in values.into_iter().enumerate() {
                let referent = instance_type.referents[index];
                let prop_data = instance_props.entry(referent).or_insert(InstanceProps {
                    type_id,
                    referent,
                    properties: HashMap::new(),
                });

                prop_data.properties.insert(prop_name.clone(), value);
            }
        }
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
            trace!("Unknown prop type {} named {}", data_type, prop_name);
        }
    }

    Ok(())
}

fn decode_prnt_chunk<R: Read>(
    source: &mut R,
    instance_parents: &mut HashMap<i32, i32>,
) -> io::Result<()> {
    let version = source.read_u8()?;

    if version != 0 {
        // TODO: Warn for version mismatch?
        return Ok(());
    }

    let number_objects = source.read_u32::<LittleEndian>()?;

    trace!("{} objects with parents", number_objects);

    let mut instance_ids = vec![0; number_objects as usize];
    let mut parent_ids = vec![0; number_objects as usize];

    decode_referent_array(source, &mut instance_ids)?;
    decode_referent_array(source, &mut parent_ids)?;

    for (id, parent_id) in instance_ids.iter().zip(&parent_ids) {
        instance_parents.insert(*id, *parent_id);
    }

    Ok(())
}
