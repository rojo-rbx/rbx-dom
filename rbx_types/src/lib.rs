pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct CFrame {
    pub position: Vector3,
    pub orientation: [f32; 9],
}

pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Color3uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
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
