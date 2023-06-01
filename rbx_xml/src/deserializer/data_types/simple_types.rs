//! Implements deserialization for simple to parse types.
//! Namely:
//! - `bool`
//! - `i32`, `i64`, `f32`, `f64`
//! - `String`
use std::io::BufRead;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn bool_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<bool, DecodeError> {
    let content = reader.eat_text()?;
    match content.as_str() {
        //TODO check if Roblox follows XSD for bool parsing
        //(XSD allows `1` and `0` for bools)
        "true" => Ok(true),
        "false" => Ok(false),
        _ => reader.error("invalid bool '{content}', should be either 'true' or 'false'"),
    }
}

pub fn string_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<String, DecodeError> {
    reader.eat_text()
}

pub fn f32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f32, DecodeError> {
    let content = reader.eat_text()?;
    match content.as_str().parse() {
        Ok(val) => Ok(val),
        Err(_) => reader.error(format!("invalid f32 (float) value '{content}'")),
    }
}

pub fn f64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f64, DecodeError> {
    let content = reader.eat_text()?;
    match content.as_str().parse() {
        Ok(val) => Ok(val),
        Err(_) => reader.error(format!("invalid f64 (double) value '{content}'")),
    }
}

pub fn i32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i32, DecodeError> {
    let content = reader.eat_text()?;
    match content.as_str().parse() {
        Ok(val) => Ok(val),
        Err(_) => reader.error(format!("invalid i32 (int) value '{content}'")),
    }
}

pub fn i64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i64, DecodeError> {
    let content = reader.eat_text()?;
    match content.as_str().parse() {
        Ok(val) => Ok(val),
        Err(_) => reader.error(format!("invalid i64 (int64) value '{content}'")),
    }
}
