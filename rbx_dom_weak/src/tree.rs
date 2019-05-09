use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    instance::{RbxInstance, RbxInstanceProperties},
};

/// Represents a tree containing rooted instances.
///
/// Rooted instances are described by
/// [RbxInstance](struct.RbxInstance.html) and have an ID, children,
/// and a parent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RbxTree {
    instances: HashMap<RbxId, RbxInstance>,
    root_id: RbxId,
}

impl RbxTree {
    /// Construct a new `RbxTree` with its root instance constructed using the
    /// given properties.
    pub fn new(root_properties: RbxInstanceProperties) -> RbxTree {
        let rooted_root = RbxInstance::new(root_properties);
        let root_id = rooted_root.get_id();

        let mut instances = HashMap::new();
        instances.insert(root_id, rooted_root);

        RbxTree {
            instances,
            root_id: root_id,
        }
    }

    /// Returns the ID of the root instance in the tree, which can be used
    /// alongside `get_instance` and friends.
    pub fn get_root_id(&self) -> RbxId {
        self.root_id
    }

    /// Returns an iterator over all IDs in the tree.
    pub fn iter_all_ids(&self) -> impl Iterator<Item=RbxId> + '_ {
        self.instances.keys().cloned()
    }

    /// Returns the instance with the given ID if it's contained in this tree.
    pub fn get_instance(&self, id: RbxId) -> Option<&RbxInstance> {
        self.instances.get(&id)
    }

    /// Returns mutable access to the instance with the given ID if it's
    /// contained in this tree.
    pub fn get_instance_mut(&mut self, id: RbxId) -> Option<&mut RbxInstance> {
        self.instances.get_mut(&id)
    }

    // TODO: Make this method public once it's ironed out
    #[allow(unused)]
    fn move_instance(&mut self, source_id: RbxId, dest_tree: &mut RbxTree, dest_parent_id: RbxId) {
        let mut to_visit = vec![(source_id, dest_parent_id)];

        loop {
            let (id, parent_id) = match to_visit.pop() {
                Some(id) => id,
                None => break,
            };

            let mut instance = self.instances.remove(&id).unwrap();
            instance.parent = Some(parent_id);
            instance.children.clear();

            for child in &instance.children {
                to_visit.push((*child, id));
            }

            dest_tree.insert_instance_internal(instance);
        }
    }

    fn insert_instance_internal(&mut self, instance: RbxInstance) {
        let parent_id = instance.parent
            .expect("Can not use insert_instance_internal on instances with no parent");

        {
            let parent = self.instances.get_mut(&parent_id)
                .expect("Cannot insert_instance_internal into an instance not in this tree");
            parent.children.push(instance.get_id());
        }

        self.instances.insert(instance.get_id(), instance);
    }

    /// Inserts a new instance with the given properties into the tree, putting it
    /// under the instance with the given ID.
    ///
    /// ## Panics
    /// Panics if the given ID does not refer to an instance in this tree.
    pub fn insert_instance(&mut self, properties: RbxInstanceProperties, parent_id: RbxId) -> RbxId {
        let mut tree_instance = RbxInstance::new(properties);
        tree_instance.parent = Some(parent_id);

        let id = tree_instance.get_id();

        self.insert_instance_internal(tree_instance);

        id
    }

    /// Given an ID, remove the instance from the tree with that ID, along with
    /// all of its descendants.
    pub fn remove_instance(&mut self, root_id: RbxId) -> Option<RbxTree> {
        if self.root_id == root_id {
            panic!("Cannot remove root ID from tree!");
        }

        let mut ids_to_visit = vec![root_id];
        let mut new_tree_instances = HashMap::new();

        let parent_id = match self.instances.get(&root_id) {
            Some(instance) => instance.parent,
            None => return None,
        };

        if let Some(parent_id) = parent_id {
            let parent = self.get_instance_mut(parent_id).unwrap();
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
            root_id: root_id,
        })
    }

    /// Returns an iterator over all of the descendants of the given instance by
    /// ID.
    pub fn descendants(&self, id: RbxId) -> Descendants<'_> {
        Descendants {
            tree: self,
            ids_to_visit: vec![id],
        }
    }
}

/// An iterator over all descendants of an instance in an [`RbxTree`]. Returned
/// by [`RbxTree::descendants`].
///
/// [`RbxTree`]: struct.RbxTree.html
/// [`RbxTree::descendants`]: struct.RbxTree.html#method.descendants
pub struct Descendants<'a> {
    tree: &'a RbxTree,
    ids_to_visit: Vec<RbxId>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a RbxInstance;

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