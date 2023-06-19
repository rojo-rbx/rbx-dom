use std::io;

use rbx_dom_weak::types::{Axes, Faces};

use super::{EncodeError, XmlWriter};

pub fn axes_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Axes,
) -> Result<(), EncodeError> {
    writer.write_element("axes", value.bits())
}

pub fn faces_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Faces,
) -> Result<(), EncodeError> {
    writer.write_element("faces", value.bits())
}
