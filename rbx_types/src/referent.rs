use std::num::NonZeroU128;

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
    pub fn none() -> Self {
        Ref(None)
    }

    /// Tells whether this `Ref` points to something.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Tells whether this `Ref` points to nothing.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn value(&self) -> u128 {
        match self.0 {
            Some(value) => value.get(),
            None => 0,
        }
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

    impl<'de> Visitor<'de> for RefVisitor {
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

    #[test]
    fn size() {
        assert_eq!(std::mem::size_of::<Ref>(), std::mem::size_of::<u128>());
    }
}
