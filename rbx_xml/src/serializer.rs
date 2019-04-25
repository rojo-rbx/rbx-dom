use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Write as FmtWrite,
    io::{self, Write},
};

use failure::Fail;
use xml::writer::{self, EventWriter, EmitterConfig};
use rbx_reflection::RbxPropertyType;
use rbx_dom_weak::{RbxTree, RbxValue, RbxValueType, RbxId};

use crate::{
    core::find_canonical_property_descriptor,
    types::{write_value_xml, write_ref},
};

pub use xml::writer::XmlEvent as XmlWriteEvent;

#[derive(Debug, Fail)]
pub enum EncodeError {
    #[fail(display = "IO Error: {}", _0)]
    IoError(#[fail(cause)] io::Error),

    #[fail(display = "XML error: {}", _0)]
    XmlError(#[fail(cause)] writer::Error),
}

impl From<xml::writer::Error> for EncodeError {
    fn from(error: xml::writer::Error) -> EncodeError {
        match error {
            xml::writer::Error::Io(inner) => EncodeError::IoError(inner),
            _ => EncodeError::XmlError(error),
        }
    }
}

/// Serialize the instances denoted by `ids` from `tree` as an XML-format model,
/// writing to `output`.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) -> Result<(), EncodeError> {
    let mut writer = XmlEventWriter::from_output(output);
    let mut state = EmitState::new();

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))?;

    for id in ids {
        serialize_instance(&mut writer, &mut state, tree, *id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

/// A wrapper around an xml-rs `EventWriter` as well as other state kept around
/// for performantly emitting XML.
pub struct XmlEventWriter<W> {
    inner: EventWriter<W>,
    character_buffer: String,
}

impl<W: Write> XmlEventWriter<W> {
    /// Constructs an `XmlEventWriter` from an output that implements `Write`.
    pub fn from_output(output: W) -> XmlEventWriter<W> {
        let inner = EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false)
            .normalize_empty_elements(false)
            .create_writer(output);

        XmlEventWriter {
            inner,
            character_buffer: String::new(),
        }
    }

    /// Writes a single XML event to the output stream.
    pub fn write<'a, E>(&mut self, event: E) -> Result<(), writer::Error>
        where E: Into<XmlWriteEvent<'a>>
    {
        self.inner.write(event)
    }

    /// Writes a string slice to the output stream as characters or CDATA.
    pub fn write_string(&mut self, value: &str) -> Result<(), writer::Error> {
        write_characters_or_cdata(&mut self.inner, value)
    }

    /// Writes a value that implements `Display` as characters or CDATA. Resuses
    /// an internal buffer to avoid unnecessary allocations.
    pub fn write_characters<T: std::fmt::Display>(&mut self, value: T) -> Result<(), writer::Error> {
        write!(self.character_buffer, "{}", value).unwrap();
        write_characters_or_cdata(&mut self.inner, &self.character_buffer)?;
        self.character_buffer.clear();

        Ok(())
    }

    /// The same as `write_characters`, but wraps the characters in a tag with
    /// the given name and no attributes.
    pub fn write_tag_characters<T: std::fmt::Display>(&mut self, tag: &str, value: T) -> Result<(), writer::Error> {
        self.write(XmlWriteEvent::start_element(tag))?;
        self.write_characters(value)?;
        self.write(XmlWriteEvent::end_element())
    }

    /// Writes a list of values that implement `Display`, with each wrapped in
    /// an associated tag. This method uses the same optimization as
    /// `write_characters` to avoid extra allocations.
    pub fn write_tag_array<T: std::fmt::Display>(&mut self, values: &[T], tags: &[&str]) -> Result<(), writer::Error> {
        assert_eq!(values.len(), tags.len());

        for (index, component) in values.iter().enumerate() {
            self.write_tag_characters(tags[index], component)?;
        }

        Ok(())
    }
}

/// Given a value, writes a `Characters` event or a `CData` event depending on
/// whether the input string contains whitespace that needs to be explicitly
/// preserved.
///
/// This method is extracted so that it can be used inside both `write_string`
/// and `write_characters` without borrowing issues.
fn write_characters_or_cdata<W: Write>(writer: &mut EventWriter<W>, value: &str) -> Result<(), writer::Error> {
    let first_char = value.chars().next();
    let last_char = value.chars().next_back();

    // If the string has leading or trailing whitespace, we switch to
    // writing it as part of a CDATA block instead of a regular characters
    // block.
    let has_outer_whitespace = match (first_char, last_char) {
        (Some(first), Some(last)) => first.is_whitespace() || last.is_whitespace(),
        (Some(char), None) | (None, Some(char)) => char.is_whitespace(),
        (None, None) => false,
    };

    if has_outer_whitespace {
        writer.write(XmlWriteEvent::cdata(value))?;
    } else {
        writer.write(XmlWriteEvent::characters(value))?;
    }

    Ok(())
}

pub struct EmitState {
    referent_map: HashMap<RbxId, u32>,
    next_referent: u32,
}

impl EmitState {
    pub fn new() -> EmitState {
        EmitState {
            referent_map: HashMap::new(),
            next_referent: 0,
        }
    }

    pub fn map_id(&mut self, id: RbxId) -> u32 {
        match self.referent_map.get(&id) {
            Some(&value) => value,
            None => {
                let referent = self.next_referent;
                self.referent_map.insert(id, referent);
                self.next_referent += 1;
                referent
            }
        }
    }
}

fn serialize_value<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    xml_name: &str,
    value: &RbxValue,
) -> Result<(), EncodeError> {
    // Refs need additional state that we don't want to thread through
    // `write_value_xml`, so we handle it here.
    match value {
        RbxValue::Ref { value: id } => write_ref(writer, xml_name, id, state),
        _ => write_value_xml(writer, xml_name, value)
    }
}

fn serialize_instance<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    tree: &RbxTree,
    id: RbxId,
) -> Result<(), EncodeError> {
    let instance = tree.get_instance(id).unwrap();
    let mapped_id = state.map_id(id);

    writer.write(XmlWriteEvent::start_element("Item")
        .attr("class", &instance.class_name)
        .attr("referent", &mapped_id.to_string()))?;

    writer.write(XmlWriteEvent::start_element("Properties"))?;

    serialize_value(writer, state, "Name", &RbxValue::String {
        value: instance.name.clone(),
    })?;

    for (property_name, value) in &instance.properties {
        if let Some(descriptor) = find_canonical_property_descriptor(&instance.class_name, property_name) {
            let serialized_name = descriptor.serialized_name()
                .unwrap_or(&property_name);

            let value_type = match descriptor.property_type() {
                RbxPropertyType::Data(value_type) => *value_type,
                RbxPropertyType::Enum(_enum_name) => RbxValueType::Enum,
                RbxPropertyType::UnimplementedType(_) => value.get_type(),
            };

            let converted_value = value.try_convert_ref(value_type)
                .unwrap_or(Cow::Borrowed(value));

            serialize_value(writer, state, &serialized_name, &converted_value)?;
        }
    }

    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, state, tree, *child_id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}