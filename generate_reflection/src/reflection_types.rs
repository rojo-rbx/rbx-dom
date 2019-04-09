use std::borrow::Cow;

use rbx_dom_weak::RbxValueType;

use crate::api_dump::{ValueType, ValueCategory};

#[path = "../../rbx_reflection/src/types.rs"]
mod inner;

pub use inner::*;

impl<'a> From<&'a ValueType> for RbxPropertyType {
    fn from(value_type: &'a ValueType) -> RbxPropertyType {
        match value_type.category {
            ValueCategory::Primitive => {
                let data_kind = match value_type.name.as_str() {
                    "bool" => RbxValueType::Bool,
                    "string" => RbxValueType::String,
                    "int" => RbxValueType::Int32,
                    "float" => RbxValueType::Float32,

                    // These aren't quite right:
                    "double" => RbxValueType::Float32,
                    "int64" => RbxValueType::Int32,

                    unknown => {
                        println!("Can't emit primitives of type {}", unknown);

                        return RbxPropertyType::UnimplementedType(Cow::Owned(value_type.name.to_owned()));
                    },
                };

                RbxPropertyType::Data(data_kind)
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

                    unknown => {
                        println!("Can't emit data of type {}", unknown);

                        return RbxPropertyType::UnimplementedType(Cow::Owned(value_type.name.to_owned()));
                    },
                };

                RbxPropertyType::Data(data_kind)
            }
            ValueCategory::Enum => RbxPropertyType::Enum(Cow::Owned(value_type.name.to_owned())),
            ValueCategory::Class => RbxPropertyType::Data(RbxValueType::Ref),
        }
    }
}