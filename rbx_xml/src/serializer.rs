use std::{
    borrow::Cow,
    collections::HashMap,
    io::Write,
};

use rbx_reflection::RbxPropertyTypeDescriptor;
use rbx_dom_weak::{RbxTree, RbxValue, RbxValueType, RbxId, RbxValueConversion};

use crate::{
    core::find_serialized_property_descriptor,
    types::{write_value_xml, write_ref},
    error::EncodeError as NewEncodeError,
};

use crate::serializer_core::{XmlEventWriter, XmlWriteEvent};

pub fn encode_internal<W: Write>(output: W, tree: &RbxTree, ids: &[RbxId], _options: EncodeOptions) -> Result<(), NewEncodeError> {
    let mut writer = XmlEventWriter::from_output(output);
    let mut state = EmitState::new();

    writer.write(XmlWriteEvent::start_element("roblox").attr("version", "4"))?;

    for id in ids {
        serialize_instance(&mut writer, &mut state, tree, *id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

/// Options available for serializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct EncodeOptions {
    use_reflection: bool,
}

impl EncodeOptions {
    /// Constructs a `EncodeOptions` with all values set to their defaults.
    pub fn new() -> EncodeOptions {
        EncodeOptions {
            use_reflection: true,
        }
    }

    /// Enabled by default.
    ///
    /// Sets whether to use the reflection database to canonicalize fields and
    /// value types.
    ///
    /// If disabled, properties will be written as-is to the disk. Files
    /// produced this way will probably not be compatible with Roblox unless
    /// they were read with the same option in `DecodeOptions`.
    // TODO: Make this public once this setting actually does anything.
    #[allow(unused)]
    fn use_reflection(self, use_reflection: bool) -> EncodeOptions {
        EncodeOptions {
            use_reflection,
            ..self
        }
    }
}

impl Default for EncodeOptions {
    fn default() -> EncodeOptions {
        EncodeOptions::new()
    }
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
) -> Result<(), NewEncodeError> {
    // Refs need additional state that we don't want to thread through
    // `write_value_xml`, so we handle it here.
    match value {
        RbxValue::Ref { value: id } => write_ref(writer, xml_name, id, state).map_err(Into::into),
        _ => write_value_xml(writer, xml_name, value)
    }
}

fn serialize_instance<W: Write>(
    writer: &mut XmlEventWriter<W>,
    state: &mut EmitState,
    tree: &RbxTree,
    id: RbxId,
) -> Result<(), NewEncodeError> {
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
        if let Some(serialized_descriptor) = find_serialized_property_descriptor(&instance.class_name, property_name) {
            let value_type = match serialized_descriptor.property_type() {
                RbxPropertyTypeDescriptor::Data(value_type) => *value_type,
                RbxPropertyTypeDescriptor::Enum(_enum_name) => RbxValueType::Enum,
                RbxPropertyTypeDescriptor::UnimplementedType(_) => value.get_type(),
            };

            let converted_value = match value.try_convert_ref(value_type) {
                RbxValueConversion::Converted(converted) => Cow::Owned(converted),
                RbxValueConversion::Unnecessary | RbxValueConversion::Failed => Cow::Borrowed(value),
            };

            serialize_value(writer, state, &serialized_descriptor.name(), &converted_value)?;
        }
    }

    writer.write(XmlWriteEvent::end_element())?;

    for child_id in instance.get_children_ids() {
        serialize_instance(writer, state, tree, *child_id)?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}