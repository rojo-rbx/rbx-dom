//! Implements deserialization for `Rect`

use std::io::BufRead;

use rbx_dom_weak::types::Rect;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

use super::vector2_deserializer;

pub fn rect_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Rect, DecodeError> {
    Ok(Rect::new(
        reader.read_named_with("min", vector2_deserializer)?,
        reader.read_named_with("max", vector2_deserializer)?,
    ))
}
