use std::io::Read;

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
};

pub fn deserialize_vector2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector2")?;

    let x: f64 = reader.read_tag_contents("X")?.parse()?;
    let y: f64 = reader.read_tag_contents("Y")?.parse()?;

    reader.expect_end_with_name("Vector2")?;

    Ok(RbxValue::Vector2 {
        value: [x, y],
    })
}

pub fn deserialize_vector2int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector2int16")?;

    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;

    reader.expect_end_with_name("Vector2int16")?;

    Ok(RbxValue::Vector2int16 {
        value: [x, y],
    })
}

pub fn deserialize_vector3<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector3")?;

    let x: f64 = reader.read_tag_contents("X")?.parse()?;
    let y: f64 = reader.read_tag_contents("Y")?.parse()?;
    let z: f64 = reader.read_tag_contents("Z")?.parse()?;

    reader.expect_end_with_name("Vector3")?;

    Ok(RbxValue::Vector3 {
        value: [x, y, z],
    })
}

pub fn deserialize_vector3int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector3int16")?;

    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;
    let z: i16 = reader.read_tag_contents("Z")?.parse()?;

    reader.expect_end_with_name("Vector3int16")?;

    Ok(RbxValue::Vector3int16 {
        value: [x, y, z],
    })
}