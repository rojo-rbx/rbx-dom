//! Wraps the reflection_types module from rbx_reflection and implements extra
//! methods its types.

use std::borrow::Cow;

use log::warn;
use rbx_dom_weak::RbxValueType;

use crate::api_dump::{ValueType, ValueCategory};

// The types this module exposes are defined in the rbx_reflection crate itself
// since they're supposed to line up 1:1 and we don't want them to get out of
// date.
//
// Normally, we'd want to use &'static str for strings in the generated code and
// String in our generator. We compromise and use Cow<'static, str> instead!
#[path = "../../rbx_reflection/src/reflection_types.rs"]
#[allow(unused)]
mod inner;

pub use inner::*;

impl RbxInstanceTags {
    pub fn from_dump_tags<T: AsRef<str>>(dump_tags: &[T]) -> RbxInstanceTags {
        let mut tags = RbxInstanceTags::empty();

        for dump_tag in dump_tags {
            let converted = match dump_tag.as_ref() {
                "Deprecated" => RbxInstanceTags::DEPRECATED,
                "NotBrowsable" => RbxInstanceTags::NOT_BROWSABLE,
                "NotCreatable" => RbxInstanceTags::NOT_CREATABLE,
                "NotReplicated" => RbxInstanceTags::NOT_REPLICATED,
                "PlayerReplicated" => RbxInstanceTags::PLAYER_REPLICATED,
                "Service" => RbxInstanceTags::SERVICE,
                "Settings" => RbxInstanceTags::SETTINGS,
                _ => {
                    warn!("Unknown instance flag {}", dump_tag.as_ref());
                    continue;
                }
            };

            tags |= converted;
        }

        tags
    }
}

impl RbxPropertyTags {
    pub fn from_dump_tags<T: AsRef<str>>(dump_tags: &[T]) -> RbxPropertyTags {
        let mut tags = RbxPropertyTags::empty();

        for dump_tag in dump_tags {
            let converted = match dump_tag.as_ref() {
                "Deprecated" =>  RbxPropertyTags::DEPRECATED,
                "Hidden" =>  RbxPropertyTags::HIDDEN,
                "NotBrowsable" =>  RbxPropertyTags::NOT_BROWSABLE,
                "NotReplicated" =>  RbxPropertyTags::NOT_REPLICATED,
                "NotScriptable" =>  RbxPropertyTags::NOT_SCRIPTABLE,
                "ReadOnly" =>  RbxPropertyTags::READ_ONLY,
                _ => {
                    warn!("Unknown instance flag {}", dump_tag.as_ref());
                    continue;
                }
            };

            tags |= converted;
        }

        tags
    }
}

impl<'a> From<&'a ValueType> for RbxPropertyTypeDescriptor {
    fn from(value_type: &'a ValueType) -> RbxPropertyTypeDescriptor {
        match value_type.category {
            ValueCategory::Primitive => {
                let data_kind = match value_type.name.as_str() {
                    "bool" => RbxValueType::Bool,
                    "string" => RbxValueType::String,
                    "int" => RbxValueType::Int32,
                    "float" => RbxValueType::Float32,
                    "double" => RbxValueType::Float64,
                    "int64" => RbxValueType::Int32,

                    unknown => {
                        println!("Can't emit primitives of type {}", unknown);

                        return RbxPropertyTypeDescriptor::UnimplementedType(Cow::Owned(value_type.name.to_owned()));
                    },
                };

                RbxPropertyTypeDescriptor::Data(data_kind)
            }
            ValueCategory::DataType => {
                let data_kind = match value_type.name.as_str() {
                    "Vector3" => RbxValueType::Vector3,
                    "Vector2" => RbxValueType::Vector2,
                    "Color3" => RbxValueType::Color3,
                    "CFrame" => RbxValueType::CFrame,
                    "PhysicalProperties" => RbxValueType::PhysicalProperties,
                    "BinaryString" => RbxValueType::BinaryString,
                    "UDim" => RbxValueType::UDim,
                    "UDim2" => RbxValueType::UDim2,
                    "Content" => RbxValueType::Content,
                    "NumberRange" => RbxValueType::NumberRange,
                    "NumberSequence" => RbxValueType::NumberSequence,
                    "ColorSequence" => RbxValueType::ColorSequence,
                    "ProtectedString" => RbxValueType::String,

                    "QDir" | "QFont" => {
                        // We're never going to support these types.
                        return RbxPropertyTypeDescriptor::UnimplementedType(Cow::Owned(value_type.name.to_owned()));
                    },

                    unknown => {
                        println!("Can't emit data of type {}", unknown);

                        return RbxPropertyTypeDescriptor::UnimplementedType(Cow::Owned(value_type.name.to_owned()));
                    },
                };

                RbxPropertyTypeDescriptor::Data(data_kind)
            }
            ValueCategory::Enum => RbxPropertyTypeDescriptor::Enum(Cow::Owned(value_type.name.to_owned())),
            ValueCategory::Class => RbxPropertyTypeDescriptor::Data(RbxValueType::Ref),
        }
    }
}