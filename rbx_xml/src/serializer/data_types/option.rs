use std::io;

use rbx_dom_weak::types::CFrame;

use super::{cframe_serializer, EncodeError, XmlWriter};

pub fn optional_cframe_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Option<CFrame>,
) -> Result<(), EncodeError> {
    if let Some(cframe) = value {
        // I'm unwilling to copy `cframe` for this.
        writer.start_element("CFrame").finalize()?;
        cframe_serializer(writer, cframe)?;
        writer.end_element("CFrame")?;
    }
    Ok(())
}
