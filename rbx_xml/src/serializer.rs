use std::io::{self, Write};
use std::fmt::Write as FmtWrite;

use log::warn;
use failure::Fail;
use xml::writer::{self, EventWriter, EmitterConfig};
use rbx_tree::{RbxTree, RbxValue, RbxId};

use crate::{
    reflection::CANONICAL_TO_XML_NAME,
    types::{
        serialize_binary_string,
        serialize_bool,
        serialize_cframe,
        serialize_color3,
        serialize_color3uint8,
        serialize_enum,
        serialize_float32,
        serialize_int32,
        serialize_physical_properties,
        serialize_string,
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

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))?;

    for id in ids {
        serialize_instance(&mut writer, tree, *id)?;
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

fn serialize_value<W: Write>(
    writer: &mut XmlEventWriter<W>,
    canonical_name: &str,
    value: &RbxValue,
) -> Result<(), EncodeError> {
    let xml_name = CANONICAL_TO_XML_NAME
        .get(canonical_name)
        .unwrap_or(&canonical_name);

    match value {
        RbxValue::Bool { value } => serialize_bool(writer, xml_name, *value),
        RbxValue::String { value } => serialize_string(writer, xml_name, value),
        RbxValue::BinaryString { value } => serialize_binary_string(writer, xml_name, value),
        RbxValue::Vector2 { value } => serialize_vector2(writer, xml_name, *value),
        RbxValue::Vector3 { value } => serialize_vector3(writer, xml_name, *value),
        RbxValue::Vector2int16 { value } => serialize_vector2int16(writer, xml_name, *value),
        RbxValue::Vector3int16 { value } => serialize_vector3int16(writer, xml_name, *value),
        RbxValue::Float32 { value } => serialize_float32(writer, xml_name, *value),
        RbxValue::Int32 { value } => serialize_int32(writer, xml_name, *value),
        RbxValue::Enum { value } => serialize_enum(writer, xml_name, *value),
        RbxValue::PhysicalProperties { value } => serialize_physical_properties(writer, xml_name, *value),
        RbxValue::CFrame { value } => serialize_cframe(writer, xml_name, *value),
        RbxValue::Color3 { value } => serialize_color3(writer, xml_name, *value),
        RbxValue::Color3uint8 { value } => serialize_color3uint8(writer, xml_name, *value),
        unknown => {
            warn!("Property value {:?} cannot be serialized yet", unknown);
            unimplemented!();
        },
    }
}

fn serialize_instance<W: Write>(writer: &mut XmlEventWriter<W>, tree: &RbxTree, id: RbxId) -> Result<(), EncodeError> {
    let instance = tree.get_instance(id).unwrap();
    writer.write(XmlWriteEvent::start_element("Item")
        .attr("class", &instance.class_name)
        .attr("referent", &instance.get_id().to_string()))?;

    writer.write(XmlWriteEvent::start_element("Properties"))?;

    serialize_value(writer, "Name", &RbxValue::String {
        value: instance.name.clone(),
    })?;

    for (name, value) in &instance.properties {
        serialize_value(writer, name, value)?;
    }
    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, tree, *child_id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::encode;

    use std::collections::HashMap;
    use std::str;

    use rbx_tree::{RbxTree, RbxInstanceProperties, RbxValue};

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