use std::io::Write;

use rbx_dom_weak::types::MaterialColors;

use crate::{
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    EncodeError,
};

pub const XML_TAG_NAME: &str = "BinaryString";

pub fn write_material_colors<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &MaterialColors,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write_string(&base64::encode(value.encode()))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}
