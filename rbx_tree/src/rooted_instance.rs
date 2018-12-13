use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    instance::RbxInstance,
};

/// Represents an instance that is rooted in a tree.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RootedRbxInstance {
    #[serde(flatten)]
    instance: RbxInstance,

    /// The unique ID of the instance
    id: RbxId,

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

    pub(crate) fn clone_without_relations(&self, new_id: RbxId) -> RootedRbxInstance {
        RootedRbxInstance {
            instance: self.instance.clone(),
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

impl Clone for RootedRbxInstance {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl std::ops::Deref for RootedRbxInstance {
    type Target = RbxInstance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl std::ops::DerefMut for RootedRbxInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}