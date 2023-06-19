//! Serializers for binary string types like `Tags` and `Attributes`

use std::io;

use rbx_dom_weak::types::{Attributes, Tags};

use super::XmlWriter;
use crate::EncodeError;

pub fn tags_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Tags,
) -> Result<(), EncodeError> {
    writer.write_base64(&value.encode())?;
    Ok(())
}

pub fn attributes_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Attributes,
) -> Result<(), EncodeError> {
    let mut attributes = Vec::new();
    value.to_writer(&mut attributes)?;
    writer.write_base64(&attributes)?;
    Ok(())
}
