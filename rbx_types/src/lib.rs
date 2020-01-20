pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

impl Vector2int16 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

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

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}

pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}

pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}

pub struct Rect {
    pub min: Vector2,
    pub max: Vector2,
}

pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

impl UDim {
    pub fn new(scale: f32, offset: i32) -> Self {
        Self { scale, offset }
    }
}

pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

pub struct PhysicalProperties {
    pub density: f32,
    pub friction: f32,
    pub elasticity: f32,
    pub friction_weight: f32,
    pub elasticity_weight: f32,
}

pub struct NumberRange {
    pub min: f32,
    pub max: f32,
}

pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>,
}

pub struct ColorSequenceKeypoint {
    pub time: f32,
    pub color: Color3,
}

pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

pub struct NumberSequenceKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
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

pub struct Faces {
    flags: FaceFlags,
}

impl Faces {
    pub fn empty() -> Self {
        Self {
            flags: FaceFlags::empty(),
        }
    }

    pub fn has_top(&self) -> bool {
        self.flags.contains(FaceFlags::TOP)
    }

    pub fn has_bottom(&self) -> bool {
        self.flags.contains(FaceFlags::BOTTOM)
    }

    pub fn has_left(&self) -> bool {
        self.flags.contains(FaceFlags::LEFT)
    }

    pub fn has_right(&self) -> bool {
        self.flags.contains(FaceFlags::RIGHT)
    }

    pub fn has_back(&self) -> bool {
        self.flags.contains(FaceFlags::BACK)
    }

    pub fn has_front(&self) -> bool {
        self.flags.contains(FaceFlags::FRONT)
    }
}

bitflags::bitflags! {
    struct AxisFlags: u8 {
        const X = 1;
        const Y = 2;
        const Z = 4;
    }
}

pub struct Axes {
    axes: AxisFlags,
}

impl Axes {
    pub fn empty() -> Self {
        Self {
            axes: AxisFlags::empty(),
        }
    }

    pub fn has_x(&self) -> bool {
        self.axes.contains(AxisFlags::X)
    }

    pub fn has_y(&self) -> bool {
        self.axes.contains(AxisFlags::Y)
    }

    pub fn has_z(&self) -> bool {
        self.axes.contains(AxisFlags::Z)
    }
}
