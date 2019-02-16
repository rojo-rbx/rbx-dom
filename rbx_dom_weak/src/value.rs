use crate::id::RbxId;
use serde_derive::{Serialize, Deserialize};

/// An enum that can hold any of the types that [`RbxValue`] can.
///
/// [`RbxValue`]: enum.RbxValue.html
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RbxValueType {
    BinaryString,
    Bool,
    CFrame,
    Color3,
    Color3uint8,
    Enum,
    Float32,
    Int32,
    PhysicalProperties,
    Ref,
    String,
    Vector2,
    Vector2int16,
    Vector3,
    Vector3int16,
    UDim,
    UDim2,
}

/// Represents a value that can be assigned to the properties of an instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    String {
        value: String,
    },

    #[serde(rename_all = "PascalCase")]
    BinaryString {
        value: Vec<u8>,
    },

    #[serde(rename_all = "PascalCase")]
    Bool {
        value: bool,
    },

    #[serde(rename_all = "PascalCase")]
    Int32 {
        value: i32,
    },

    #[serde(rename_all = "PascalCase")]
    Float32 {
        value: f32,
    },

    #[serde(rename_all = "PascalCase")]
    Enum {
        value: u32,
    },

    #[serde(rename_all = "PascalCase")]
    Vector3 {
        value: [f32; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector2 {
        value: [f32; 2],
    },

    #[serde(rename_all = "PascalCase")]
    Color3 {
        value: [f32; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Color3uint8 {
        value: [u8; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector3int16 {
        value: [i16; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector2int16 {
        value: [i16; 2],
    },

    #[serde(rename_all = "PascalCase")]
    CFrame {
        value: [f32; 12],
    },

    #[serde(rename_all = "PascalCase")]
    PhysicalProperties {
        value: Option<PhysicalProperties>,
    },

    #[serde(rename_all = "PascalCase")]
    Ref {
        value: Option<RbxId>,
    },

    #[serde(rename_all = "PascalCase")]
    UDim {
        value: [f32; 2],
    },

    #[serde(rename_all = "PascalCase")]
    UDim2 {
        value: [f32; 4],
    }
}

impl RbxValue {
    /// Returns the type of this value as a [`RbxValueType`].
    ///
    /// [`RbxValueType`]: enum.RbxValueType.html
    pub fn get_type(&self) -> RbxValueType {
        match self {
            RbxValue::BinaryString { .. } => RbxValueType::BinaryString,
            RbxValue::Bool { .. } => RbxValueType::Bool,
            RbxValue::CFrame { .. } => RbxValueType::CFrame,
            RbxValue::Color3 { .. } => RbxValueType::Color3,
            RbxValue::Color3uint8 { .. } => RbxValueType::Color3uint8,
            RbxValue::Enum { .. } => RbxValueType::Enum,
            RbxValue::Float32 { .. } => RbxValueType::Float32,
            RbxValue::Int32 { .. } => RbxValueType::Int32,
            RbxValue::PhysicalProperties { .. } => RbxValueType::PhysicalProperties,
            RbxValue::Ref { .. } => RbxValueType::Ref,
            RbxValue::String { .. } => RbxValueType::String,
            RbxValue::Vector2 { .. } => RbxValueType::Vector2,
            RbxValue::Vector2int16 { .. } => RbxValueType::Vector2int16,
            RbxValue::Vector3 { .. } => RbxValueType::Vector3,
            RbxValue::Vector3int16 { .. } => RbxValueType::Vector3int16,
            RbxValue::UDim { .. } => RbxValueType::UDim,
            RbxValue::UDim2 { .. } => RbxValueType::UDim2,
        }
    }
}

/// Represents possible custom physical properties on a `BasePart`.
///
/// Currently a stub.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalProperties;