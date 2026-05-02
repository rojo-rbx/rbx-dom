// Refs are random, and so implementing Default doesn't really make sense.
#![allow(clippy::new_without_default)]

use std::fmt;
use std::num::NonZeroU128;
use std::str::FromStr;

/// A universally unique reference to a Roblox instance.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SomeRef(NonZeroU128);

impl SomeRef {
    /// Generate a new random `SomeRef`.
    #[inline]
    pub fn new_random() -> Self {
        Self(rand::random())
    }

    /// Construct a `SomeRef`.
    #[inline]
    pub const fn new(value: u128) -> Option<Self> {
        match NonZeroU128::new(value) {
            Some(value) => Some(SomeRef(value)),
            None => None,
        }
    }

    #[inline]
    pub const fn to_optional_ref(&self) -> OptionalRef {
        OptionalRef(Some(*self))
    }

    const fn value(&self) -> u128 {
        self.0.get()
    }
}

/// A universally unique, optional reference to a Roblox instance.
/// This is a type alias, the type has been renamed to `OptionalRef`.
pub type Ref = OptionalRef;

/// A universally unique, optional reference to a Roblox instance.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptionalRef(Option<SomeRef>);

impl OptionalRef {
    /// Generate a new random non-nil `Ref`.
    #[inline]
    pub fn new() -> Self {
        OptionalRef(Some(SomeRef::new_random()))
    }

    /// Construct a `Ref` that points to nothing.
    #[inline]
    pub const fn none() -> Self {
        OptionalRef(None)
    }

    /// Construct a `Ref` that points to something.
    ///
    /// ## Panics
    /// Panics if `value` is 0. Use the Ref::none()
    /// constructor instead to create a `Ref` that
    /// points to nothing.
    #[inline]
    pub const fn some(value: u128) -> Self {
        OptionalRef(Some(SomeRef::new(value).expect("Ref value is 0")))
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
    pub const fn to_some_ref(&self) -> Option<SomeRef> {
        self.0
    }

    const fn new_value(value: u128) -> Self {
        OptionalRef(SomeRef::new(value))
    }

    const fn value(&self) -> u128 {
        match self.0 {
            Some(value) => value.value(),
            None => 0,
        }
    }
}

impl fmt::Display for OptionalRef {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:032x}", self.value())
    }
}
impl fmt::Display for SomeRef {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:032x}", self.value())
    }
}

impl FromStr for OptionalRef {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = u128::from_str_radix(input, 16)?;

        Ok(OptionalRef::new_value(value))
    }
}
impl FromStr for SomeRef {
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

impl From<Option<SomeRef>> for OptionalRef {
    fn from(value: Option<SomeRef>) -> Self {
        OptionalRef(value)
    }
}
impl From<SomeRef> for OptionalRef {
    fn from(value: SomeRef) -> Self {
        value.to_optional_ref()
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

    impl Serialize for SomeRef {
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

    struct SomeRefVisitor;

    impl Visitor<'_> for SomeRefVisitor {
        type Value = SomeRef;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a non-nil Roblox referent")
        }

        fn visit_u128<E: Error>(self, value: u128) -> Result<Self::Value, E> {
            SomeRef::new(value).ok_or_else(|| E::custom("SomeRef value is 0"))
        }

        fn visit_str<E: Error>(self, ref_str: &str) -> Result<Self::Value, E> {
            let ref_value = u128::from_str_radix(ref_str, 16).map_err(E::custom)?;
            SomeRef::new(ref_value).ok_or_else(|| E::custom("SomeRef value is 0"))
        }
    }

    impl<'de> Deserialize<'de> for SomeRef {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(SomeRefVisitor)
            } else {
                deserializer.deserialize_u128(SomeRefVisitor)
            }
        }
    }

    impl Serialize for OptionalRef {
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

    struct OptionalRefVisitor;

    impl Visitor<'_> for OptionalRefVisitor {
        type Value = OptionalRef;

        fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "a Roblox referent")
        }

        fn visit_u128<E: Error>(self, value: u128) -> Result<Self::Value, E> {
            Ok(OptionalRef::new_value(value))
        }

        fn visit_str<E: Error>(self, ref_str: &str) -> Result<Self::Value, E> {
            let ref_value = u128::from_str_radix(ref_str, 16).map_err(E::custom)?;
            Ok(OptionalRef::new_value(ref_value))
        }
    }

    impl<'de> Deserialize<'de> for OptionalRef {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(OptionalRefVisitor)
            } else {
                deserializer.deserialize_u128(OptionalRefVisitor)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            OptionalRef::none().to_string(),
            "00000000000000000000000000000000"
        );

        let thirty = OptionalRef::new_value(30);
        assert_eq!(thirty.to_string(), "0000000000000000000000000000001e");

        let max = OptionalRef::new_value(u128::MAX);
        assert_eq!(max.to_string(), "ffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            OptionalRef::from_str("00000000000000000000000000000000").unwrap(),
            OptionalRef::none()
        );

        assert_eq!(
            OptionalRef::from_str("00000000300000e00f00000000000001").unwrap(),
            OptionalRef::new_value(14855284604576099720297971713)
        );

        assert_eq!(
            OptionalRef::from_str("ffffffffffffffffffffffffffffffff").unwrap(),
            OptionalRef::new_value(u128::MAX)
        );
    }

    #[test]
    fn size() {
        assert_eq!(
            std::mem::size_of::<OptionalRef>(),
            std::mem::size_of::<u128>()
        );
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human_none() {
        let value = OptionalRef::none();

        let ser = serde_json::to_string(&value).unwrap();
        assert_eq!(ser, "\"00000000000000000000000000000000\"");

        let de: OptionalRef = serde_json::from_str(&ser).unwrap();

        assert_eq!(value, de);
    }

    #[test]
    fn human() {
        let value = OptionalRef::new();

        let ser = serde_json::to_string(&value).unwrap();
        let de = serde_json::from_str(&ser).unwrap();

        assert_eq!(value, de);
    }

    #[test]
    fn non_human() {
        let value = OptionalRef::new();

        let ser = bincode::serialize(&value).unwrap();
        let de = bincode::deserialize(&ser).unwrap();

        assert_eq!(value, de);
    }
}
