use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;
use log::warn;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlEventWriter},
    types,
};

pub fn read_value_xml<R: Read>(
    reader: &mut EventIterator<R>,
    property_type: &str,
) -> Result<RbxValue, DecodeError> {
    match property_type {
        types::BinaryString::XML_NAME => types::BinaryString::read_xml(reader),
        types::Bool::XML_NAME => types::Bool::read_xml(reader),
        types::Color3::XML_NAME => types::Color3::read_xml(reader),
        types::Color3uint8::XML_NAME => types::Color3uint8::read_xml(reader),
        "Content" => types::content::deserialize(reader),
        types::CFrame::XML_NAME => types::CFrame::read_xml(reader),
        "double" => types::float64::deserialize(reader),
        "float" => types::float32::deserialize(reader),
        "int" => types::int32::deserialize(reader),
        "int64" => types::int64::deserialize(reader),
        "PhysicalProperties" => types::physical_properties::deserialize(reader),
        "ProtectedString" => types::protected_string::deserialize(reader),
        "Ref" => types::referent::deserialize(reader),
        types::String::XML_NAME => types::String::read_xml(reader),
        "token" => types::enumeration::deserialize(reader),
        types::UDim::XML_NAME => types::UDim::read_xml(reader),
        types::UDim2::XML_NAME => types::UDim2::read_xml(reader),
        types::Vector2::XML_NAME => types::Vector2::read_xml(reader),
        types::Vector2int16::XML_NAME => types::Vector2int16::read_xml(reader),
        types::Vector3::XML_NAME => types::Vector3::read_xml(reader),
        types::Vector3int16::XML_NAME => types::Vector3int16::read_xml(reader),

        unknown => {
            warn!("Properties of type {:?} cannot be deserialized yet", unknown);
            Err(DecodeError::Message("Can't decode properties of this type yet"))
        },
    }
}

pub fn write_value_xml<W: Write>(
    writer: &mut XmlEventWriter<W>,
    xml_name: &str,
    value: &RbxValue,
) -> Result<(), EncodeError> {
    match value {
        RbxValue::BinaryString { value } => types::BinaryString::write_xml(writer, xml_name, value),
        RbxValue::Bool { value } => types::Bool::write_xml(writer, xml_name, value),
        RbxValue::CFrame { value } => types::CFrame::write_xml(writer, xml_name, value),
        RbxValue::Color3 { value } => types::Color3::write_xml(writer, xml_name, value),
        RbxValue::Color3uint8 { value } => types::Color3uint8::write_xml(writer, xml_name, value),
        RbxValue::Content { value } => types::content::serialize(writer, xml_name, value),
        RbxValue::Enum { value } => types::enumeration::serialize(writer, xml_name, *value),
        RbxValue::Float32 { value } => types::float32::serialize(writer, xml_name, *value),
        RbxValue::Float64 { value } => types::float64::serialize(writer, xml_name, *value),
        RbxValue::Int32 { value } => types::int32::serialize(writer, xml_name, *value),
        RbxValue::Int64 { value } => types::int64::serialize(writer, xml_name, *value),
        RbxValue::PhysicalProperties { value } => types::physical_properties::serialize(writer, xml_name, *value),
        RbxValue::Ref { value } => types::referent::serialize(writer, xml_name, *value),
        RbxValue::String { value } => types::String::write_xml(writer, xml_name, value),
        RbxValue::UDim { value } => types::UDim::write_xml(writer, xml_name, value),
        RbxValue::UDim2 { value } => types::UDim2::write_xml(writer, xml_name, value),
        RbxValue::Vector2 { value } => types::Vector2::write_xml(writer, xml_name, value),
        RbxValue::Vector2int16 { value } => types::Vector2int16::write_xml(writer, xml_name, value),
        RbxValue::Vector3 { value } => types::Vector3::write_xml(writer, xml_name, value),
        RbxValue::Vector3int16 { value } => types::Vector3int16::write_xml(writer, xml_name, value),

        unknown => {
            warn!("Property value {:?} cannot be serialized yet", unknown);
            unimplemented!();
        },
    }
}