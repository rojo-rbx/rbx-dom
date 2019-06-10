//! SharedString values need extra state, similar to Refs.

use std::io::{Read, Write};

use rbx_dom_weak::{RbxId, RbxValue, SharedString};

use crate::{
    error::{EncodeError, DecodeError},
    deserializer_core::XmlEventReader,
    serializer_core::{XmlWriteEvent, XmlEventWriter},
    deserializer::ParseState,
    serializer::EmitState,
};

pub const XML_TAG_NAME: &'static str = "SharedString";

pub fn write_shared_string<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &SharedString,
    state: &mut EmitState,
) -> Result<(), EncodeError> {
    state.add_shared_string(value.clone());

    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write_string(&base64::encode(&value.md5_hash()))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn read_shared_string<R: Read>(
    reader: &mut XmlEventReader<R>,
    id: RbxId,
    property_name: &str,
    state: &mut ParseState,
) -> Result<RbxValue, DecodeError> {
    let contents = reader.read_tag_contents(XML_TAG_NAME)?;

    state.add_shared_string_rewrite(id, property_name.to_owned(), contents);

    // The value we actually pick here doesn't matter, it'll be overwritten
    // later.
    Ok(RbxValue::BinaryString {
        value: Vec::new(),
    })
}