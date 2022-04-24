//! Defines the conversion from Roblox Studio's API dump format into rbx-dom's
//! reflection database format.

use anyhow::bail;
use rbx_dom_weak::types::VariantType;
use rbx_reflection::ReflectionDatabase;

use super::types::Dump;

pub fn database_from_dump(dump: &Dump) -> ReflectionDatabase {
    ReflectionDatabase::new() // TODO
}

fn variant_type_from_str(value: &str) -> anyhow::Result<Option<VariantType>> {
    Ok(Some(match value {
        "Axes" => VariantType::Axes,
        "BinaryString" => VariantType::BinaryString,
        "BrickColor" => VariantType::BrickColor,
        "CFrame" => VariantType::CFrame,
        "Color3" => VariantType::Color3,
        "ColorSequence" => VariantType::ColorSequence,
        "Content" => VariantType::Content,
        "Faces" => VariantType::Faces,
        "Instance" => VariantType::Ref,
        "NumberRange" => VariantType::NumberRange,
        "NumberSequence" => VariantType::NumberSequence,
        "PhysicalProperties" => VariantType::PhysicalProperties,
        "Ray" => VariantType::Ray,
        "Rect" => VariantType::Rect,
        "Region3" => VariantType::Region3,
        "Region3int16" => VariantType::Region3int16,
        "UDim" => VariantType::UDim,
        "UDim2" => VariantType::UDim2,
        "Vector2" => VariantType::Vector2,
        "Vector2int16" => VariantType::Vector2int16,
        "Vector3" => VariantType::Vector3,
        "Vector3int16" => VariantType::Vector3int16,
        "bool" => VariantType::Bool,
        "double" => VariantType::Float64,
        "float" => VariantType::Float32,
        "int" => VariantType::Int32,
        "int64" => VariantType::Int64,
        "string" => VariantType::String,

        // ProtectedString is handled as the same as string
        "ProtectedString" => VariantType::String,

        // TweenInfo is not supported by rbx_types yet
        "TweenInfo" => return Ok(None),

        // Font is not supported by rbx_types yet
        "Font" => return Ok(None),

        // While DateTime is possible to Serialize, the only use it has as a
        // DataType is for the TextChatMessage class, which cannot be serialized
        // (at least not saved to file as it is locked to nil parent)
        "DateTime" => return Ok(None),

        // These types are not generally implemented right now.
        "QDir" | "QFont" => return Ok(None),

        _ => bail!("Unknown type {}", value),
    }))
}
