use std::fmt;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess, SeqAccess};

use crate::value::RbxValue;

/// Represents a value that was deserialized that might not have full type
/// information attached.
#[derive(Debug, Clone, PartialEq)]
pub enum UnresolvedRbxValue {
    /// The type has full type information that was either declared explicitly
    /// or was inferred and unambiguous.
    Concrete(RbxValue),

    /// The type did not have type information, but the concrete type may be
    /// inferable given more type information.
    Ambiguous(AmbiguousRbxValue),
}

impl From<RbxValue> for UnresolvedRbxValue {
    fn from(value: RbxValue) -> UnresolvedRbxValue {
        UnresolvedRbxValue::Concrete(value)
    }
}

/// Represents a value that doesn't have explicit type information attached to
/// it. Given more reflection information, it should be possible to recover the
/// exact type of this value.
#[derive(Debug, Clone, PartialEq)]
pub enum AmbiguousRbxValue {
    /// One of String or Enum
    String(String),

    /// One of Float32, Int32, or Enum
    Float1(f64),

    /// One of Vector2, Vector2int16, or UDim
    Float2(f64, f64),

    /// One of Vector3, Vector3int16, Color3, or Color3uint8
    Float3(f64, f64, f64),
}

impl Serialize for UnresolvedRbxValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            UnresolvedRbxValue::Ambiguous(ambiguous) => {
                match ambiguous {
                    AmbiguousRbxValue::String(value) => serializer.serialize_str(&value),
                    AmbiguousRbxValue::Float1(value) => serializer.serialize_f64(*value),
                    AmbiguousRbxValue::Float2(x, y) => (x, y).serialize(serializer),
                    AmbiguousRbxValue::Float3(x, y, z) => (x, y, z).serialize(serializer),
                }
            },
            UnresolvedRbxValue::Concrete(concrete) => concrete.serialize(serializer),
        }
    }
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

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnresolvedRbxValue::Concrete(RbxValue::Bool { value }))
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

                // The value is either a Float2, a Float3, a UDim, or a CFrame here

                let third: Option<f64> = visitor.next_element()?;
                let third = match third {
                    Some(value) => value,
                    None => {
                        return Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float2(first, second)));
                    },
                };

                // The value is either a Float3, a UDim2, or a CFrame here

                let fourth: Option<f64> = visitor.next_element()?;
                let fourth = match fourth {
                    Some(value) => value,
                    None => {
                        return Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(first, second, third)));
                    },
                };

                // The value is either a UDim2 or a CFrame here

                let fifth: Option<f64> = visitor.next_element()?;
                let fifth = match fifth {
                    Some(value) => value,
                    None => {
                        return Ok(UnresolvedRbxValue::Concrete(RbxValue::UDim2 {
                            value: (first as f32, second as i32, third as f32, fourth as i32),
                        }));
                    },
                };

                // The value must be a CFrame
                let mut value = [0.0; 12];
                value[0] = first as f32;
                value[1] = second as f32;
                value[2] = third as f32;
                value[3] = fourth as f32;
                value[4] = fifth as f32;

                for i in 5..12 {
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
    fn implicit_quadruple() {
        let input = r#"
            [1, 2, 5, 6]
        "#;

        let value: UnresolvedRbxValue = serde_json::from_str(input).unwrap();

        assert_eq!(value, UnresolvedRbxValue::Concrete(RbxValue::UDim2 {
            value: (1.0, 2, 5.0, 6),
        }));
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

    #[test]
    fn round_trip_ambiguous() {
        let values = vec![
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String("Hello, world!".to_owned())),
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(1.0)),
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float2(3.0, 2.0)),
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(5.0, 7.0, 21.0)),
        ];

        let round_tripped: Vec<UnresolvedRbxValue> = values
            .iter()
            .map(|value| serde_json::to_string(value).unwrap())
            .map(|str| serde_json::from_str(&str).unwrap())
            .collect();

        assert_eq!(values, round_tripped);
    }

    #[test]
    fn round_trip_concrete() {
        use crate::value::PhysicalProperties;

        let values = vec![
            RbxValue::BinaryString { value: vec![0, 1, 2, 3] },
            RbxValue::Bool { value: true },
            RbxValue::CFrame { value: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0] },
            RbxValue::Color3 { value: [1.0, 2.0, 3.0] },
            RbxValue::Color3uint8 { value: [0, 8, 129] },
            RbxValue::Content { value: "some content path".to_owned() },
            RbxValue::Float32 { value: 5.0 },
            RbxValue::Int32 { value: 50 },
            RbxValue::PhysicalProperties {
                value: Some(PhysicalProperties {
                    density: 1.0,
                    friction: 2.0,
                    elasticity: 3.0,
                    friction_weight: 4.0,
                    elasticity_weight: 5.0,
                })
            },
            RbxValue::Ref { value: None },
            RbxValue::String { value: "pretty cool string, huh?".to_owned() },
            RbxValue::UDim { value: (1.0, 20) },
            RbxValue::UDim2 { value: (2.0, 50, 3.0, 100) },
            RbxValue::Vector2 { value: [516.0, 792.0] },
            RbxValue::Vector2int16 { value: [12345, 5674] },
            RbxValue::Vector3 { value: [9876.0, 65325.0, 12331.0] },
            RbxValue::Vector3int16 { value: [5234, 2361, 12355] },
            RbxValue::Enum { value: 0 },
        ];

        let unresolved: Vec<UnresolvedRbxValue> = values
            .iter()
            .map(|value| UnresolvedRbxValue::Concrete(value.clone()))
            .collect();

        let round_tripped: Vec<UnresolvedRbxValue> = unresolved
            .iter()
            .map(|value| serde_json::to_string(value).unwrap())
            .map(|str| serde_json::from_str(&str).unwrap())
            .collect();

        assert_eq!(unresolved, round_tripped);
    }
}