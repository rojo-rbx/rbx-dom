use std::io::{self, Write};

use xml::writer::{EventWriter, EmitterConfig, XmlEvent};

use rbx_tree::{RbxTree, RbxValue, RbxId};

#[derive(Debug)]
pub enum EncodeError {
    IoError(io::Error),
    InternalXmlError,
}

impl From<xml::writer::Error> for EncodeError {
    fn from(error: xml::writer::Error) -> EncodeError {
        match error {
            xml::writer::Error::Io(inner) => EncodeError::IoError(inner),
            _ => EncodeError::InternalXmlError,
        }
    }
}

/// Serialize the instances denoted by `ids` from `tree` as an XML-format model,
/// writing to `output`.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) -> Result<(), EncodeError> {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false)
        .create_writer(output);

    writer.write(XmlEvent::start_element("roblox").attr("version", "4"))?;;

    for id in ids {
        serialize_instance(&mut writer, tree, *id)?;
    }

    writer.write(XmlEvent::end_element())?;

    Ok(())
}

fn serialize_value<W: Write>(writer: &mut EventWriter<W>, name: &str, value: &RbxValue) -> xml::writer::Result<()> {
    match value {
        RbxValue::String { value } => {
            writer.write(XmlEvent::start_element("string").attr("name", name))?;
            writer.write(XmlEvent::characters(&value))?;
            writer.write(XmlEvent::end_element())?;
        },
        RbxValue::Bool { value } => {
            writer.write(XmlEvent::start_element("bool").attr("name", name))?;

            let value_as_str = if *value {
                "true"
            } else {
                "false"
            };

            writer.write(XmlEvent::characters(value_as_str))?;
            writer.write(XmlEvent::end_element())?;
        },
        _ => unimplemented!(),
    }

    Ok(())
}

fn serialize_instance<W: Write>(writer: &mut EventWriter<W>, tree: &RbxTree, id: RbxId) -> xml::writer::Result<()> {
    let instance = tree.get_instance(id).unwrap();
    writer.write(XmlEvent::start_element("Item")
        .attr("class", &instance.class_name)
        .attr("referent", &instance.get_id().to_string()))?;

    writer.write(XmlEvent::start_element("Properties"))?;

    serialize_value(writer, "Name", &RbxValue::String {
        value: instance.name.clone(),
    })?;

    for (name, value) in &instance.properties {
        serialize_value(writer, name, value)?;
    }
    writer.write(XmlEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, tree, *child_id)?;
    }

    writer.write(XmlEvent::end_element())?;

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