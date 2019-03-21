use rbx_dom_weak::RbxValueType;

use crate::api_dump::{ValueType, ValueCategory};

#[derive(Debug, Clone, PartialEq)]
pub enum RbxPropertyType<'a> {
    Data(RbxValueType),
    Enum(&'a str),

    UnimplementedType(&'a str),
}

impl<'a> From<&'a ValueType> for RbxPropertyType<'a> {
    fn from(value_type: &'a ValueType) -> RbxPropertyType<'a> {
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

                        return RbxPropertyType::UnimplementedType(&value_type.name);
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

                        return RbxPropertyType::UnimplementedType(&value_type.name);
                    },
                };

                RbxPropertyType::Data(data_kind)
            }
            ValueCategory::Enum => RbxPropertyType::Enum(&value_type.name),
            ValueCategory::Class => RbxPropertyType::Data(RbxValueType::Ref),
        }
    }
}