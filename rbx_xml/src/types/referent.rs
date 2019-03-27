use std::io::{Read, Write};

use rbx_dom_weak::{RbxId, RbxValue};

use crate::{
    deserializer::{DecodeError, XmlEventReader, ParseState},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter, EmitState},
};

pub const XML_TAG_NAME: &'static str = "Ref";

pub fn write_ref<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &Option<RbxId>,
    state: &mut EmitState,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;

    match value {
        Some(id) => writer.write_characters(state.map_id(*id))?,
        None => writer.write(XmlWriteEvent::characters("null"))?
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn read_ref<R: Read>(
    reader: &mut XmlEventReader<R>,
    id: RbxId,
    property_name: &str,
    state: &mut ParseState,
) -> Result<RbxValue, DecodeError> {
    let ref_contents = reader.read_tag_contents(XML_TAG_NAME)?;

    if ref_contents != "null" {
        state.add_id_rewrite(id, property_name.to_owned(), ref_contents);
    }

    Ok(RbxValue::Ref {
        value: None,
    })
}