use std::io;

use rbx_dom_weak::types::Rect;

use super::{EncodeError, XmlWriter};

pub fn rect_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Rect,
) -> Result<(), EncodeError> {
    writer.write_rbx("min", value.min)?;
    writer.write_rbx("max", value.max)?;
    Ok(())
}
