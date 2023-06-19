use std::io;

use rbx_dom_weak::types::Ray;

use super::XmlWriter;
use crate::EncodeError;

pub fn ray_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Ray,
) -> Result<(), EncodeError> {
    writer.write_rbx("origin", value.origin)?;
    writer.write_rbx("direction", value.direction)?;

    Ok(())
}
