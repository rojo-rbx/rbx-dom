/// Super early XML format (rbxmx and rbxlx) serializer for rbx-tree.

extern crate rbx_tree;
extern crate xml;

use std::io::Write;

use xml::writer::{EventWriter, EmitterConfig, XmlEvent};

use rbx_tree::{RbxTree, RbxValue, RbxId};

fn serialize_value<W: Write>(writer: &mut EventWriter<W>, name: &str, value: &RbxValue) {
    match value {
        RbxValue::String { value } => {
            writer.write(XmlEvent::start_element("string").attr("name", name)).unwrap();
            writer.write(XmlEvent::characters(&value)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        },
        RbxValue::Bool { value } => {
            writer.write(XmlEvent::start_element("bool").attr("name", name)).unwrap();

            let value_as_str = if value {
                "true"
            } else {
                "false"
            };

            writer.write(XmlEvent::characters(value_as_str)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap();
        },
        _ => unimplemented!(),
    }
}

fn serialize_instance<W: Write>(writer: &mut EventWriter<W>, tree: &RbxTree, id: RbxId) {
    let instance = tree.get_instance(id).unwrap();
    writer.write(XmlEvent::start_element("Item")
        .attr("class", &instance.class_name)
        .attr("referent", &instance.get_id().to_string())).unwrap();

    writer.write(XmlEvent::start_element("Properties")).unwrap();
    for (name, value) in &instance.properties {
        serialize_value(writer, name, value);
    }
    writer.write(XmlEvent::end_element()).unwrap();

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, tree, *child_id);
    }

    writer.write(XmlEvent::end_element()).unwrap();
}

pub enum RbxmxParseError {
}

pub fn decode(_source: &[u8]) -> Result<RbxTree, RbxmxParseError> {
    unimplemented!()
}

pub fn encode(tree: &RbxTree) -> Vec<u8> {
    let mut output = Vec::new();

    {
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut output);

        writer.write(XmlEvent::start_element("roblox").attr("version", "4")).unwrap();

        serialize_instance(&mut writer, tree, tree.get_root_id());

        writer.write(XmlEvent::end_element()).unwrap();
    }

    output
}

#[cfg(test)]
mod test {
    use super::encode;

    use std::collections::HashMap;
    use std::str;

    use rbx_tree::{RbxTree, RbxInstance, RbxValue};

    #[test]
    fn serialize() {
        let mut properties = HashMap::new();
        properties.insert("SomethingEnabled".to_string(), RbxValue::String {
            value: "Yes Please".to_string(),
        });

        let root_instance = RbxInstance {
            name: "DataModel".to_string(),
            class_name: "DataModel".to_string(),
            properties,
        };

        let mut child_properties = HashMap::new();
        child_properties.insert("StreamingEnabled".to_string(), RbxValue::Bool {
            value: true,
        });

        let child = RbxInstance {
            name: "Workspace".to_string(),
            class_name: "Workspace".to_string(),
            properties: child_properties,
        };

        let mut tree = RbxTree::new(root_instance);
        let root_id = tree.get_root_id();
        tree.insert_instance(child, root_id);

        let encoded = encode(&tree);
        let as_str = str::from_utf8(&encoded).unwrap();

        // TODO: Serialize/deserialize and assert output?
    }
}