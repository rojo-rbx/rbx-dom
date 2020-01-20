mod lister;

use std::fmt;

use lister::Lister;

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
pub struct PhysicalProperties {
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

bitflags::bitflags! {
    struct FaceFlags: u8 {
        const RIGHT = 1;
        const TOP = 2;
        const BACK = 4;
        const LEFT = 8;
        const BOTTOM = 16;
        const FRONT = 32;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Faces {
    flags: FaceFlags,
}

impl Faces {
    pub const RIGHT: Self = Self {
        flags: FaceFlags::RIGHT,
    };

    pub const TOP: Self = Self {
        flags: FaceFlags::TOP,
    };

    pub const BACK: Self = Self {
        flags: FaceFlags::BACK,
    };

    pub const LEFT: Self = Self {
        flags: FaceFlags::LEFT,
    };

    pub const BOTTOM: Self = Self {
        flags: FaceFlags::BOTTOM,
    };

    pub const FRONT: Self = Self {
        flags: FaceFlags::FRONT,
    };
}

impl Faces {
    pub fn empty() -> Self {
        Self {
            flags: FaceFlags::empty(),
        }
    }

    pub fn all() -> Self {
        Self {
            flags: FaceFlags::all(),
        }
    }

    pub fn contains(self, other: Self) -> bool {
        self.flags.contains(other.flags)
    }
}

impl fmt::Display for Faces {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut list = Lister::new();

        write!(out, "Faces(")?;

        if self.contains(Faces::RIGHT) {
            list.write(out, "Right")?;
        }

        if self.contains(Faces::TOP) {
            list.write(out, "Top")?;
        }

        if self.contains(Faces::BACK) {
            list.write(out, "Back")?;
        }

        if self.contains(Faces::LEFT) {
            list.write(out, "Left")?;
        }

        if self.contains(Faces::BOTTOM) {
            list.write(out, "Bottom")?;
        }

        if self.contains(Faces::FRONT) {
            list.write(out, "Front")?;
        }

        write!(out, ")")
    }
}

bitflags::bitflags! {
    struct AxisFlags: u8 {
        const X = 1;
        const Y = 2;
        const Z = 4;
    }
}

#[derive(Clone, Copy)]
pub struct Axes {
    flags: AxisFlags,
}

impl Axes {
    pub const X: Self = Self {
        flags: AxisFlags::X,
    };

    pub const Y: Self = Self {
        flags: AxisFlags::Y,
    };

    pub const Z: Self = Self {
        flags: AxisFlags::Z,
    };
}

impl Axes {
    pub fn empty() -> Self {
        Self {
            flags: AxisFlags::empty(),
        }
    }

    pub fn all() -> Self {
        Self {
            flags: AxisFlags::all(),
        }
    }

    pub fn contains(self, other: Self) -> bool {
        self.flags.contains(other.flags)
    }
}

impl fmt::Debug for Axes {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut list = Lister::new();

        write!(out, "Axes(")?;

        if self.contains(Self::X) {
            list.write(out, "X")?;
        }

        if self.contains(Self::Y) {
            list.write(out, "Y")?;
        }

        if self.contains(Self::Z) {
            list.write(out, "Z")?;
        }

        write!(out, ")")
    }
}
