use serde_derive::{Serialize, Deserialize};

// TODO: Custom serialize/deserialize to encode literals

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum RbxValue {
    String {
        value: String,
    },
    Number {
        value: f64,
    },
    Bool {
        value: bool,
    },
    Vector3 {
        value: [f64; 3],
    },
    Color3 {
        value: [u8; 3],
    },
}