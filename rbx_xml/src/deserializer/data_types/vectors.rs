use std::io::BufRead;

use rbx_dom_weak::types::{Vector2, Vector3, Vector3int16};

use super::f32_deserializer;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn vector3_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Vector3, DecodeError> {
    reader.expect_start_with_name("X")?;
    let x = f32_deserializer(reader)?;
    reader.expect_end_with_name("X")?;
    reader.expect_start_with_name("Y")?;
    let y = f32_deserializer(reader)?;
    reader.expect_end_with_name("Y")?;
    reader.expect_start_with_name("Z")?;
    let z = f32_deserializer(reader)?;
    reader.expect_end_with_name("Z")?;

    Ok(Vector3::new(x, y, z))
}

pub fn vector2_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Vector2, DecodeError> {
    reader.expect_start_with_name("X")?;
    let x = f32_deserializer(reader)?;
    reader.expect_end_with_name("X")?;
    reader.expect_start_with_name("Y")?;
    let y = f32_deserializer(reader)?;
    reader.expect_end_with_name("Y")?;

    Ok(Vector2::new(x, y))
}

pub fn vector3int16_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<Vector3int16, DecodeError> {
    reader.expect_start_with_name("X")?;
    let x = match reader.eat_text()?.parse() {
        Ok(val) => val,
        Err(_) => return reader.error("invalid i16 value for Vector3int16.X"),
    };
    reader.expect_end_with_name("X")?;

    reader.expect_start_with_name("Y")?;
    let y = match reader.eat_text()?.parse() {
        Ok(val) => val,
        Err(_) => return reader.error("invalid i16 value for Vector3int16.Y"),
    };
    reader.expect_end_with_name("Y")?;

    reader.expect_start_with_name("Z")?;
    let z = match reader.eat_text()?.parse() {
        Ok(val) => val,
        Err(_) => return reader.error("invalid i16 value for Vector3int16.Z"),
    };
    reader.expect_end_with_name("Z")?;

    Ok(Vector3int16::new(x, y, z))
}
