use std::io::BufRead;

use rbx_dom_weak::types::{Vector2, Vector3, Vector3int16};

use super::f32_deserializer;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn vector3_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Vector3, DecodeError> {
    Ok(Vector3::new(
        reader.read_named_with("X", f32_deserializer)?,
        reader.read_named_with("Y", f32_deserializer)?,
        reader.read_named_with("Z", f32_deserializer)?,
    ))
}

pub fn vector2_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Vector2, DecodeError> {
    Ok(Vector2::new(
        reader.read_named_with("X", f32_deserializer)?,
        reader.read_named_with("Y", f32_deserializer)?,
    ))
}

pub fn vector3int16_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<Vector3int16, DecodeError> {
    Ok(Vector3int16::new(
        reader.read_named_with("X", i16_deserializer)?,
        reader.read_named_with("Y", i16_deserializer)?,
        reader.read_named_with("Z", i16_deserializer)?,
    ))
}

fn i16_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i16, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not read 16-bit int from `{content}` because {err}"
        ))
    })
}
