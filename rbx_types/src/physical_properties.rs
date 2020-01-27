#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents the physical properties that parts can have.
///
/// Equivalent to Roblox's [`PhysicalProperties`][PhysicalProperties] type, with
/// the difference that `Default` is a variant here, instead of a hidden state
/// that `PhysicalProperties` can have.
///
/// [PhysicalProperties]: https://developer.roblox.com/en-us/api-reference/datatype/PhysicalProperties
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhysicalProperties {
    Default,
    Custom(CustomPhysicalProperties),
}

/// Custom physics properties that can be given to parts.
///
/// Documentation for what these do can be found on the
/// [`PhysicalProperties`][PhysicalProperties] DevHub documentation.
///
/// [PhysicalProperties]: https://developer.roblox.com/en-us/api-reference/datatype/PhysicalProperties
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct CustomPhysicalProperties {
    pub density: f32,
    pub friction: f32,
    pub elasticity: f32,
    pub friction_weight: f32,
    pub elasticity_weight: f32,
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::fmt;

    use serde::de;

    use super::*;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    enum TaggedPhysicalProperties {
        Default,
        Custom(CustomPhysicalProperties),
    }

    impl From<PhysicalProperties> for TaggedPhysicalProperties {
        fn from(value: PhysicalProperties) -> Self {
            match value {
                PhysicalProperties::Default => TaggedPhysicalProperties::Default,
                PhysicalProperties::Custom(custom) => TaggedPhysicalProperties::Custom(custom),
            }
        }
    }

    impl Into<PhysicalProperties> for TaggedPhysicalProperties {
        fn into(self) -> PhysicalProperties {
            match self {
                TaggedPhysicalProperties::Default => PhysicalProperties::Default,
                TaggedPhysicalProperties::Custom(custom) => PhysicalProperties::Custom(custom),
            }
        }
    }

    impl Serialize for PhysicalProperties {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                match self {
                    PhysicalProperties::Default => serializer.serialize_str("Default"),
                    PhysicalProperties::Custom(custom) => custom.serialize(serializer),
                }
            } else {
                TaggedPhysicalProperties::from(*self).serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for PhysicalProperties {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct Visitor;

            impl<'de> de::Visitor<'de> for Visitor {
                type Value = PhysicalProperties;

                fn expecting(&self, out: &mut fmt::Formatter) -> fmt::Result {
                    write!(
                        out,
                        "the string \"Default\" or a CustomPhysicalProperties struct"
                    )
                }

                fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                    if value == "Default" {
                        Ok(PhysicalProperties::Default)
                    } else {
                        Err(E::invalid_value(de::Unexpected::Str(value), &self))
                    }
                }

                fn visit_map<M: de::MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
                        .map(PhysicalProperties::Custom)
                }
            }

            if deserializer.is_human_readable() {
                deserializer.deserialize_any(Visitor)
            } else {
                TaggedPhysicalProperties::deserialize(deserializer).map(Into::into)
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn json_default() {
        let default = PhysicalProperties::Default;

        let ser = serde_json::to_string(&default).unwrap();
        assert_eq!(ser, "\"Default\"");

        let de: PhysicalProperties = serde_json::from_str("\"Default\"").unwrap();
        assert_eq!(de, default);
    }

    #[test]
    fn json_custom() {
        let custom = PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 1.0,
            friction: 0.5,
            elasticity: 0.0,
            elasticity_weight: 5.0,
            friction_weight: 6.0,
        });

        let ser = serde_json::to_string(&custom).unwrap();
        assert_eq!(ser, "{\"Density\":1.0,\"Friction\":0.5,\"Elasticity\":0.0,\"FrictionWeight\":6.0,\"ElasticityWeight\":5.0}");

        let de: PhysicalProperties = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, custom);
    }

    #[test]
    fn bincode_default() {
        let default = PhysicalProperties::Default;

        let ser = bincode::serialize(&default).unwrap();
        let de: PhysicalProperties = bincode::deserialize(&ser).unwrap();

        assert_eq!(de, default);
    }

    #[test]
    fn bincode_custom() {
        let custom = PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 1.0,
            friction: 0.5,
            elasticity: 0.0,
            elasticity_weight: 5.0,
            friction_weight: 6.0,
        });

        let ser = bincode::serialize(&custom).unwrap();
        let de: PhysicalProperties = bincode::deserialize(&ser).unwrap();

        assert_eq!(de, custom);
    }
}
