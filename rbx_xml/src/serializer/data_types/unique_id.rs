use std::io;

use super::{EncodeError, XmlWriter};
use rbx_dom_weak::types::UniqueId;

pub fn unique_id_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &UniqueId,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_string())?;
    Ok(())
}
