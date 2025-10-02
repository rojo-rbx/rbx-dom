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

impl From<CustomPhysicalProperties> for PhysicalProperties {
    fn from(value: CustomPhysicalProperties) -> Self {
        Self::Custom(value)
    }
}

/// Custom physics properties that can be given to parts.
///
/// Documentation for what these do can be found on the
/// [`PhysicalProperties`][PhysicalProperties] DevHub documentation.
///
/// [PhysicalProperties]: https://developer.roblox.com/en-us/api-reference/datatype/PhysicalProperties
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[non_exhaustive]
pub struct CustomPhysicalProperties {
    density: f32,
    friction: f32,
    elasticity: f32,
    friction_weight: f32,
    elasticity_weight: f32,
    acoustic_absorption: Option<f32>,
}

impl CustomPhysicalProperties {
    pub fn new(
        density: f32,
        friction: f32,
        elasticity: f32,
        friction_weight: f32,
        elasticity_weight: f32,
        acoustic_absorption: Option<f32>,
    ) -> Self {
        Self {
            density,
            friction,
            elasticity,
            friction_weight,
            elasticity_weight,
            acoustic_absorption,
        }
    }

    #[inline]
    pub fn density(&self) -> f32 {
        self.density
    }

    #[inline]
    pub fn set_density(&mut self, density: f32) {
        self.density = density
    }

    #[inline]
    pub fn friction(&self) -> f32 {
        self.friction
    }

    #[inline]
    pub fn set_friction(&mut self, friction: f32) {
        self.friction = friction
    }

    #[inline]
    pub fn elasticity(&self) -> f32 {
        self.elasticity
    }

    #[inline]
    pub fn set_elasticity(&mut self, elasticity: f32) {
        self.elasticity = elasticity
    }

    #[inline]
    pub fn friction_weight(&self) -> f32 {
        self.friction_weight
    }

    #[inline]
    pub fn set_friction_weight(&mut self, friction_weight: f32) {
        self.friction_weight = friction_weight
    }

    #[inline]
    pub fn elasticity_weight(&self) -> f32 {
        self.elasticity_weight
    }

    #[inline]
    pub fn set_elasticity_weight(&mut self, elasticity_weight: f32) {
        self.elasticity_weight = elasticity_weight
    }

    #[inline]
    pub fn acoustic_absorption(&self) -> Option<f32> {
        self.acoustic_absorption
    }

    #[inline]
    pub fn set_acoustic_absorption(&mut self, acoustic_absorption: Option<f32>) {
        self.acoustic_absorption = acoustic_absorption
    }
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

    impl From<TaggedPhysicalProperties> for PhysicalProperties {
        fn from(value: TaggedPhysicalProperties) -> Self {
            match value {
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
            acoustic_absorption: None,
        });

        let ser = serde_json::to_string(&custom).unwrap();
        assert_eq!(ser, "{\"density\":1.0,\"friction\":0.5,\"elasticity\":0.0,\"frictionWeight\":6.0,\"elasticityWeight\":5.0,\"acousticAbsorption\":null}");

        let de: PhysicalProperties = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, custom);
    }

    #[test]
    fn json_custom_acoustic() {
        let custom = PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 1.0,
            friction: 0.5,
            elasticity: 0.0,
            elasticity_weight: 5.0,
            friction_weight: 6.0,
            acoustic_absorption: Some(1337.0),
        });

        let ser = serde_json::to_string(&custom).unwrap();
        assert_eq!(ser, "{\"density\":1.0,\"friction\":0.5,\"elasticity\":0.0,\"frictionWeight\":6.0,\"elasticityWeight\":5.0,\"acousticAbsorption\":1337.0}");

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
            acoustic_absorption: None,
        });

        let ser = bincode::serialize(&custom).unwrap();
        let de: PhysicalProperties = bincode::deserialize(&ser).unwrap();

        assert_eq!(de, custom);
    }
}
