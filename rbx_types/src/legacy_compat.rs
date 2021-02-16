//! Compatibility conversions for rbx_dom_weak v1 and this crate.

use std::convert::TryFrom;

use rbx_dom_weak::{
    BrickColor as LegacyBrickColor, ColorSequence as LegacyColorSequence,
    ColorSequenceKeypoint as LegacyColorSequenceKeypoint, NumberSequence as LegacyNumberSequence,
    NumberSequenceKeypoint as LegacyNumberSequenceKeypoint,
    PhysicalProperties as LegacyPhysicalProperties, Ray as LegacyRay, RbxValue, Rect as LegacyRect,
    SharedString as LegacySharedString,
};

use crate::{
    BrickColor, CFrame, Color3, Color3uint8, ColorSequence, ColorSequenceKeypoint,
    CustomPhysicalProperties, Enum, Matrix3, NumberRange, NumberSequence, NumberSequenceKeypoint,
    PhysicalProperties, Ray, Rect, SharedString, UDim, UDim2, Variant, Vector2, Vector2int16,
    Vector3, Vector3int16,
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
            RbxValue::Enum { value } => Variant::Enum(Enum::from_u32(value)),
            RbxValue::NumberRange { value } => Variant::NumberRange(NumberRange {
                min: value.0,
                max: value.1,
            }),
            RbxValue::NumberSequence { value } => {
                let keypoints = value
                    .keypoints
                    .into_iter()
                    .map(|keypoint| {
                        NumberSequenceKeypoint::new(
                            keypoint.time,
                            keypoint.value,
                            keypoint.envelope,
                        )
                    })
                    .collect();

                Variant::NumberSequence(NumberSequence { keypoints })
            }

            RbxValue::PhysicalProperties { value } => {
                if let Some(properties) = value {
                    Variant::PhysicalProperties(PhysicalProperties::Custom(
                        CustomPhysicalProperties {
                            density: properties.density,
                            friction: properties.friction,
                            elasticity: properties.elasticity,
                            friction_weight: properties.friction_weight,
                            elasticity_weight: properties.elasticity_weight,
                        },
                    ))
                } else {
                    Variant::PhysicalProperties(PhysicalProperties::Default)
                }
            }

            RbxValue::Ray { value } => {
                let origin = Vector3::new(value.origin[0], value.origin[1], value.origin[2]);
                let direction =
                    Vector3::new(value.direction[0], value.direction[1], value.direction[2]);

                Variant::Ray(Ray { origin, direction })
            }

            RbxValue::Rect { value } => Variant::Rect(Rect {
                min: Vector2::new(value.min.0, value.min.1),
                max: Vector2::new(value.max.0, value.max.1),
            }),

            RbxValue::SharedString { value } => {
                Variant::SharedString(SharedString::new(value.data().to_vec()))
            }

            RbxValue::UDim { value } => Variant::UDim(UDim::new(value.0, value.1)),

            RbxValue::UDim2 { value } => Variant::UDim2(UDim2::new(
                UDim::new(value.0, value.1),
                UDim::new(value.2, value.3),
            )),

            RbxValue::Vector2 { value } => Variant::Vector2(Vector2::new(value[0], value[1])),
            RbxValue::Vector2int16 { value } => {
                Variant::Vector2int16(Vector2int16::new(value[0], value[1]))
            }
            RbxValue::Vector3 { value } => {
                Variant::Vector3(Vector3::new(value[0], value[1], value[2]))
            }
            RbxValue::Vector3int16 { value } => {
                Variant::Vector3int16(Vector3int16::new(value[0], value[1], value[2]))
            }

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
            Variant::Enum(value) => RbxValue::Enum {
                value: value.to_u32(),
            },
            Variant::NumberRange(value) => RbxValue::NumberRange {
                value: (value.min, value.max),
            },
            Variant::NumberSequence(value) => {
                let keypoints = value
                    .keypoints
                    .into_iter()
                    .map(|keypoint| LegacyNumberSequenceKeypoint {
                        time: keypoint.time,
                        value: keypoint.time,
                        envelope: keypoint.envelope,
                    })
                    .collect();

                RbxValue::NumberSequence {
                    value: LegacyNumberSequence { keypoints },
                }
            }

            Variant::PhysicalProperties(value) => {
                let new_value = match value {
                    PhysicalProperties::Custom(properties) => Some(LegacyPhysicalProperties {
                        density: properties.density,
                        friction: properties.friction,
                        elasticity: properties.elasticity,
                        friction_weight: properties.friction_weight,
                        elasticity_weight: properties.elasticity_weight,
                    }),
                    PhysicalProperties::Default => None,
                };

                RbxValue::PhysicalProperties { value: new_value }
            }

            Variant::Ray(value) => RbxValue::Ray {
                value: LegacyRay {
                    origin: [value.origin.x, value.origin.y, value.origin.z],
                    direction: [value.direction.x, value.direction.y, value.direction.z],
                },
            },

            Variant::Rect(value) => RbxValue::Rect {
                value: LegacyRect {
                    min: (value.min.x, value.min.y),
                    max: (value.max.x, value.max.y),
                },
            },

            Variant::SharedString(value) => RbxValue::SharedString {
                value: LegacySharedString::new(value.data().to_vec()),
            },

            Variant::UDim(value) => RbxValue::UDim {
                value: (value.scale, value.offset),
            },

            Variant::UDim2(value) => RbxValue::UDim2 {
                value: (value.x.scale, value.x.offset, value.y.scale, value.y.offset),
            },

            Variant::Vector2(value) => RbxValue::Vector2 {
                value: [value.x, value.y],
            },
            Variant::Vector2int16(value) => RbxValue::Vector2int16 {
                value: [value.x, value.y],
            },
            Variant::Vector3(value) => RbxValue::Vector3 {
                value: [value.x, value.y, value.z],
            },
            Variant::Vector3int16(value) => RbxValue::Vector3int16 {
                value: [value.x, value.y, value.z],
            },

            _ => return Err(format!("Cannot convert Variant {:?} to RbxValue", value)),
        })
    }
}
