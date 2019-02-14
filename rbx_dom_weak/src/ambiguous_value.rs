use std::fmt;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess, SeqAccess};

use crate::value::RbxValue;

/// Represents a value that was deserialized that might not have full type
/// information attached.
#[derive(Debug, PartialEq)]
pub enum UnresolvedRbxValue {
    /// The type has full type information that was either declared explicitly
    /// or was inferred and unambiguous.
    Concrete(RbxValue),

    /// The type did not have type information, but the concrete type may be
    /// inferable given more type information.
    Ambiguous(AmbiguousRbxValue),
}

// Dubious? Untagged? Ambiguous?
/// Represents a value that doesn't have explicit type information attached to
/// it. Given more reflection information, it should be possible to recover the
/// exact type of this value.
#[derive(Debug, PartialEq)]
pub enum AmbiguousRbxValue {
    /// One of String or Enum
    String(String),

    /// One of Float32, Int32, or Enum
    Float1(f64),

    /// One of Vector2 or Vector2int16
    Float2(f64, f64),

    /// One of Vector3, Vector3int16, Color3, or Color3uint8
    Float3(f64, f64, f64),
}

impl<'de> Deserialize<'de> for UnresolvedRbxValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = UnresolvedRbxValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Roblox value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(value.to_owned())))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(value)))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(value as f64)))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(value as f64)))
            }

            fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let first: f64 = visitor.next_element()?.ok_or_else(||
                    de::Error::invalid_length(0, &"sequence of length 2, 3, or 12")
                )?;

                let second: f64 = visitor.next_element()?.ok_or_else(||
                    de::Error::invalid_length(1, &"sequence of length 2, 3, or 12")
                )?;

                // The value is either a Float2, a Float3, or a CFrame here

                let third: Option<f64> = visitor.next_element()?;
                let third = match third {
                    Some(value) => value,
                    None => {
                        return Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float2(first, second)));
                    },
                };

                // The value is either a Float3 or a CFrame here

                let fourth: Option<f64> = visitor.next_element()?;
                let fourth = match fourth {
                    Some(value) => value,
                    None => {
                        return Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(first, second, third)));
                    },
                };

                // The value must be a CFrame
                let mut value = [0.0; 12];
                value[0] = first as f32;
                value[1] = second as f32;
                value[2] = third as f32;
                value[3] = fourth as f32;

                for i in 4..12 {
                    value[i] = visitor.next_element()?.ok_or_else(||
                        de::Error::invalid_length(i, &"sequence of length 2, 3, or 12")
                    )?;
                }

                Ok(UnresolvedRbxValue::Concrete(RbxValue::CFrame {
                    value,
                }))
            }

            fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let inner = Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))?;

                Ok(UnresolvedRbxValue::Concrete(inner))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit() {
        let input = r#"
            {
                "Type": "String",
                "Value": "Hello"
            }
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Concrete(RbxValue::String {
            value: String::from("Hello"),
        }));
    }

    #[test]
    fn implicit_string() {
        let input = r#"
            "Hello"
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(String::from("Hello"))));
    }

    #[test]
    fn implicit_float() {
        let input = r#"
            5.0
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(5.0)));
    }

    #[test]
    fn implicit_integer() {
        let input = r#"
            5
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(5.0)));
    }

    #[test]
    fn implicit_pair() {
        let input = r#"
            [1, 2]
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float2(1.0, 2.0)));
    }

    #[test]
    fn implicit_triple() {
        let input = r#"
            [1, 2, 5]
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(1.0, 2.0, 5.0)));
    }

    #[test]
    fn implicit_cframe() {
        let input = r#"
            [
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
                10, 11, 12
            ]
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Concrete(RbxValue::CFrame {
            value: [
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
                10.0, 11.0, 12.0,
            ],
        }));
    }
}