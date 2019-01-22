use std::{
    io::{Read, Write},
};

use xml::{
    writer::{XmlEvent as XmlWriteEvent, EventWriter},
    reader::{XmlEvent as XmlReadEvent},
};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::EncodeError,
};

pub fn serialize_string<W: Write>(writer: &mut EventWriter<W>, name: &str, value: &str) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("string").attr("name", name))?;
    writer.write(XmlWriteEvent::characters(&value))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_string<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    read_event!(reader, XmlReadEvent::Characters(value) => Ok(RbxValue::String { value: value.to_owned() }))
}