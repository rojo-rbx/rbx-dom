use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    value::RbxValue,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RbxInstanceProperties {
    /// Maps to the `Name` property on Instance.
    pub name: String,

    /// Maps to the `ClassName` property on Instance.
    pub class_name: String,

    /// Contains all other properties of an Instance.
    pub properties: HashMap<String, RbxValue>,
}

/// Represents an instance that is rooted in a tree.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RbxInstance {
    #[serde(flatten)]
    properties: RbxInstanceProperties,

    /// The unique ID of the instance
    id: RbxId,

    /// All of the children of this instance. Order is relevant to preserve!
    pub(crate) children: Vec<RbxId>,

    /// The parent of the instance, if there is one.
    pub(crate) parent: Option<RbxId>,
}

impl RbxInstance {
    pub(crate) fn new(properties: RbxInstanceProperties) -> RbxInstance {
        RbxInstance {
            properties,
            id: RbxId::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub(crate) fn clone_without_relations(&self, new_id: RbxId) -> RbxInstance {
        RbxInstance {
            properties: self.properties.clone(),
            id: new_id,
            parent: None,
            children: Vec::new(),
        }
    }

    /// Returns the unique ID associated with the rooted instance.
    pub fn get_id(&self) -> RbxId {
        self.id
    }

    /// Returns the ID of the parent of this instance, if it has a parent.
    pub fn get_parent_id(&self) -> Option<RbxId> {
        self.parent
    }

    /// Returns a list of the IDs of the children of this instance.
    pub fn get_children_ids(&self) -> &[RbxId] {
        &self.children
    }
}

impl Clone for RbxInstance {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl std::ops::Deref for RbxInstance {
    type Target = RbxInstanceProperties;

    fn deref(&self) -> &Self::Target {
        &self.properties
    }
}

impl std::ops::DerefMut for RbxInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.properties
    }
}