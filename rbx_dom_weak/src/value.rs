use std::borrow::Cow;

use serde_derive::{Serialize, Deserialize};

use crate::{
    brick_color::BrickColor,
    id::RbxId,
};

/// An enum that can hold any of the types that [`RbxValue`] can.
///
/// [`RbxValue`]: enum.RbxValue.html
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RbxValueType {
    BinaryString,
    BrickColor,
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
    NumberRange,
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
    BrickColor { value: BrickColor },

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
    NumberRange { value: (f32, f32) },

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
            RbxValue::BrickColor { .. } => RbxValueType::BrickColor,
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
            RbxValue::NumberRange { .. } => RbxValueType::NumberRange,
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

    /// Attempts to convert the `RbxValue` into a new value with the given type.
    ///
    /// Is a no-op if the value is already of the correct type.
    ///
    /// If the conversion fails, the value will be given back in the `Err` case.
    pub fn try_convert(self, target_type: RbxValueType) -> Result<RbxValue, RbxValue> {
        if self.get_type() == target_type {
            return Ok(self)
        }

        match (self, target_type) {
            (RbxValue::Float32 { value }, RbxValueType::Float64) => Ok(RbxValue::Float64 { value: value as f64 }),
            (RbxValue::Float64 { value }, RbxValueType::Float32) => Ok(RbxValue::Float32 { value: value as f32 }),

            (RbxValue::Int32 { value }, RbxValueType::Int64) => Ok(RbxValue::Int64 { value: value as i64 }),
            (RbxValue::Int64 { value }, RbxValueType::Int32) => Ok(RbxValue::Int32 { value: value as i32 }),

            (this, _) => Err(this)
        }
    }

    /// Attempts to convert a reference to an `RbxValue` to a new value with the
    /// given type.
    ///
    /// Is a no-op (by returning `Some(Cow::Borrowed(_))`) if the value is
    /// already the right type.
    ///
    /// If the conversion wasn't successful, returns `None`.
    pub fn try_convert_ref<'a>(&'a self, target_type: RbxValueType) -> Option<Cow<'a, RbxValue>> {
        if self.get_type() == target_type {
            return Some(Cow::Borrowed(self))
        }

        // TODO: Reduce duplication with try_convert

        match (self, target_type) {
            (RbxValue::Float32 { value }, RbxValueType::Float64) => Some(Cow::Owned(RbxValue::Float64 { value: *value as f64 })),
            (RbxValue::Float64 { value }, RbxValueType::Float32) => Some(Cow::Owned(RbxValue::Float32 { value: *value as f32 })),

            (RbxValue::Int32 { value }, RbxValueType::Int64) => Some(Cow::Owned(RbxValue::Int64 { value: *value as i64 })),
            (RbxValue::Int64 { value }, RbxValueType::Int32) => Some(Cow::Owned(RbxValue::Int32 { value: *value as i32 })),

            (_this, _) => None
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