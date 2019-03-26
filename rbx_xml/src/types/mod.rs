//! This file packs up all of the type implementations in rbx_xml and exposes
//! them through two methods, `read_value_xml` and `write_value_xml`.
//!
//! To support a new type in rbx_xml:
//!
//! 1. Implement the type in a submodule (with tests!)
//! 2. Add a 'mod' statement immediately below this comment
//! 3. Add the type(s) to the declare_rbx_types! macro invocation

mod binary_string;
mod bool;
mod cframe;
mod colors;
mod content;
mod enumeration;
mod numbers;
mod physical_properties;
mod rect;
mod referent;
mod strings;
mod udims;
mod vectors;
mod number_range;

use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;
use log::warn;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlEventWriter},
};

/// The `declare_rbx_types` macro generates the two big match statements that
/// rbx_xml uses to read/write values inside of `read_value_xml` and
/// `write_value_xml`.
macro_rules! declare_rbx_types {
    { $($typedef: path => $rbx_type: ident),* } => {

        /// Reads a Roblox property value with the given type from the XML event
        /// stream.
        pub fn read_value_xml<R: Read>(
            reader: &mut XmlEventReader<R>,
            xml_type_name: &str,
        ) -> Result<RbxValue, DecodeError> {
            match xml_type_name {
                $(<$typedef>::XML_TAG_NAME => <$typedef>::read_xml(reader),)*

                // Protected strings are only read, never written
                self::strings::ProtectedStringType::XML_TAG_NAME => self::strings::ProtectedStringType::read_xml(reader),

                unknown => {
                    warn!("Properties of type {:?} cannot be deserialized yet", unknown);
                    Err(DecodeError::Message("Can't decode properties of this type yet"))
                },
            }
        }

        /// Writes a Roblox property value with the given XML name to the XML
        /// stream.
        pub fn write_value_xml<W: Write>(
            writer: &mut XmlEventWriter<W>,
            xml_property_name: &str,
            value: &RbxValue,
        ) -> Result<(), EncodeError> {
            match value {
                $(RbxValue::$rbx_type { value } => <$typedef>::write_xml(writer, xml_property_name, value),)*

                unknown => {
                    warn!("Property value {:?} cannot be serialized yet", unknown);
                    unimplemented!();
                },
            }
        }
    }
}

declare_rbx_types! {
    self::binary_string::BinaryStringType => BinaryString,
    self::bool::BoolType => Bool,
    self::cframe::CFrameType => CFrame,
    self::colors::Color3Type => Color3,
    self::colors::Color3uint8Type => Color3uint8,
    self::content::ContentType => Content,
    self::enumeration::EnumType => Enum,
    self::number_range::NumberRangeType => NumberRange,
    self::numbers::Float32Type => Float32,
    self::numbers::Float64Type => Float64,
    self::numbers::Int32Type => Int32,
    self::numbers::Int64Type => Int64,
    self::physical_properties::PhysicalPropertiesType => PhysicalProperties,
    self::rect::RectType => Rect,
    self::referent::RefType => Ref,
    self::strings::StringType => String,
    self::udims::UDim2Type => UDim2,
    self::udims::UDimType => UDim,
    self::vectors::Vector2Type => Vector2,
    self::vectors::Vector2int16Type => Vector2int16,
    self::vectors::Vector3Type => Vector3,
    self::vectors::Vector3int16Type => Vector3int16
}