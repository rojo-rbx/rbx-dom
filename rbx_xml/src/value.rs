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
        "Color3" => types::color3::deserialize(reader),
        "Color3uint8" => types::color3uint8::deserialize(reader),
        "Content" => types::content::deserialize(reader),
        types::CFrame::XML_NAME => types::CFrame::read_xml(reader),
        "double" => types::float64::deserialize(reader),
        "float" => types::float32::deserialize(reader),
        "int" => types::int32::deserialize(reader),
        "int64" => types::int64::deserialize(reader),
        "PhysicalProperties" => types::physical_properties::deserialize(reader),
        "ProtectedString" => types::protected_string::deserialize(reader),
        "Ref" => types::referent::deserialize(reader),
        "string" => types::string::deserialize(reader),
        "token" => types::enumeration::deserialize(reader),
        "UDim" => types::udim::deserialize(reader),
        types::UDim2::XML_NAME => types::UDim2::read_xml(reader),
        "Vector2" => types::vector2::deserialize(reader),
        "Vector2int16" => types::vector2int16::deserialize(reader),
        "Vector3" => types::vector3::deserialize(reader),
        "Vector3int16" => types::vector3int16::deserialize(reader),

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
        RbxValue::Color3 { value } => types::color3::serialize(writer, xml_name, *value),
        RbxValue::Color3uint8 { value } => types::color3uint8::serialize(writer, xml_name, *value),
        RbxValue::Content { value } => types::content::serialize(writer, xml_name, value),
        RbxValue::Enum { value } => types::enumeration::serialize(writer, xml_name, *value),
        RbxValue::Float32 { value } => types::float32::serialize(writer, xml_name, *value),
        RbxValue::Float64 { value } => types::float64::serialize(writer, xml_name, *value),
        RbxValue::Int32 { value } => types::int32::serialize(writer, xml_name, *value),
        RbxValue::Int64 { value } => types::int64::serialize(writer, xml_name, *value),
        RbxValue::PhysicalProperties { value } => types::physical_properties::serialize(writer, xml_name, *value),
        RbxValue::Ref { value } => types::referent::serialize(writer, xml_name, *value),
        RbxValue::String { value } => types::string::serialize(writer, xml_name, value),
        RbxValue::UDim { value } => types::udim::serialize(writer, xml_name, *value),
        RbxValue::UDim2 { value } => types::UDim2::write_xml(writer, xml_name, value),
        RbxValue::Vector2 { value } => types::vector2::serialize(writer, xml_name, *value),
        RbxValue::Vector2int16 { value } => types::vector2int16::serialize(writer, xml_name, *value),
        RbxValue::Vector3 { value } => types::vector3::serialize(writer, xml_name, *value),
        RbxValue::Vector3int16 { value } => types::vector3int16::serialize(writer, xml_name, *value),

        unknown => {
            warn!("Property value {:?} cannot be serialized yet", unknown);
            unimplemented!();
        },
    }
}