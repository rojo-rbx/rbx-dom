use std::io;

use rbx_dom_weak::types::Ray;

use super::vector3_serializer;
use super::XmlWriter;
use crate::serializer::error::EncodeError;

pub fn ray_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Ray,
) -> Result<(), EncodeError> {
    writer.start_element("origin").finalize()?;
    vector3_serializer(writer, &value.origin)?;
    writer.end_element("origin")?;

    writer.start_element("direction").finalize()?;
    vector3_serializer(writer, &value.direction)?;
    writer.end_element("direction")?;

    Ok(())
}
