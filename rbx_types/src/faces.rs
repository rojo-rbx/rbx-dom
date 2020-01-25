use std::fmt;

use crate::lister::Lister;

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
    pub const fn empty() -> Self {
        Self {
            flags: FaceFlags::empty(),
        }
    }

    pub const fn all() -> Self {
        Self {
            flags: FaceFlags::all(),
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        self.flags.contains(other.flags)
    }

    pub const fn bits(self) -> u8 {
        self.flags.bits()
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        FaceFlags::from_bits(bits).map(|flags| Self { flags })
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
