use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    instance::RbxInstance,
};

/// Represents an instance that is rooted in a tree.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootedRbxInstance {
    #[serde(flatten)]
    pub(crate) instance: RbxInstance,

    /// The unique ID of the instance
    pub(crate) id: RbxId,

    /// All of the children of this instance. Order is relevant to preserve!
    pub(crate) children: Vec<RbxId>,

    /// The parent of the instance, if there is one.
    pub(crate) parent: Option<RbxId>,
}

impl RootedRbxInstance {
    pub(crate) fn new(instance: RbxInstance) -> RootedRbxInstance {
        RootedRbxInstance {
            instance,
            id: RbxId::new(),
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

impl std::ops::Deref for RootedRbxInstance {
    type Target = RbxInstance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}