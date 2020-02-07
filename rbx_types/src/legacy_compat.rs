//! Compatibility conversions for rbx_dom_weak v1 and this crate.

use std::convert::TryFrom;

use rbx_dom_weak::{
    BrickColor as LegacyBrickColor, ColorSequence as LegacyColorSequence,
    ColorSequenceKeypoint as LegacyColorSequenceKeypoint, RbxValue,
};

use crate::{
    BrickColor, CFrame, Color3, Color3uint8, ColorSequence, ColorSequenceKeypoint, EnumValue,
    Matrix3, Variant, Vector3,
};

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
            RbxValue::BrickColor { value } => {
                Variant::BrickColor(BrickColor::from_number(value as u16).unwrap())
            }

            RbxValue::CFrame { value } => Variant::CFrame(CFrame::new(
                Vector3::new(value[0], value[1], value[2]),
                Matrix3 {
                    x: Vector3::new(value[3], value[4], value[5]),
                    y: Vector3::new(value[6], value[7], value[8]),
                    z: Vector3::new(value[9], value[10], value[11]),
                },
            )),

            RbxValue::Color3 { value } => {
                Variant::Color3(Color3::new(value[0], value[1], value[2]))
            }
            RbxValue::Color3uint8 { value } => {
                Variant::Color3uint8(Color3uint8::new(value[0], value[1], value[2]))
            }

            RbxValue::ColorSequence { value } => {
                let keypoints = value
                    .keypoints
                    .into_iter()
                    .map(|keypoint| {
                        ColorSequenceKeypoint::new(
                            keypoint.time,
                            Color3::new(keypoint.color[0], keypoint.color[1], keypoint.color[2]),
                        )
                    })
                    .collect();

                Variant::ColorSequence(ColorSequence { keypoints })
            }

            RbxValue::Content { value } => Variant::Content(value.into()),
            RbxValue::Enum { value } => Variant::EnumValue(EnumValue::from_u32(value)),
            // RbxValue::NumberRange { value } => Variant::NumberRange(value),
            // RbxValue::NumberSequence { value } => Variant::NumberSequence(value),
            // RbxValue::PhysicalProperties { value } => Variant::PhysicalProperties(value),
            // RbxValue::Ray { value } => Variant::Ray(value),
            // RbxValue::Rect { value } => Variant::Rect(value),
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

            Variant::BinaryString(value) => RbxValue::BinaryString {
                value: value.into(),
            },

            Variant::BrickColor(value) => RbxValue::BrickColor {
                value: LegacyBrickColor::from_number(value as u16).unwrap(),
            },

            Variant::CFrame(value) => RbxValue::CFrame {
                value: [
                    value.position.x,
                    value.position.y,
                    value.position.z,
                    value.orientation.x.x,
                    value.orientation.x.y,
                    value.orientation.x.z,
                    value.orientation.y.x,
                    value.orientation.y.y,
                    value.orientation.y.z,
                    value.orientation.z.x,
                    value.orientation.z.y,
                    value.orientation.z.z,
                ],
            },

            Variant::Color3(value) => RbxValue::Color3 {
                value: [value.r, value.g, value.b],
            },
            Variant::Color3uint8(value) => RbxValue::Color3uint8 {
                value: [value.r, value.g, value.b],
            },

            Variant::ColorSequence(value) => {
                let keypoints = value
                    .keypoints
                    .into_iter()
                    .map(|keypoint| LegacyColorSequenceKeypoint {
                        time: keypoint.time,
                        color: [keypoint.color.r, keypoint.color.g, keypoint.color.b],
                    })
                    .collect();

                RbxValue::ColorSequence {
                    value: LegacyColorSequence { keypoints },
                }
            }

            Variant::Content(value) => RbxValue::Content {
                value: value.into_string(),
            },
            Variant::EnumValue(value) => RbxValue::Enum {
                value: value.to_u32(),
            },
            // Variant::NumberRange(value) => RbxValue::NumberRange { value },
            // Variant::NumberSequence(value) => RbxValue::NumberSequence { value },
            // Variant::PhysicalProperties(value) => RbxValue::PhysicalProperties { value },
            // Variant::Ray(value) => RbxValue::Ray { value },
            // Variant::Rect(value) => RbxValue::Rect { value },
            // Variant::SharedString(value) => RbxValue::SharedString { value },
            // Variant::UDim(value) => RbxValue::UDim { value },
            // Variant::UDim2(value) => RbxValue::UDim2 { value },
            // Variant::Vector2(value) => RbxValue::Vector2 { value },
            // Variant::Vector2int16(value) => RbxValue::Vector2int16 { value },
            // Variant::Vector3(value) => RbxValue::Vector3 { value },
            // Variant::Vector3int16(value) => RbxValue::Vector3int16 { value },
            _ => return Err(format!("Cannot convert Variant {:?} to RbxValue", value)),
        })
    }
}
