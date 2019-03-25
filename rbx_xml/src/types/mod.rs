//! This file packs up all of the type implementations in rbx_xml and exposes
//! them through two methods, `read_value_xml` and `write_value_xml`.
//!
//! To support a new type in rbx_xml:
//!
//! 1. Implement the type in a submodule (with tests!)
//! 2. Add a 'mod' statement immediately below this comment
//! 3. Add the mod to the 'use' statement below the macro definition
//! 4. Add the type(s) to the rbx_types! macro invocation

mod binary_string;
mod bool;
mod cframe;
mod colors;
mod content;
mod enumeration;
mod numbers;
mod physical_properties;
mod referent;
mod strings;
mod udims;
mod vectors;

use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;
use log::warn;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlEventWriter},
    types,
};

/// The `rbx_types` macro generates the two big match statements that rbx_xml
/// uses to read/write values inside of `read_value_xml` and `write_value_xml`.
macro_rules! rbx_types {
    [$($rbx_type: ident),*] => {
        pub fn read_value_xml<R: Read>(
            reader: &mut EventIterator<R>,
            property_type: &str,
        ) -> Result<RbxValue, DecodeError> {
            match property_type {
                $(types::$rbx_type::XML_NAME => types::$rbx_type::read_xml(reader),)*

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
                $(RbxValue::$rbx_type { value } => types::$rbx_type::write_xml(writer, xml_name, value),)*

                unknown => {
                    warn!("Property value {:?} cannot be serialized yet", unknown);
                    unimplemented!();
                },
            }
        }
    }
}

use self::{
    binary_string::BinaryString,
    bool::Bool,
    cframe::CFrame,
    colors::{Color3, Color3uint8},
    content::Content,
    enumeration::Enum,
    numbers::{Float32, Float64, Int32, Int64},
    physical_properties::PhysicalProperties,
    referent::Ref,
    strings::{String, ProtectedString},
    udims::{UDim, UDim2},
    vectors::{Vector2, Vector2int16, Vector3, Vector3int16},
};

rbx_types![
    BinaryString,
    Bool,
    CFrame,
    Color3,
    Color3uint8,
    Content,
    Enum,
    Float32,
    Float64,
    Int32,
    Int64,
    PhysicalProperties,
    Ref,
    String,
    UDim,
    UDim2,
    Vector2,
    Vector2int16,
    Vector3,
    Vector3int16
];