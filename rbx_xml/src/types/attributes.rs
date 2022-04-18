use std::io::Write;
use rbx_dom_weak::types::Attributes;

use crate::{
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    error::{EncodeError, EncodeErrorKind},
};

pub const XML_TAG_NAME: &str = "BinaryString";

pub fn write_attributes<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &Attributes,
) -> Result<(), EncodeError> {
    let mut encoded = Vec::new();
    let written = value.to_writer(&mut encoded);

    if written.is_err() {
        let err = written.unwrap_err();
        return Err(writer.error(EncodeErrorKind::TypeError(err)));
    }

    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write(XmlWriteEvent::cdata(&base64::encode(&encoded)))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}
