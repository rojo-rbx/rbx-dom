use serde_derive::{Serialize, Deserialize};

use crate::{
    brick_color::BrickColor,
    shared_string::SharedString,
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
    SharedString,
    String,
    UDim,
    UDim2,
    Vector2,
    Vector2int16,
    Vector3,
    Vector3int16,

    #[doc(hidden)]
    #[serde(skip)]
    __Nonexhaustive,
}

/// Represents a value that can be assigned to the properties of an instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    BinaryString {
        #[serde(with = "base64_encoding")]
        value: Vec<u8>,
    },

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
    SharedString { value: SharedString },

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
    #[serde(skip)]
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
            RbxValue::SharedString { .. } => RbxValueType::SharedString,
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

    /// Attempts to convert a reference to an `RbxValue` to a new value with the
    /// given type.
    ///
    /// Is a no-op (by returning `RbxValueConversion::Unnecessary`) if the value
    /// is already the right type.
    ///
    /// If the conversion wasn't successful, returns
    /// `RbxValueConversion::Failed`.
    pub fn try_convert_ref(&self, target_type: RbxValueType) -> RbxValueConversion {
        use self::RbxValueConversion::*;

        if self.get_type() == target_type {
            return Unnecessary;
        }

        // These conversions should be uniform in style.
        #[allow(clippy::cast_lossless)]
        match (self, target_type) {
            // Floats can be widened for compatibility
            (RbxValue::Float32 { value }, RbxValueType::Float64) =>
                Converted(RbxValue::Float64 { value: *value as f64 }),
            (RbxValue::Float64 { value }, RbxValueType::Float32) =>
                Converted(RbxValue::Float32 { value: *value as f32 }),

            // Integers can be widened; MANY types migrated from Int32 to Int64
            // and may appear as either.
            (RbxValue::Int32 { value }, RbxValueType::Int64) => Converted(RbxValue::Int64 { value: *value as i64 }),
            (RbxValue::Int64 { value }, RbxValueType::Int32) => Converted(RbxValue::Int32 { value: *value as i32 }),

            // Strings can be treated as content values for compatibility, and
            // since their representation is functionally identical.
            (RbxValue::String { value }, RbxValueType::Content) => Converted(RbxValue::Content { value: value.clone() }),

            // The difference between Color3 and Color3uint8 isn't surfaced to
            // users. The difference is meaningful for lighting properties that
            // can represent HDR colors.
            (RbxValue::Color3 { value }, RbxValueType::Color3uint8) => {
                Converted(RbxValue::Color3uint8 {
                    value: [
                        (value[0] * 255.0) as u8,
                        (value[1] * 255.0) as u8,
                        (value[2] * 255.0) as u8,
                    ],
                })
            }
            (RbxValue::Color3uint8 { value }, RbxValueType::Color3) => {
                Converted(RbxValue::Color3 {
                    value: [
                        value[0] as f32 / 255.0,
                        value[1] as f32 / 255.0,
                        value[2] as f32 / 255.0,
                    ],
                })
            }

            // BrickColor can be converted one-way to Color3 or Color3uint8,
            // which is generally preferred. We don't have the opposite
            // conversion, which is much less useful.
            (RbxValue::BrickColor { value }, RbxValueType::Color3) =>
                Converted(RbxValue::Color3 { value: value.as_rgb_f32() }),
            (RbxValue::BrickColor { value }, RbxValueType::Color3uint8) =>
                Converted(RbxValue::Color3uint8 { value: value.as_rgb() }),

            // Some BrickColor properties (like SpawnLocation.TeamColor) are
            // ints for some reason, so we downcast them if they're in range for
            // BrickColor.
            (RbxValue::Int32 { value }, RbxValueType::BrickColor) => {
                if *value > 255 || *value < 0 {
                    return Failed;
                }

                match BrickColor::from_palette(*value as u8) {
                    Some(converted) => Converted(RbxValue::BrickColor { value: converted }),
                    None => Failed,
                }
            }

            _ => Failed
        }
    }
}

/// Contains the result of trying to convert an `RbxValue` to another type using
/// `RbxValue::try_convert_ref`.
#[derive(Debug, Clone, PartialEq)]
pub enum RbxValueConversion {
    /// The value was converted successfully, the value is attached.
    Converted(RbxValue),

    /// The value was already of the right type, so a conversion is unnecessary.
    Unnecessary,

    /// The value and target type were incompatible and a conversion could not
    /// occur.
    Failed,
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

/// Methods to help encode BinaryString values to base64 when used with
/// human-readable formats like JSON.
mod base64_encoding {
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if serializer.is_human_readable() {
            serializer.collect_str(&base64::display::Base64Display::with_config(bytes, base64::STANDARD))
        } else {
            serializer.serialize_bytes(bytes)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where D: Deserializer<'de>
    {
        if deserializer.is_human_readable() {
            let s = <&str>::deserialize(deserializer)?;
            base64::decode(s).map_err(de::Error::custom)
        } else {
            <Vec<u8>>::deserialize(deserializer)
        }
    }
}