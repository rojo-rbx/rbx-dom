//! Referents are strange compared to other property types.
//!
//! Refs require extra information, which is why they aren't part of the
//! `XmlType`-implementing family of structs like other types. I think this is
//! a better approach than widening the values that `XmlType` accepts just for
//! this type.
//!
//! Specifically, deserializing refs needs access to a special list of
//! 'rewrites'. It's used as part of a second pass to make sure that refs
//! pointing to instances that we haven't reached yet work okay.

use std::io::{Read, Write};

use rbx_dom_weak::{RbxId, RbxValue};

use crate::{
    deserializer::ParseState,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer::EmitState,
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

pub const XML_TAG_NAME: &'static str = "Ref";

pub fn write_ref<W: Write>(
    writer: &mut XmlEventWriter<W>,
    xml_property_name: &str,
    value: &Option<RbxId>,
    state: &mut EmitState,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", xml_property_name))?;

    match value {
        Some(id) => writer.write_characters(state.map_id(*id))?,
        None => writer.write(XmlWriteEvent::characters("null"))?,
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
        // We need to rewrite this property as part of a follow-up pass.
        //
        // We might not know which ID this referent points to yet, so instead of
        // trying to handle the case where we do here, we just let all referents
        // get written later.
        state.add_referent_rewrite(id, property_name.to_owned(), ref_contents);
    }

    Ok(RbxValue::Ref { value: None })
}
