use std::io;

use rbx_dom_weak::types::{Vector2, Vector3, Vector3int16};

use super::f32_serializer;
use super::XmlWriter;
use crate::EncodeError;

pub fn vector3_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector3,
) -> Result<(), EncodeError> {
    writer.start_element("X").finalize()?;
    f32_serializer(writer, &value.x)?;
    writer.end_element("X")?;

    writer.start_element("Y").finalize()?;
    f32_serializer(writer, &value.y)?;
    writer.end_element("Y")?;

    writer.start_element("Z").finalize()?;
    f32_serializer(writer, &value.z)?;
    writer.end_element("Z")?;

    Ok(())
}

pub fn vector2_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector2,
) -> Result<(), EncodeError> {
    writer.start_element("X").finalize()?;
    f32_serializer(writer, &value.x)?;
    writer.end_element("X")?;

    writer.start_element("Y").finalize()?;
    f32_serializer(writer, &value.y)?;
    writer.end_element("Y")?;

    Ok(())
}

pub fn vector3int16_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Vector3int16,
) -> Result<(), EncodeError> {
    writer.start_element("X").finalize()?;
    writer.write_text(&value.x.to_string())?;
    writer.end_element("X")?;

    writer.start_element("Y").finalize()?;
    writer.write_text(&value.y.to_string())?;
    writer.end_element("Y")?;

    writer.start_element("Z").finalize()?;
    writer.write_text(&value.z.to_string())?;
    writer.end_element("Z")?;
    Ok(())
}
