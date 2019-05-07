use std::{
    borrow::Cow,
    collections::HashMap,
    io::Write,
};

use rbx_reflection::RbxPropertyTypeDescriptor;
use rbx_dom_weak::{RbxTree, RbxValue, RbxValueType, RbxId, RbxValueConversion};

use crate::{
    core::find_canonical_property_descriptor,
    types::{write_value_xml, write_ref},
    error::EncodeError as NewEncodeError,
};

pub use crate::serializer_core::*;

/// Serialize the instances denoted by `ids` from `tree` as an XML-format model,
/// writing to `output`.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) -> Result<(), NewEncodeError> {
    let mut writer = XmlEventWriter::from_output(output);
    let mut state = EmitState::new();

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))
        .map_err(|e| writer.error(e))?;

    for id in ids {
        serialize_instance(&mut writer, &mut state, tree, *id)?;
    }

    writer.write(XmlWriteEvent::end_element())
        .map_err(|e| writer.error(e))?;

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
                RbxPropertyTypeDescriptor::Data(value_type) => *value_type,
                RbxPropertyTypeDescriptor::Enum(_enum_name) => RbxValueType::Enum,
                RbxPropertyTypeDescriptor::UnimplementedType(_) => value.get_type(),
            };

            let converted_value = match value.try_convert_ref(value_type) {
                RbxValueConversion::Converted(converted) => Cow::Owned(converted),
                RbxValueConversion::Unnecessary | RbxValueConversion::Failed => Cow::Borrowed(value),
            };

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