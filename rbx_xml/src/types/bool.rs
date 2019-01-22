use std::io::{Read, Write};

use xml::{
    writer::{XmlEvent as XmlWriteEvent, EventWriter},
    reader::{XmlEvent as XmlReadEvent},
};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::EncodeError,
};

pub fn serialize_bool<W: Write>(writer: &mut EventWriter<W>, name: &str, value: bool) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("bool").attr("name", name))?;

    let value_as_str = if value {
        "true"
    } else {
        "false"
    };

    writer.write(XmlWriteEvent::characters(value_as_str))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_bool<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let value = read_event!(reader, XmlReadEvent::Characters(content) => {
        match content.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(DecodeError::Message("invalid boolean value, expected true or false")),
        }
    });

    Ok(RbxValue::Bool {
        value
    })
}