use std::fmt;

use crate::lister::Lister;

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
