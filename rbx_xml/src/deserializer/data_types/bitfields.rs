//! Implements deserialization for Axes and Faces, which are stored as
//! bitfields internally.

use std::io::BufRead;

use rbx_dom_weak::types::{Axes, Faces};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn axes_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Axes, DecodeError> {
    reader.expect_start_with_name("axes")?;
    let content = reader.eat_text()?;
    let ret = match content.parse() {
        Ok(val) => Axes::from_bits(val)
            .ok_or_else(|| reader.error(format!("invalid Axes value `{content}`"))),
        Err(_) => Err(reader.error(format!("invalid Axes value `{content}`"))),
    };
    reader.expect_end_with_name("axes")?;
    ret
}

pub fn faces_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Faces, DecodeError> {
    reader.expect_start_with_name("faces")?;
    let content = reader.eat_text()?;
    let ret = match content.parse() {
        Ok(val) => Faces::from_bits(val)
            .ok_or_else(|| reader.error(format!("invalid Faces value `{content}`"))),
        Err(_) => Err(reader.error(format!("invalid Faces value `{content}`"))),
    };
    reader.expect_end_with_name("faces")?;
    ret
}
