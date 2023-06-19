use std::io;

use super::{EncodeError, XmlWriter};
use rbx_dom_weak::types::{Color3, Color3uint8};

pub fn color3_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Color3,
) -> Result<(), EncodeError> {
    writer.write_rbx("R", value.r)?;
    writer.write_rbx("G", value.g)?;
    writer.write_rbx("B", value.b)?;
    Ok(())
}

pub fn color3uint8_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Color3uint8,
) -> Result<(), EncodeError> {
    let packed: u32 = (value.r as u32) << 16 | (value.g as u32) << 8 | (value.b as u32);
    writer.write_text(&packed.to_string())?;
    Ok(())
}
