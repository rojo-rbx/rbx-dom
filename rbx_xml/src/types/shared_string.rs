//! SharedString values need extra state, similar to Refs.

use std::io::{Read, Write};

use rbx_dom_weak::types::{BinaryString, Ref, SharedString, Variant};

use crate::{
    deserializer::ParseState,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer::EmitState,
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

pub const XML_TAG_NAME: &str = "SharedString";

pub fn write_shared_string<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &SharedString,
    state: &mut EmitState,
) -> Result<(), EncodeError> {
    state.add_shared_string(value.clone());

    // Roblox expects SharedString hashes to be the same length as an MD5
    // hash: 16 bytes, so we truncate our larger hashes to fit.
    let full_hash = value.hash();
    let truncated_hash = &full_hash.as_bytes()[..16];

    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write_string(&base64::encode(truncated_hash))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn read_shared_string<R: Read>(
    reader: &mut XmlEventReader<R>,
    referent: Ref,
    property_name: &str,
    state: &mut ParseState,
) -> Result<Variant, DecodeError> {
    let contents = reader.read_tag_contents(XML_TAG_NAME)?;

    state.add_shared_string_rewrite(referent, property_name.into(), contents);

    // The value we actually pick here doesn't matter, it'll be overwritten
    // later.
    Ok(Variant::BinaryString(BinaryString::new()))
}
