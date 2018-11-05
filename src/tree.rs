use std::collections::HashMap;

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
    inner: RbxInstance,

    /// The unique ID of the instance
    id: RbxId,

    /// All of the children of this instance. Order is relevant to preserve!
    children: Vec<RbxId>,

    /// The parent of the instance, if there is one.
    parent: Option<RbxId>,
}

impl RootedRbxInstance {
    fn new(instance: RbxInstance, parent: RbxId) -> RootedRbxInstance {
        RootedRbxInstance {
            inner: instance,
            id: RbxId::new(),
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    fn new_without_parent(instance: RbxInstance) -> RootedRbxInstance {
        RootedRbxInstance {
            inner: instance,
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
        &self.inner
    }
}

/// Represents a tree containing one or more rooted instances.
///
/// Rooted instances are described by
/// [RootedRbxInstance](struct.RootedRbxInstance.html).
#[derive(Debug, Serialize, Deserialize)]
pub struct RbxTree {
    instances: HashMap<RbxId, RootedRbxInstance>,
    root_id: RbxId,
}

impl RbxTree {
    pub fn new(root: RbxInstance) -> RbxTree {
        let root = RootedRbxInstance::new_without_parent(root);
        let root_id = root.id;

        let mut instances = HashMap::new();
        instances.insert(root_id, root);

        RbxTree {
            instances,
            root_id: root_id,
        }
    }

    pub fn get_root_id(&self) -> RbxId {
        self.root_id
    }

    pub fn get_instance(&self, id: RbxId) -> Option<&RootedRbxInstance> {
        self.instances.get(&id)
    }

    pub fn get_instance_mut(&mut self, id: RbxId) -> Option<&mut RootedRbxInstance> {
        self.instances.get_mut(&id)
    }

    pub fn insert_tree(&mut self, mut tree: RbxTree, new_parent_id: RbxId) {
        let mut to_visit = vec![(tree.root_id, new_parent_id)];

        loop {
            let (id, parent_id) = match to_visit.pop() {
                Some(id) => id,
                None => break,
            };

            let mut new_child = tree.instances.remove(&id).unwrap();
            new_child.parent = Some(parent_id);
            new_child.children.clear();

            for child in &new_child.children {
                to_visit.push((*child, id));
            }

            self.insert_instance_internal(new_child);
        }
    }

    fn insert_instance_internal(&mut self, instance: RootedRbxInstance) {
        let parent_id = instance.parent
            .expect("Cannot insert_instance_internal with an instance without a parent");

        {
            let parent = self.instances.get_mut(&parent_id)
                .expect("Cannot insert_instance_internal into an instance not in this tree");
            parent.children.push(instance.get_id());
        }

        self.instances.insert(instance.get_id(), instance);
    }

    pub fn insert_instance(&mut self, instance: RbxInstance, parent_id: RbxId) -> RbxId {
        let tree_instance = RootedRbxInstance::new(instance, parent_id);
        let id = tree_instance.get_id();

        self.insert_instance_internal(tree_instance);

        id
    }

    /// Given an ID, remove the instance from the tree with that ID, along with
    /// all of its descendants.
    pub fn unroot(&mut self, root_id: RbxId) -> Option<RbxTree> {
        let mut ids_to_visit = vec![root_id];
        let mut new_tree_instances = HashMap::new();

        let parent_id = match self.instances.get(&root_id) {
            Some(instance) => instance.parent,
            None => return None,
        };

        if let Some(parent_id) = parent_id {
            let mut parent = self.get_instance_mut(parent_id).unwrap();
            let index = parent.children.iter().position(|&id| id == root_id).unwrap();

            parent.children.remove(index);
        }

        loop {
            let id = match ids_to_visit.pop() {
                Some(id) => id,
                None => break,
            };

            match self.instances.get(&id) {
                Some(instance) => ids_to_visit.extend_from_slice(&instance.children),
                None => continue,
            }

            let instance = self.instances.remove(&id).unwrap();
            new_tree_instances.insert(id, instance);
        }

        Some(RbxTree {
            instances: new_tree_instances,
            root_id,
        })
    }

    /// Returns an iterator over all of the descendants of the given instance by
    /// ID.
    pub fn descendants(&self, id: RbxId) -> Descendants {
        Descendants {
            tree: self,
            ids_to_visit: vec![id],
        }
    }
}

impl Clone for RbxTree {
    fn clone(&self) -> RbxTree {
        unimplemented!()
    }
}

pub struct Descendants<'a> {
    tree: &'a RbxTree,
    ids_to_visit: Vec<RbxId>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a RootedRbxInstance;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let id = match self.ids_to_visit.pop() {
                Some(id) => id,
                None => break,
            };

            match self.tree.get_instance(id) {
                Some(instance) => {
                    for child_id in &instance.children {
                        self.ids_to_visit.push(*child_id);
                    }

                    return Some(instance);
                },
                None => continue,
            }
        }

        None
    }
}