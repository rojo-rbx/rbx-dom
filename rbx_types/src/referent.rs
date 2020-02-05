/// A reference to a Roblox instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref(u128);

impl Ref {
    pub fn new() -> Self {
        Ref(rand::random())
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
                serializer.serialize_str(&format!("{:x}", self.0))
            } else {
                serializer.serialize_u128(self.0)
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
            Ok(Ref(value))
        }

        fn visit_str<E: Error>(self, ref_str: &str) -> Result<Self::Value, E> {
            let ref_value = u128::from_str_radix(ref_str, 16).map_err(E::custom)?;
            Ok(Ref(ref_value))
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
