use std::{convert::TryFrom, fmt};

use rbx_dom_weak::RbxValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Type {
    String = 0x01,
    Bool = 0x02,
    Int32 = 0x03,
    Float32 = 0x04,
    Float64 = 0x05,
    UDim = 0x06,
    UDim2 = 0x07,
    Ray = 0x08,
    Faces = 0x09,
    Axis = 0x0A,
    BrickColor = 0x0B,
    Color3 = 0x0C,
    Vector2 = 0x0D,
    Vector3 = 0x0E,
    CFrame = 0x10,
    Enum = 0x12,
    Ref = 0x13,
    Vector3int16 = 0x14,
    NumberSequence = 0x15,
    ColorSequence = 0x16,
    NumberRange = 0x17,
    Rect = 0x18,
    PhysicalProperties = 0x19,
    Color3uint8 = 0x1A,
    Int64 = 0x1B,
}

impl Type {
    pub fn from_rbx_type(rbx_type: RbxValueType) -> Option<Type> {
        Some(match rbx_type {
            // These types all serialize the same way in the binary format.
            RbxValueType::String => Type::String,
            RbxValueType::BinaryString => Type::String,
            RbxValueType::Content => Type::String,

            RbxValueType::Bool => Type::Bool,
            RbxValueType::Int32 => Type::Int32,
            RbxValueType::Float32 => Type::Float32,
            RbxValueType::Float64 => Type::Float64,
            RbxValueType::UDim => Type::UDim,
            RbxValueType::UDim2 => Type::UDim2,
            RbxValueType::Ray => Type::Ray,

            // Types not supported in rbx_dom_weak yet:
            // RbxValueType::Faces => Type::Faces,
            // RbxValueType::Axis => Type::Axis,
            RbxValueType::BrickColor => Type::BrickColor,
            RbxValueType::Color3 => Type::Color3,
            RbxValueType::Vector2 => Type::Vector2,
            RbxValueType::Vector3 => Type::Vector3,
            RbxValueType::CFrame => Type::CFrame,
            RbxValueType::Enum => Type::Enum,
            RbxValueType::Ref => Type::Ref,
            RbxValueType::Vector3int16 => Type::Vector3int16,
            RbxValueType::NumberSequence => Type::NumberSequence,
            RbxValueType::ColorSequence => Type::ColorSequence,
            RbxValueType::NumberRange => Type::NumberRange,
            RbxValueType::Rect => Type::Rect,
            RbxValueType::PhysicalProperties => Type::PhysicalProperties,
            RbxValueType::Color3uint8 => Type::Color3uint8,
            RbxValueType::Int64 => Type::Int64,

            _ => return None,
        })
    }

    pub fn to_default_rbx_type(&self) -> Option<RbxValueType> {
        Some(match self {
            // Since many buffers aren't going to be valid UTF-8, it's safer to
            // pick BinaryString for unknown property types instead of String.
            Type::String => RbxValueType::BinaryString,
            Type::Bool => RbxValueType::Bool,
            Type::Int32 => RbxValueType::Int32,
            Type::Float32 => RbxValueType::Float32,
            Type::Float64 => RbxValueType::Float64,
            Type::UDim => RbxValueType::UDim,
            Type::UDim2 => RbxValueType::UDim2,
            Type::Ray => RbxValueType::Ray,
            Type::Faces => return None,
            Type::Axis => return None,
            Type::BrickColor => RbxValueType::BrickColor,
            Type::Color3 => RbxValueType::Color3,
            Type::Vector2 => RbxValueType::Vector2,
            Type::Vector3 => RbxValueType::Vector3,
            Type::CFrame => RbxValueType::CFrame,
            Type::Enum => RbxValueType::Enum,
            Type::Ref => RbxValueType::Ref,
            Type::Vector3int16 => RbxValueType::Vector3int16,
            Type::NumberSequence => RbxValueType::NumberSequence,
            Type::ColorSequence => RbxValueType::ColorSequence,
            Type::NumberRange => RbxValueType::NumberRange,
            Type::Rect => RbxValueType::Rect,
            Type::PhysicalProperties => RbxValueType::PhysicalProperties,
            Type::Color3uint8 => RbxValueType::Color3uint8,
            Type::Int64 => RbxValueType::Int64,
        })
    }
}

impl TryFrom<u8> for Type {
    type Error = InvalidTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Type::*;

        Ok(match value {
            0x01 => String,
            0x02 => Bool,
            0x03 => Int32,
            0x04 => Float32,
            0x05 => Float64,
            0x06 => UDim,
            0x07 => UDim2,
            0x08 => Ray,

            // Unsupported:
            0x09 => Faces,
            0x0A => Axis,
            0x0B => BrickColor,
            0x0C => Color3,
            0x0D => Vector2,
            0x0E => Vector3,
            0x10 => CFrame,
            0x12 => Enum,
            0x13 => Ref,
            0x14 => Vector3int16,
            0x15 => NumberSequence,
            0x16 => ColorSequence,
            0x17 => NumberRange,
            0x18 => Rect,
            0x19 => PhysicalProperties,
            0x1A => Color3uint8,
            0x1B => Int64,
            _ => return Err(InvalidTypeError(value)),
        })
    }
}

#[derive(Debug)]
pub struct InvalidTypeError(u8);

impl std::error::Error for InvalidTypeError {}

impl fmt::Display for InvalidTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Invalid binary type value {:x?}", self.0)
    }
}
