use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;
use log::warn;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlEventWriter},
    types,
};

macro_rules! rbx_types {
    ($($rbx_type: ident => $ser_type: ident),*) => {
        pub fn read_value_xml<R: Read>(
            reader: &mut EventIterator<R>,
            property_type: &str,
        ) -> Result<RbxValue, DecodeError> {
            match property_type {
                $(types::$ser_type::XML_NAME => types::$ser_type::read_xml(reader),)*

                // Protected string is only read, never written, so it's a special case.
                types::ProtectedString::XML_NAME => types::ProtectedString::read_xml(reader),

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
                $(RbxValue::$rbx_type { value } => types::$ser_type::write_xml(writer, xml_name, value),)*

                unknown => {
                    warn!("Property value {:?} cannot be serialized yet", unknown);
                    unimplemented!();
                },
            }
        }
    }
}

rbx_types!(
    BinaryString => BinaryString,
    Bool => Bool,
    CFrame => CFrame,
    Color3 => Color3,
    Color3uint8 => Color3uint8,
    Content => Content,
    Enum => Enumeration,
    Float32 => Float32,
    Float64 => Float64,
    Int32 => Int32,
    Int64 => Int64,
    PhysicalProperties => PhysicalProperties,
    Ref => Referent,
    String => String,
    UDim => UDim,
    UDim2 => UDim2,
    Vector2 => Vector2,
    Vector2int16 => Vector2int16,
    Vector3 => Vector3,
    Vector3int16 => Vector3int16
);