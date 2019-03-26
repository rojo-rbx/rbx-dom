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
    ColorSequence,
    Content,
    Enum,
    Float32,
    Float64,
    Int32,
    Int64,
    NumberSequence,
    PhysicalProperties,
    Ray,
    Rect,
    Ref,
    String,
    UDim,
    UDim2,
    Vector2,
    Vector2int16,
    Vector3,
    Vector3int16,

    #[doc(hidden)]
    __Nonexhaustive,
}

/// Represents a value that can be assigned to the properties of an instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    BinaryString { value: Vec<u8> },

    #[serde(rename_all = "PascalCase")]
    Bool { value: bool },

    #[serde(rename_all = "PascalCase")]
    CFrame { value: [f32; 12] },

    #[serde(rename_all = "PascalCase")]
    Color3 { value: [f32; 3] },

    #[serde(rename_all = "PascalCase")]
    Color3uint8 { value: [u8; 3] },

    #[serde(rename_all = "PascalCase")]
    ColorSequence { value: ColorSequence },

    #[serde(rename_all = "PascalCase")]
    Content { value: String },

    #[serde(rename_all = "PascalCase")]
    Enum { value: u32 },

    #[serde(rename_all = "PascalCase")]
    Float32 { value: f32 },

    #[serde(rename_all = "PascalCase")]
    Float64 { value: f64 },

    #[serde(rename_all = "PascalCase")]
    Int32 { value: i32 },

    #[serde(rename_all = "PascalCase")]
    Int64 { value: i64 },

    #[serde(rename_all = "PascalCase")]
    NumberSequence { value: NumberSequence },

    #[serde(rename_all = "PascalCase")]
    PhysicalProperties { value: Option<PhysicalProperties> },

    #[serde(rename_all = "PascalCase")]
    Ray { value: Ray },

    #[serde(rename_all = "PascalCase")]
    Rect { value: Rect },

    #[serde(rename_all = "PascalCase")]
    Ref { value: Option<RbxId> },

    #[serde(rename_all = "PascalCase")]
    String { value: String },

    #[serde(rename_all = "PascalCase")]
    UDim { value: (f32, i32) },

    #[serde(rename_all = "PascalCase")]
    UDim2 { value: (f32, i32, f32, i32) },

    #[serde(rename_all = "PascalCase")]
    Vector2 { value: [f32; 2] },

    #[serde(rename_all = "PascalCase")]
    Vector2int16 { value: [i16; 2] },

    #[serde(rename_all = "PascalCase")]
    Vector3 { value: [f32; 3] },

    #[serde(rename_all = "PascalCase")]
    Vector3int16 { value: [i16; 3] },

    #[doc(hidden)]
    __Nonexhaustive,
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
            RbxValue::ColorSequence { .. } => RbxValueType::ColorSequence,
            RbxValue::Content { .. } => RbxValueType::Content,
            RbxValue::Enum { .. } => RbxValueType::Enum,
            RbxValue::Float32 { .. } => RbxValueType::Float32,
            RbxValue::Float64 { .. } => RbxValueType::Float64,
            RbxValue::Int32 { .. } => RbxValueType::Int32,
            RbxValue::Int64 { .. } => RbxValueType::Int64,
            RbxValue::NumberSequence { .. } => RbxValueType::NumberSequence,
            RbxValue::PhysicalProperties { .. } => RbxValueType::PhysicalProperties,
            RbxValue::Ray { .. } => RbxValueType::Ray,
            RbxValue::Rect { .. } => RbxValueType::Rect,
            RbxValue::Ref { .. } => RbxValueType::Ref,
            RbxValue::String { .. } => RbxValueType::String,
            RbxValue::UDim { .. } => RbxValueType::UDim,
            RbxValue::UDim2 { .. } => RbxValueType::UDim2,
            RbxValue::Vector2 { .. } => RbxValueType::Vector2,
            RbxValue::Vector2int16 { .. } => RbxValueType::Vector2int16,
            RbxValue::Vector3 { .. } => RbxValueType::Vector3,
            RbxValue::Vector3int16 { .. } => RbxValueType::Vector3int16,
            RbxValue::__Nonexhaustive => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorSequenceKeypoint {
    pub time: f32,
    pub color: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NumberSequenceKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rect {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

/// Represents possible custom physical properties on a `BasePart`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhysicalProperties {
    pub density: f32,
    pub friction: f32,
    pub elasticity: f32,
    pub friction_weight: f32,
    pub elasticity_weight: f32,
}