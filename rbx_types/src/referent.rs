// Refs are random, and so implementing Default doesn't really make sense.
#![allow(clippy::new_without_default)]

use std::fmt;
use std::num::NonZeroU128;
use std::str::FromStr;

/// A universally unique reference to a Roblox instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref(NonZeroU128);

impl Ref {
    /// Generate a new random `Ref`.
    #[inline]
    pub fn new_random() -> Self {
        Self(rand::random())
    }

    /// Construct a `Ref`.
    #[inline]
    pub const fn new(value: u128) -> Option<Self> {
        match NonZeroU128::new(value) {
            Some(value) => Some(Ref(value)),
            None => None,
        }
    }

    const fn value(&self) -> u128 {
        self.0.get()
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:032x}", self.value())
    }
}

impl FromStr for Ref {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = u128::from_str_radix(input, 16)?;

        Self::new(value).ok_or_else(|| {
            // from_str_radix does not support NonZeroU128 so
            // generate a ParseIntError with IntErrorKind::Zero
            "0".parse::<NonZeroU128>().err().unwrap()
        })
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl Serialize for Ref {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&format!("{:032x}", self.value()))
            } else {
                serializer.serialize_u128(self.value())
            }
        }
    }

    struct RefVisitor;

    impl Visitor<'_> for RefVisitor {
        type Value = Ref;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a non-nil Roblox referent")
        }

        fn visit_u128<E: Error>(self, value: u128) -> Result<Self::Value, E> {
            Ref::new(value).ok_or_else(|| E::custom("Ref value is 0"))
        }

        fn visit_str<E: Error>(self, ref_str: &str) -> Result<Self::Value, E> {
            let ref_value = u128::from_str_radix(ref_str, 16).map_err(E::custom)?;
            Ref::new(ref_value).ok_or_else(|| E::custom("Ref value is 0"))
        }
    }

    impl<'de> Deserialize<'de> for Ref {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(RefVisitor)
            } else {
                deserializer.deserialize_u128(RefVisitor)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        let thirty = Ref::new(30).unwrap();
        assert_eq!(thirty.to_string(), "0000000000000000000000000000001e");

        let max = Ref::new(u128::MAX).unwrap();
        assert_eq!(max.to_string(), "ffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Ref::from_str("00000000300000e00f00000000000001").unwrap(),
            Ref::new(14855284604576099720297971713).unwrap()
        );

        assert_eq!(
            Ref::from_str("ffffffffffffffffffffffffffffffff").unwrap(),
            Ref::new(u128::MAX).unwrap()
        );
    }

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<Ref>(), std::mem::size_of::<u128>());
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human() {
        let value = Ref::new_random();

        let ser = serde_json::to_string(&value).unwrap();
        let de = serde_json::from_str(&ser).unwrap();

        assert_eq!(value, de);
    }

    #[test]
    fn non_human() {
        let value = Ref::new_random();

        let ser = bincode::serialize(&value).unwrap();
        let de = bincode::deserialize(&ser).unwrap();

        assert_eq!(value, de);
    }
}
