//! Compatibility conversions for rbx_dom_weak v1 and this crate.

use std::convert::TryFrom;

use rbx_dom_weak::{RbxValue, RbxValueType};

use crate::{Variant, VariantType};

impl TryFrom<RbxValue> for Variant {
    type Error = String;

    fn try_from(value: RbxValue) -> Result<Self, Self::Error> {
        Ok(match value {
            RbxValue::String { value } => value.into(),
            RbxValue::Bool { value } => value.into(),
            RbxValue::Int32 { value } => value.into(),
            RbxValue::Int64 { value } => value.into(),
            RbxValue::Float32 { value } => value.into(),
            RbxValue::Float64 { value } => value.into(),

            RbxValue::BinaryString { value } => Variant::BinaryString(value.into()),

            // RbxValue::BrickColor { value } => Variant::BrickColor(value),
            // RbxValue::CFrame { value } => Variant::CFrame(value),
            // RbxValue::Color3 { value } => Variant::Color3(value),
            // RbxValue::Color3uint8 { value } => Variant::Color3uint8(value),
            // RbxValue::ColorSequence { value } => Variant::ColorSequence(value),
            // RbxValue::Content { value } => Variant::Content(value),
            // RbxValue::Enum { value } => Variant::EnumValue(value),
            // RbxValue::NumberRange { value } => Variant::NumberRange(value),
            // RbxValue::NumberSequence { value } => Variant::NumberSequence(value),
            // RbxValue::PhysicalProperties { value } => Variant::PhysicalProperties(value),
            // RbxValue::Ray { value } => Variant::Ray(value),
            // RbxValue::Rect { value } => Variant::Rect(value),
            // RbxValue::Ref { value } => Variant::Ref(value),
            // RbxValue::SharedString { value } => Variant::SharedString(value),
            // RbxValue::UDim { value } => Variant::UDim(value),
            // RbxValue::UDim2 { value } => Variant::UDim2(value),
            // RbxValue::Vector2 { value } => Variant::Vector2(value),
            // RbxValue::Vector2int16 { value } => Variant::Vector2int16(value),
            // RbxValue::Vector3 { value } => Variant::Vector3(value),
            // RbxValue::Vector3int16 { value } => Variant::Vector3int16(value),
            _ => return Err(format!("Cannot convert RbxValue {:?} to Variant", value)),
        })
    }
}

impl TryFrom<Variant> for RbxValue {
    type Error = String;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        Ok(match value {
            Variant::String(value) => RbxValue::String { value },
            Variant::Bool(value) => RbxValue::Bool { value },
            Variant::Int32(value) => RbxValue::Int32 { value },
            Variant::Int64(value) => RbxValue::Int64 { value },
            Variant::Float32(value) => RbxValue::Float32 { value },
            Variant::Float64(value) => RbxValue::Float64 { value },

            // RbxValue::BrickColor { value } => Variant::BrickColor(value),
            // RbxValue::CFrame { value } => Variant::CFrame(value),
            // RbxValue::Color3 { value } => Variant::Color3(value),
            // RbxValue::Color3uint8 { value } => Variant::Color3uint8(value),
            // RbxValue::ColorSequence { value } => Variant::ColorSequence(value),
            // RbxValue::Content { value } => Variant::Content(value),
            // RbxValue::Enum { value } => Variant::EnumValue(value),
            // RbxValue::NumberRange { value } => Variant::NumberRange(value),
            // RbxValue::NumberSequence { value } => Variant::NumberSequence(value),
            // RbxValue::PhysicalProperties { value } => Variant::PhysicalProperties(value),
            // RbxValue::Ray { value } => Variant::Ray(value),
            // RbxValue::Rect { value } => Variant::Rect(value),
            // RbxValue::Ref { value } => Variant::Ref(value),
            // RbxValue::SharedString { value } => Variant::SharedString(value),
            // RbxValue::UDim { value } => Variant::UDim(value),
            // RbxValue::UDim2 { value } => Variant::UDim2(value),
            // RbxValue::Vector2 { value } => Variant::Vector2(value),
            // RbxValue::Vector2int16 { value } => Variant::Vector2int16(value),
            // RbxValue::Vector3 { value } => Variant::Vector3(value),
            // RbxValue::Vector3int16 { value } => Variant::Vector3int16(value),
            _ => return Err(format!("Cannot convert Variant {:?} to RbxValue", value)),
        })
    }
}
