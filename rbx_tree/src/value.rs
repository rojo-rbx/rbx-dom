use serde_derive::{Serialize, Deserialize};

// TODO: Custom serialize/deserialize to encode literals more compactly in JSON.

/// Represents a value that can be assigned to the properties of an instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    String {
        value: String,
    },
    #[serde(rename_all = "PascalCase")]
    Bool {
        value: bool,
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
    }
}