use std::io::Write;

use rbx_dom_weak::types::SerializedMap;

use crate::{
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    EncodeError,
};

pub const XML_TAG_NAME: &str = "BinaryString";

pub fn write_serialized_map<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &SerializedMap,
) -> Result<(), EncodeError> {
    let mut buffer = Vec::new();

    if let Err(write_error) = value.to_writer(&mut buffer) {
        return Err(writer.error(write_error));
    }

    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write_string(&base64::encode(&buffer))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}
