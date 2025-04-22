// Refs are random, and so implementing Default doesn't really make sense.
#![allow(clippy::new_without_default)]

use std::fmt;
use std::num::NonZeroU128;
use std::str::FromStr;

/// An universally unique, optional reference to a Roblox instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref(Option<NonZeroU128>);

impl Ref {
    /// Generate a new random `Ref`.
    #[inline]
    pub fn new() -> Self {
        Ref(Some(rand::random()))
    }

    /// Generate a `Ref` that points to nothing.
    #[inline]
    pub const fn none() -> Self {
        Ref(None)
    }

    /// Tells whether this `Ref` points to something.
    #[inline]
    pub const fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Tells whether this `Ref` points to nothing.
    #[inline]
    pub const fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub const fn from_value(value: u128) -> Self {
        Ref(NonZeroU128::new(value))
    }

    fn value(&self) -> u128 {
        match self.0 {
            Some(value) => value.get(),
            None => 0,
        }
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:032x}", self.value())
    }
}

impl From<u128> for Ref {
    #[inline]
    fn from(value: u128) -> Self {
        Ref::from_value(value)
    }
}

impl FromStr for Ref {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = u128::from_str_radix(input, 16)?;

        Ok(Ref::from_value(value))
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
            write!(out, "a Roblox referent")
        }

        fn visit_u128<E: Error>(self, value: u128) -> Result<Self::Value, E> {
            Ok(Ref(NonZeroU128::new(value)))
        }

        fn visit_str<E: Error>(self, ref_str: &str) -> Result<Self::Value, E> {
            let ref_value = u128::from_str_radix(ref_str, 16).map_err(E::custom)?;
            Ok(Ref(NonZeroU128::new(ref_value)))
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
        assert_eq!(Ref::none().to_string(), "00000000000000000000000000000000");

        let thirty = Ref(NonZeroU128::new(30));
        assert_eq!(thirty.to_string(), "0000000000000000000000000000001e");

        let max = Ref(NonZeroU128::new(u128::MAX));
        assert_eq!(max.to_string(), "ffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Ref::from_str("00000000000000000000000000000000").unwrap(),
            Ref::none()
        );

        assert_eq!(
            Ref::from_str("00000000300000e00f00000000000001").unwrap(),
            Ref(NonZeroU128::new(14855284604576099720297971713))
        );

        assert_eq!(
            Ref::from_str("ffffffffffffffffffffffffffffffff").unwrap(),
            Ref(NonZeroU128::new(u128::MAX))
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
    fn human_none() {
        let value = Ref::none();

        let ser = serde_json::to_string(&value).unwrap();
        assert_eq!(ser, "\"00000000000000000000000000000000\"");

        let de: Ref = serde_json::from_str(&ser).unwrap();

        assert_eq!(value, de);
    }

    #[test]
    fn human() {
        let value = Ref::new();

        let ser = serde_json::to_string(&value).unwrap();
        let de = serde_json::from_str(&ser).unwrap();

        assert_eq!(value, de);
    }

    #[test]
    fn non_human() {
        let value = Ref::new();

        let ser = bincode::serialize(&value).unwrap();
        let de = bincode::deserialize(&ser).unwrap();

        assert_eq!(value, de);
    }
}
