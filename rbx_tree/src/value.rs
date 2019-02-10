use std::fmt;

use serde_derive::{Serialize, Deserialize};
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

/// Represents a value that can be assigned to the properties of an instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    String {
        value: String,
    },

    #[serde(rename_all = "PascalCase")]
    BinaryString {
        value: Vec<u8>,
    },

    #[serde(rename_all = "PascalCase")]
    Bool {
        value: bool,
    },

    #[serde(rename_all = "PascalCase")]
    Int32 {
        value: i32,
    },

    #[serde(rename_all = "PascalCase")]
    Float32 {
        value: f32,
    },

    #[serde(rename_all = "PascalCase")]
    Enum {
        value: u32,
    },

    #[serde(rename_all = "PascalCase")]
    Vector3 {
        value: [f32; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector2 {
        value: [f32; 2],
    },

    #[serde(rename_all = "PascalCase")]
    Color3 {
        value: [f32; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Color3uint8 {
        value: [u8; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector3int16 {
        value: [i16; 3],
    },

    #[serde(rename_all = "PascalCase")]
    Vector2int16 {
        value: [i16; 2],
    },

    #[serde(rename_all = "PascalCase")]
    CFrame {
        value: [f32; 12],
    },

    #[serde(rename_all = "PascalCase")]
    PhysicalProperties {
        value: Option<PhysicalProperties>,
    }
}

/// Represents possible custom physical properties on a `BasePart`.
///
/// Currently a stub.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalProperties;

fn deserialize_value<'de, D>(deserializer: D) -> Result<RbxValue, D::Error>
where
    D: Deserializer<'de>,
{
    struct RbxValueVisitor;

    impl<'de> Visitor<'de> for RbxValueVisitor {
        type Value = RbxValue;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(RbxValue::String {
                value: value.to_owned(),
            })
        }

        fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(RbxValueVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_value")] RbxValue);

    #[test]
    fn explicit() {
        let input = r#"
            {
                "Type": "String",
                "Value": "Hello"
            }
        "#;

        let value: Wrapper = serde_json::from_str(input).unwrap();

        assert_eq!(value.0, RbxValue::String {
            value: String::from("Hello"),
        });
    }

    #[test]
    fn implicit_string() {
        let input = r#"
            "Hello"
        "#;

        let value: Wrapper = serde_json::from_str(input).unwrap();

        assert_eq!(value.0, RbxValue::String {
            value: String::from("Hello"),
        });
    }
}