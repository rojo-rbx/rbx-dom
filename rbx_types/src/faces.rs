use std::fmt;

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

/// Represents a set of zero or more faces of a cube.
///
/// ## See Also
/// * [Faces on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Faces)
#[derive(Clone, Copy, PartialEq, Eq)]
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

    const FACE_NAMES: [(Faces, &'static str); 6] = [
        (Faces::RIGHT, "Right"),
        (Faces::TOP, "Top"),
        (Faces::BACK, "Back"),
        (Faces::LEFT, "Left"),
        (Faces::BOTTOM, "Bottom"),
        (Faces::FRONT, "Front"),
    ];
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

    pub const fn from_bits(bits: u8) -> Option<Self> {
        match FaceFlags::from_bits(bits) {
            Some(flags) => Some(Self { flags }),
            None => None,
        }
    }

    #[cfg(feature = "serde")]
    fn len(self) -> usize {
        self.bits().count_ones() as usize
    }
}

impl fmt::Debug for Faces {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Faces(")?;

        let mut iter = IntoIterator::into_iter(Self::FACE_NAMES)
            .filter_map(|(face, name)| self.contains(face).then_some(name));

        if let Some(first_name) = iter.next() {
            write!(out, "{first_name}")?;
            for name in iter {
                write!(out, ", {name}")?;
            }
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

    impl Serialize for Faces {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                let mut seq = serializer.serialize_seq(Some(self.len()))?;

                if self.contains(Self::RIGHT) {
                    seq.serialize_element("Right")?;
                }

                if self.contains(Self::TOP) {
                    seq.serialize_element("Top")?;
                }

                if self.contains(Self::BACK) {
                    seq.serialize_element("Back")?;
                }

                if self.contains(Self::LEFT) {
                    seq.serialize_element("Left")?;
                }

                if self.contains(Self::BOTTOM) {
                    seq.serialize_element("Bottom")?;
                }

                if self.contains(Self::FRONT) {
                    seq.serialize_element("Front")?;
                }

                seq.end()
            } else {
                serializer.serialize_u8(self.bits())
            }
        }
    }

    struct HumanVisitor;

    impl<'de> Visitor<'de> for HumanVisitor {
        type Value = Faces;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a list of strings representing faces")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut flags = FaceFlags::empty();

            while let Some(face_str) = seq.next_element::<String>()? {
                match face_str.as_str() {
                    "Right" => flags |= FaceFlags::RIGHT,
                    "Top" => flags |= FaceFlags::TOP,
                    "Back" => flags |= FaceFlags::BACK,
                    "Left" => flags |= FaceFlags::LEFT,
                    "Bottom" => flags |= FaceFlags::BOTTOM,
                    "Front" => flags |= FaceFlags::FRONT,
                    _ => {
                        return Err(A::Error::custom(format!("invalid face '{face_str}'")));
                    }
                }
            }

            Ok(Faces { flags })
        }
    }

    impl<'de> Deserialize<'de> for Faces {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                deserializer.deserialize_seq(HumanVisitor)
            } else {
                let value = u8::deserialize(deserializer)?;

                Faces::from_bits(value)
                    .ok_or_else(|| D::Error::custom("value must a u8 bitmask of faces"))
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human_de() {
        let empty: Faces = serde_json::from_str("[]").unwrap();
        assert_eq!(empty, Faces::empty());

        let x: Faces = serde_json::from_str(r#"["Right"]"#).unwrap();
        assert_eq!(x, Faces::RIGHT);

        let all: Faces =
            serde_json::from_str(r#"["Right", "Top", "Back", "Left", "Bottom", "Front"]"#).unwrap();
        assert_eq!(all, Faces::all());
    }

    #[test]
    fn human_ser() {
        let empty = serde_json::to_string(&Faces::empty()).unwrap();
        assert_eq!(empty, "[]");

        let x = serde_json::to_string(&Faces::LEFT).unwrap();
        assert_eq!(x, r#"["Left"]"#);

        let all = serde_json::to_string(&Faces::all()).unwrap();
        assert_eq!(all, r#"["Right","Top","Back","Left","Bottom","Front"]"#);
    }

    #[test]
    fn human_duplicate() {
        let x: Faces = serde_json::from_str(r#"["Right", "Right", "Right", "Right"]"#).unwrap();
        assert_eq!(x, Faces::RIGHT);
    }

    #[test]
    fn human_invalid() {
        // calzone is not a face of a cube
        let invalid = serde_json::from_str::<Faces>(r#"["calzone"]"#);
        assert!(invalid.is_err());
    }

    #[test]
    fn non_human() {
        let empty = Faces::empty();
        let ser_empty = bincode::serialize(&empty).unwrap();
        let de_empty = bincode::deserialize(&ser_empty).unwrap();
        assert_eq!(empty, de_empty);

        let right = Faces::RIGHT;
        let ser_right = bincode::serialize(&right).unwrap();
        let de_right = bincode::deserialize(&ser_right).unwrap();
        assert_eq!(right, de_right);

        let all = Faces::all();
        let ser_all = bincode::serialize(&all).unwrap();
        let de_all = bincode::deserialize(&ser_all).unwrap();
        assert_eq!(all, de_all);
    }
}
