#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Ref;

#[derive(Debug, Clone, Copy)]
pub struct EnumValue {
    value: u32,
}

impl EnumValue {
    pub fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub fn to_u32(self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

impl Vector2int16 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Vector3int16 {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CFrame {
    pub position: Vector3,
    pub orientation: Matrix3,
}

impl CFrame {
    pub fn new(position: Vector3, orientation: Matrix3) -> Self {
        Self {
            position,
            orientation,
        }
    }
}

/// Used to represent the `orientation` field of `CFrame` and not a standalone
/// type in Roblox.
#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn identity() -> Self {
        Self {
            x: Vector3::new(1.0, 0.0, 1.0),
            y: Vector3::new(0.0, 1.0, 0.0),
            z: Vector3::new(0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color3uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3uint8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Region3 {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}

impl Region3int16 {
    pub fn new(min: Vector3int16, max: Vector3int16) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vector2,
    pub max: Vector2,
}

impl Rect {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

impl UDim {
    pub fn new(scale: f32, offset: i32) -> Self {
        Self { scale, offset }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    pub fn new(x: UDim, y: UDim) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PhysicalProperties {
    Default,
    Custom(CustomPhysicalProperties),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct CustomPhysicalProperties {
    pub density: f32,
    pub friction: f32,
    pub elasticity: f32,
    pub friction_weight: f32,
    pub elasticity_weight: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct NumberRange {
    pub min: f32,
    pub max: f32,
}

impl NumberRange {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone)]
pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorSequenceKeypoint {
    pub time: f32,
    pub color: Color3,
}

impl ColorSequenceKeypoint {
    pub fn new(time: f32, color: Color3) -> Self {
        Self { time, color }
    }
}

#[derive(Debug, Clone)]
pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

#[derive(Debug, Clone, Copy)]
pub struct NumberSequenceKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
}

impl NumberSequenceKeypoint {
    pub fn new(time: f32, value: f32, envelope: f32) -> Self {
        Self {
            time,
            value,
            envelope,
        }
    }
}

#[cfg(feature = "serde")]
serde_tuple! {
    Vector2(x: f32, y: f32),
    Vector2int16(x: i16, y: i16),
    Vector3(x: f32, y: f32, z: f32),
    Vector3int16(x: i16, y: i16, z: i16),

    Color3(r: f32, g: f32, b: f32),
    Color3uint8(r: u8, g: u8, b: u8),

    UDim(scale: f32, offset: i32),

    NumberRange(min: f32, max: f32),
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn vec2_json() {
        let out = serde_json::to_string(&Vector2 { x: 2.0, y: 3.5 }).unwrap();

        assert_eq!(out, "[2.0,3.5]");
    }

    #[test]
    fn physical_properties() {
        let custom = serde_json::to_string(&PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 1.0,
            friction: 0.5,
            elasticity: 0.0,
            elasticity_weight: 5.0,
            friction_weight: 6.0,
        }))
        .unwrap();

        assert_eq!(custom, "{\"Density\":1.0,\"Friction\":0.5,\"Elasticity\":0.0,\"FrictionWeight\":6.0,\"ElasticityWeight\":5.0}");

        let _default = serde_json::to_string(&PhysicalProperties::Default).unwrap();

        // TODO: Manually implement Serialize/Deserialize to ensure this is the
        // result. Currently, we get "null", which is unintuitive.

        // assert_eq!(default, "\"Default\"");
    }
}
