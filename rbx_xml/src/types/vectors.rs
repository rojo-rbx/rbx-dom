use std::io::Read;

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
};

pub fn deserialize_vector2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let x: f64 = reader.read_tag_contents("X")?.parse()?;
    let y: f64 = reader.read_tag_contents("Y")?.parse()?;

    Ok(RbxValue::Vector2 {
        value: [x, y],
    })
}

pub fn deserialize_vector2int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;

    Ok(RbxValue::Vector2int16 {
        value: [x, y],
    })
}

pub fn deserialize_vector3<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let x: f64 = reader.read_tag_contents("X")?.parse()?;
    let y: f64 = reader.read_tag_contents("Y")?.parse()?;
    let z: f64 = reader.read_tag_contents("Z")?.parse()?;

    Ok(RbxValue::Vector3 {
        value: [x, y, z],
    })
}

pub fn deserialize_vector3int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;
    let z: i16 = reader.read_tag_contents("Z")?.parse()?;

    Ok(RbxValue::Vector3int16 {
        value: [x, y, z],
    })
}