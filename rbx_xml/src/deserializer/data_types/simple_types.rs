//! Implements deserialization for simple to parse types.
//! Namely:
//! - `bool`
//! - `i32`, `i64`, `f32`, `f64`, `Enum`
//! - `String`, `ProtectedString`, `BinaryString`
//!
//! Does not handle parsing particular `BinaryString` subtypes and instead
//! provides for parsing the raw base64 into a `rbx_types::BinaryString`.
use std::io::BufRead;

use rbx_dom_weak::types::{BinaryString, Enum};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn string_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<String, DecodeError> {
    reader.eat_text()
}

pub fn binary_string_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<BinaryString, DecodeError> {
    Ok(BinaryString::from(reader.eat_base64()?))
}

pub fn bool_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<bool, DecodeError> {
    let content = reader.eat_text()?;
    content
        .parse()
        .map_err(|_| reader.error(format!("value '{content}' is not a valid bool")))
}

pub fn f32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f32, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 32-bit float from `{content}` because {err}"
        ))
    })
}

pub fn f64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f64, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 64-bit float from `{content}` because {err}"
        ))
    })
}

pub fn i32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i32, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 32-bit int from `{content}` because {err}"
        ))
    })
}

pub fn i64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i64, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 64-bit int from `{content}` because {err}"
        ))
    })
}

pub fn enum_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Enum, DecodeError> {
    let content = reader.eat_text()?;
    content
        .parse()
        .map(Enum::from_u32)
        .map_err(|err| reader.error(format!("could not get Enum from `{content}` because {err}")))
}
