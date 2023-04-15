use std::fmt;

use crate::lister::Lister;

bitflags::bitflags! {
    struct AxisFlags: u8 {
        const X = 1;
        const Y = 2;
        const Z = 4;
    }
}

/// Represents a set of zero or more 3D axes.
///
/// ## See also
///
/// * [`Axes` in the Roblox Creator Documentation](https://create.roblox.com/docs/reference/engine/datatypes/Axes)
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub const fn empty() -> Self {
        Self {
            flags: AxisFlags::empty(),
        }
    }

    pub const fn all() -> Self {
        Self {
            flags: AxisFlags::all(),
        }
    }

    pub const fn contains(self, other: Self) -> bool {
        self.flags.contains(other.flags)
    }

    pub const fn bits(self) -> u8 {
        self.flags.bits()
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        AxisFlags::from_bits(bits).map(|flags| Self { flags })
    }

    #[cfg(feature = "serde")]
    fn len(self) -> usize {
        self.bits().count_ones() as usize
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

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use std::fmt;

    use serde::{
        de::{Error as _, SeqAccess, Visitor},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl Serialize for Axes {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                let mut seq = serializer.serialize_seq(Some(self.len()))?;

                if self.contains(Self::X) {
                    seq.serialize_element("X")?;
                }

                if self.contains(Self::Y) {
                    seq.serialize_element("Y")?;
                }

                if self.contains(Self::Z) {
                    seq.serialize_element("Z")?;
                }

                seq.end()
            } else {
                serializer.serialize_u8(self.bits())
            }
        }
    }

    struct HumanVisitor;

    impl<'de> Visitor<'de> for HumanVisitor {
        type Value = Axes;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a list of strings representing axes")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut flags = AxisFlags::empty();

            while let Some(axis_str) = seq.next_element::<&str>()? {
                match axis_str {
                    "X" => flags |= AxisFlags::X,
                    "Y" => flags |= AxisFlags::Y,
                    "Z" => flags |= AxisFlags::Z,
                    _ => {
                        return Err(A::Error::custom(format!("invalid axis '{axis_str}'")));
                    }
                }
            }

            Ok(Axes { flags })
        }
    }

    impl<'de> Deserialize<'de> for Axes {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                deserializer.deserialize_seq(HumanVisitor)
            } else {
                let value = u8::deserialize(deserializer)?;

                Axes::from_bits(value)
                    .ok_or_else(|| D::Error::custom("value must a u8 bitmask of axes"))
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human_de() {
        let empty: Axes = serde_json::from_str("[]").unwrap();
        assert_eq!(empty, Axes::empty());

        let x: Axes = serde_json::from_str(r#"["X"]"#).unwrap();
        assert_eq!(x, Axes::X);

        let all: Axes = serde_json::from_str(r#"["X", "Y", "Z"]"#).unwrap();
        assert_eq!(all, Axes::all());
    }

    #[test]
    fn human_ser() {
        let empty = serde_json::to_string(&Axes::empty()).unwrap();
        assert_eq!(empty, "[]");

        let x = serde_json::to_string(&Axes::X).unwrap();
        assert_eq!(x, r#"["X"]"#);

        let all = serde_json::to_string(&Axes::all()).unwrap();
        assert_eq!(all, r#"["X","Y","Z"]"#);
    }

    #[test]
    fn human_duplicate() {
        let x: Axes = serde_json::from_str(r#"["X", "X", "X", "X"]"#).unwrap();
        assert_eq!(x, Axes::X);
    }

    #[test]
    fn human_invalid() {
        // pizza is not an axis in 3D space.
        let invalid = serde_json::from_str::<Axes>(r#"["pizza"]"#);
        assert!(invalid.is_err());
    }

    #[test]
    fn non_human() {
        let empty = Axes::empty();
        let ser_empty = bincode::serialize(&empty).unwrap();
        let de_empty = bincode::deserialize(&ser_empty).unwrap();
        assert_eq!(empty, de_empty);

        let x = Axes::X;
        let ser_x = bincode::serialize(&x).unwrap();
        let de_x = bincode::deserialize(&ser_x).unwrap();
        assert_eq!(x, de_x);

        let all = Axes::all();
        let ser_all = bincode::serialize(&all).unwrap();
        let de_all = bincode::deserialize(&ser_all).unwrap();
        assert_eq!(all, de_all);
    }
}
