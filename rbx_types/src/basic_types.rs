/// A reference to a Roblox instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ref([u8; 16]);

impl Ref {
    pub fn new() -> Self {
        Ref(rand::random())
    }
}

/// Represents any Roblox enum value.
///
/// Roblox enums are not strongly typed, so the meaning of a value depends on
/// where they're assigned.
///
/// A list of all enums and their values are available [on the Roblox Developer
/// Hub](https://developer.roblox.com/en-us/api-reference/enum).
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

/// The standard 2D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector2int16`][struct.Vector2int16.html]
/// * [Vector2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2)
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

/// A version of [`Vector2`][Vector2] whose coordinates are signed 16-bit
/// integers.
///
/// ## See Also
/// * [`Vector2`][Vector2], which is used for most values.
/// * [Vector2int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2int16)
///
/// [Vector2]: struct.Vector2.html
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

/// The standard 3D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector3int16`][struct.Vector3int16.html]
/// * [Vector3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3)
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

/// A version of [`Vector3`][Vector3] whose coordinates are signed 16-bit
/// integers. `Vector3int16` is often used when working with Terrain.
///
/// ## See Also
/// * [`Vector3`][Vector3], which is used for most values.
/// * [Vector3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3int16)
///
/// [Vector3]: struct.Vector3.html
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

/// Represents a position and orientation in 3D space.
///
/// ## See Also
/// * [CFrame on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/CFrame)
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

/// Represents any color, including HDR colors.
///
/// ## See Also
/// * [`Color3uint8`](struct.Color3uint8.html), which is used instead of
///   `Color3` on some types and does not represent HDR colors.
/// * [Color3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Color3)
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

/// Represents non-HDR colors, i.e. those whose individual color channels do not
/// exceed 1. This type is used for serializing properties like
/// [`BasePart.Color`][BasePart.Color], but is not exposed as a distinct type to
/// Lua code.
///
/// ## See Also
/// * [`Color3`](struct.Color3.html), which is more common and can represent HDR
///   colors.
///
/// [BasePart.Color]: https://developer.roblox.com/en-us/api-reference/property/BasePart/Color
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

/// Represents a ray in 3D space. Direction does not have to be a unit vector,
/// and is used by APIs like [`Workspace:FindPartOnRay`][FindPartOnRay] to set a
/// max distance.
///
/// ## See Also
/// * [Ray on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Ray)
///
/// [FindPartOnRay]: https://developer.roblox.com/en-us/api-reference/function/WorldRoot/FindPartOnRay
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

/// Represents a bounding box in 3D space.
///
/// ## See Also
/// * [`Region3int16`](struct.Region3int16.html)
/// * [Region3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3)
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

/// A version of [`Region3`][Region3] that uses signed 16-bit integers instead
/// of floats. `Region3int16` is generally used in Terrain APIs.
///
/// ## See Also
/// * [`Region`][Region3]
/// * [Region3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3int16)
///
/// [Region3]: struct.Region3.html
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

/// Represents a bounding rectangle in 2D space.
///
/// ## See Also
/// * [Rect on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Rect)
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

/// Standard unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim)
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

/// Standard 2D unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim2)
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

/// A range between two numbers.
///
/// ## See Also
/// * [NumberRange on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberRange)
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

/// A series of colors that can be tweened through.
///
/// ## See Also
/// * [ColorSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequence)
#[derive(Debug, Clone)]
pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>,
}

/// A single color and point in time of a [`ColorSequence`][ColorSequence]
///
/// ## See Also
/// * [ColorSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequenceKeypoint)
///
/// [ColorSequence]: struct.ColorSequence.html
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

/// A sequence of numbers on a timeline. Each point contains a timestamp, a
/// value, and a range that allows for randomized values.
///
/// ## See Also
/// * [NumberSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequence)
#[derive(Debug, Clone)]
pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

/// A single value, envelope, and point in time of a [`NumberSequence`][NumberSequence]
///
/// ## See Also
/// * [`NumberSequence`][NumberSequence]
/// * [NumberSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequenceKeypoint)
///
/// [NumberSequence]: struct.NumberSequence.html
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
}
