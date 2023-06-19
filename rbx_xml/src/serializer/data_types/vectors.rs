use std::io;

use rbx_dom_weak::types::{Vector2, Vector3, Vector3int16};

use super::XmlWriter;
use crate::EncodeError;

pub fn vector3_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector3,
) -> Result<(), EncodeError> {
    writer.write_rbx("X", value.x)?;
    writer.write_rbx("Y", value.y)?;
    writer.write_rbx("Z", value.z)?;

    Ok(())
}

pub fn vector2_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector2,
) -> Result<(), EncodeError> {
    writer.write_rbx("X", value.x)?;
    writer.write_rbx("Y", value.y)?;

    Ok(())
}

pub fn vector3int16_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector3int16,
) -> Result<(), EncodeError> {
    writer.write_element("X", value.x)?;
    writer.write_element("Y", value.y)?;
    writer.write_element("Z", value.z)?;
    Ok(())
}
