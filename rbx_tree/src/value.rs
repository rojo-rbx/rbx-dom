use serde_derive::{Serialize, Deserialize};

// TODO: Custom serialize/deserialize to encode literals

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
        value: [f64; 3],
    },
}