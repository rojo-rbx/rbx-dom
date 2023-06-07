use std::io;

use rbx_dom_weak::types::BinaryString;

use super::XmlWriter;

use crate::serializer::error::EncodeError;

pub fn string_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &str,
) -> Result<(), EncodeError> {
    if value.starts_with(char::is_whitespace) || value.ends_with(char::is_whitespace) {
        writer.write_raw_text(value)
    } else {
        writer.write_text(value)
    }
}

pub fn binary_string_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &BinaryString,
) -> Result<(), EncodeError> {
    writer.write_base64(value.as_ref())
}

pub fn bool_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &bool,
) -> Result<(), EncodeError> {
    writer.write_text(match value {
        true => "true",
        false => "false",
    })
}

pub fn i32_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &i32,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_string())
}

pub fn i64_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &i64,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_string())
}

pub fn f32_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &f32,
) -> Result<(), EncodeError> {
    if value.is_nan() {
        writer.write_text("NAN")
    } else if *value == f32::INFINITY {
        writer.write_text("INF")
    } else if *value == f32::NEG_INFINITY {
        writer.write_text("-INF")
    } else {
        writer.write_text(&value.to_string())
    }
}

pub fn f64_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &f64,
) -> Result<(), EncodeError> {
    if value.is_nan() {
        writer.write_text("NAN")
    } else if *value == f64::INFINITY {
        writer.write_text("INF")
    } else if *value == f64::NEG_INFINITY {
        writer.write_text("-INF")
    } else {
        writer.write_text(&value.to_string())
    }
}
