use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    value::RbxValue,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RbxInstance {
    /// Maps to the `Name` property on Instance.
    pub name: String,

    /// Maps to the `ClassName` property on Instance.
    pub class_name: String,

    /// Contains all other properties of an Instance.
    #[serde(flatten)]
    pub properties: HashMap<String, RbxValue>,
}