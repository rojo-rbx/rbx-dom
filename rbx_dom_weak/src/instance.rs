use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    value::RbxValue,
};

/// The properties associated with a Roblox Instance that might not exist yet.
///
/// To construct a real instance with an ID and children, insert an
/// `RbxInstanceProperties` object into an existing [`RbxTree`] with
/// [`RbxTree::insert_instance`] or by creating a new tree with it as the root
/// using [`RbxTree::new`].
///
/// [`RbxTree`]: struct.RbxTree.html
/// [`RbxTree::insert_instance`]: struct.RbxTree.html#method.insert_instance
/// [`RbxTree::new`]: struct.RbxTree.html#method.new
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RbxInstanceProperties {
    /// Maps to the `Name` property on Instance.
    pub name: String,

    /// Maps to the `ClassName` property on Instance.
    pub class_name: String,

    /// Contains all other properties of the Instance.
    pub properties: HashMap<String, RbxValue>,
}

/// Represents an instance that is rooted in an [`RbxTree`]. These are always
/// returned from an existing [`RbxTree`] with a method like
/// [`RbxTree::get_instance`].
///
/// `RbxInstance` derefs to `RbxInstanceProperties` to make accessing properties
/// easier.
///
/// [`RbxTree`]: struct.RbxTree.html
/// [`RbxTree::get_instance`]: struct.RbxTree.html#method.get_instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Returns the unique ID associated with this instance.
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

    /// Re-orders children using the given key function. This sort is stable.
    ///
    /// Works the same as `Vec::sort_by_key` which is used internally.
    pub fn sort_children_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(RbxId) -> K,
        K: Ord,
    {
        self.children.sort_by_key(|&id| f(id));
    }

    /// Re-orders children using the given key function. This sort is unstable.
    ///
    /// Works the same as `Vec::sort_unstable_by_key` which is used internally.
    pub fn sort_children_unstable_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(RbxId) -> K,
        K: Ord,
    {
        self.children.sort_unstable_by_key(|&id| f(id));
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