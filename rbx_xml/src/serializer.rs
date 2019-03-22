use std::{
    collections::HashMap,
    io::{self, Write},
    fmt::Write as FmtWrite,
};

use log::warn;
use failure::Fail;
use xml::writer::{self, EventWriter, EmitterConfig};
use rbx_dom_weak::{RbxTree, RbxValue, RbxId};

use crate::{
    reflection::CANONICAL_TO_XML_NAME,
    types::{
        float32,
        float64,
        int32,
        int64,
        serialize_binary_string,
        serialize_bool,
        serialize_cframe,
        serialize_color3,
        serialize_color3uint8,
        serialize_content,
        serialize_enum,
        serialize_physical_properties,
        serialize_ref,
        serialize_string,
        serialize_udim,
        serialize_udim2,
        serialize_vector2,
        serialize_vector2int16,
        serialize_vector3,
        serialize_vector3int16,
    },
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

pub struct XmlEventWriter<W> {
    inner: EventWriter<W>,
    character_buffer: String,
}

impl<W: Write> XmlEventWriter<W> {
    pub fn from_output(output: W) -> XmlEventWriter<W> {
        let inner = EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false)
            .create_writer(output);

        XmlEventWriter {
            inner,
            character_buffer: String::new(),
        }
    }

    pub fn write<'a, E>(&mut self, event: E) -> Result<(), writer::Error>
        where E: Into<XmlWriteEvent<'a>>
    {
        self.inner.write(event)
    }

    /// A more efficient implementation to write characters to the XML output
    /// stream that reuses a buffer for each string.
    pub fn write_characters<T: std::fmt::Display>(&mut self, value: T) -> Result<(), writer::Error> {
        write!(self.character_buffer, "{}", value).unwrap();
        self.inner.write(XmlWriteEvent::characters(&self.character_buffer))?;
        self.character_buffer.clear();

        Ok(())
    }

    pub fn write_tag_characters<T: std::fmt::Display>(&mut self, tag: &str, value: T) -> Result<(), writer::Error> {
        self.write(XmlWriteEvent::start_element(tag))?;
        self.write_characters(value)?;
        self.write(XmlWriteEvent::end_element())
    }

    pub fn write_tag_array<T: std::fmt::Display>(&mut self, values: &[T], tags: &[&str]) -> Result<(), writer::Error> {
        assert_eq!(values.len(), tags.len());

        for (index, component) in values.iter().enumerate() {
            self.write(XmlWriteEvent::start_element(tags[index]))?;
            self.write_characters(component)?;
            self.write(XmlWriteEvent::end_element())?;
        }

        Ok(())
    }
}

struct EmitState {
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
                self.next_referent += 1;
                self.referent_map.insert(id, referent);
                referent
            }
        }
    }
}

fn serialize_value<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    canonical_name: &str,
    value: &RbxValue,
) -> Result<(), EncodeError> {
    let xml_name = CANONICAL_TO_XML_NAME
        .get(canonical_name)
        .unwrap_or(&canonical_name);

    match value {
        RbxValue::BinaryString { value } => serialize_binary_string(writer, xml_name, value),
        RbxValue::Bool { value } => serialize_bool(writer, xml_name, *value),
        RbxValue::CFrame { value } => serialize_cframe(writer, xml_name, *value),
        RbxValue::Color3 { value } => serialize_color3(writer, xml_name, *value),
        RbxValue::Color3uint8 { value } => serialize_color3uint8(writer, xml_name, *value),
        RbxValue::Content { value } => serialize_content(writer, xml_name, value),
        RbxValue::Enum { value } => serialize_enum(writer, xml_name, *value),
        RbxValue::Float32 { value } => float32::serialize(writer, xml_name, *value),
        RbxValue::Float64 { value } => float64::serialize(writer, xml_name, *value),
        RbxValue::Int32 { value } => int32::serialize(writer, xml_name, *value),
        RbxValue::Int64 { value } => int64::serialize(writer, xml_name, *value),
        RbxValue::PhysicalProperties { value } => serialize_physical_properties(writer, xml_name, *value),
        RbxValue::Ref { value } => serialize_ref(writer, xml_name, *value),
        RbxValue::String { value } => serialize_string(writer, xml_name, value),
        RbxValue::UDim { value } => serialize_udim(writer, xml_name, *value),
        RbxValue::UDim2 { value } => serialize_udim2(writer, xml_name, *value),
        RbxValue::Vector2 { value } => serialize_vector2(writer, xml_name, *value),
        RbxValue::Vector2int16 { value } => serialize_vector2int16(writer, xml_name, *value),
        RbxValue::Vector3 { value } => serialize_vector3(writer, xml_name, *value),
        RbxValue::Vector3int16 { value } => serialize_vector3int16(writer, xml_name, *value),

        unknown => {
            warn!("Property value {:?} cannot be serialized yet", unknown);
            unimplemented!();
        },
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

    for (name, value) in &instance.properties {
        serialize_value(writer, state, name, value)?;
    }
    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, state, tree, *child_id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::encode;

    use std::collections::HashMap;
    use std::str;

    use rbx_dom_weak::{RbxTree, RbxInstanceProperties, RbxValue};

    #[test]
    fn serialize() {
        let _ = env_logger::try_init();

        let mut properties = HashMap::new();
        properties.insert("SomethingEnabled".to_string(), RbxValue::String {
            value: "Yes Please".to_string(),
        });

        let root_instance = RbxInstanceProperties {
            name: "DataModel".to_string(),
            class_name: "DataModel".to_string(),
            properties,
        };

        let mut child_properties = HashMap::new();
        child_properties.insert("StreamingEnabled".to_string(), RbxValue::Bool {
            value: true,
        });

        let child = RbxInstanceProperties {
            name: "Workspace".to_string(),
            class_name: "Workspace".to_string(),
            properties: child_properties,
        };

        let mut tree = RbxTree::new(root_instance);
        let root_id = tree.get_root_id();
        tree.insert_instance(child, root_id);

        let root = tree.get_instance(root_id).unwrap();

        let mut output = Vec::new();
        encode(&tree, &root.get_children_ids(), &mut output).unwrap();
        let _as_str = str::from_utf8(&output).unwrap();

        // TODO: Serialize/deserialize and assert output?
    }
}