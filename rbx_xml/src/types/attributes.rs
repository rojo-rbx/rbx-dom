use std::io::Write;

use rbx_dom_weak::types::Attributes;

use crate::{
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    EncodeError,
};

pub const XML_TAG_NAME: &str = "BinaryString";

pub fn write_attributes<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &Attributes,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    // TODO: Write attributes.
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}
