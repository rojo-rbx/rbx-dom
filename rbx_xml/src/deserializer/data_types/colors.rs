use std::io::BufRead;

use rbx_dom_weak::types::{Color3, Color3uint8};

use super::f32_deserializer;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn color3_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Color3, DecodeError> {
    Ok(Color3::new(
        reader.read_named_with("R", f32_deserializer)?,
        reader.read_named_with("G", f32_deserializer)?,
        reader.read_named_with("B", f32_deserializer)?,
    ))
}

pub fn color3uint8_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<Color3uint8, DecodeError> {
    let content = reader.eat_text()?;
    match content.parse::<u32>() {
        Ok(val) => Ok(Color3uint8::new(
            (val >> 16 & 0xFF) as u8,
            (val >> 8 & 0xFF) as u8,
            (val & 0xFF) as u8,
        )),
        Err(err) => Err(reader.error(format!(
            "value '{content}' is not a valid Color3uint8 value: {err}"
        ))),
    }
}
