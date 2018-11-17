use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    instance::RbxInstance,
    rooted_instance::RootedRbxInstance,
};

/// Represents a tree containing rooted instances.
///
/// Rooted instances are described by
/// [RootedRbxInstance](struct.RootedRbxInstance.html) and have an ID, children,
/// and a parent.
#[derive(Debug, Serialize, Deserialize)]
pub struct RbxTree {
    instances: HashMap<RbxId, RootedRbxInstance>,
    root_id: RbxId,
}

impl RbxTree {
    pub fn new(root: RbxInstance) -> RbxTree {
        let rooted_root = RootedRbxInstance::new(root);
        let root_id = rooted_root.get_id();

        let mut instances = HashMap::new();
        instances.insert(root_id, rooted_root);

        RbxTree {
            instances,
            root_id: root_id,
        }
    }

    pub fn get_root_id(&self) -> RbxId {
        self.root_id
    }

    pub fn get_all_ids(&self) -> impl Iterator<Item=RbxId> + '_ {
        self.instances.keys().cloned()
    }

    pub fn get_instance(&self, id: RbxId) -> Option<&RootedRbxInstance> {
        self.instances.get(&id)
    }

    pub fn get_instance_mut(&mut self, id: RbxId) -> Option<&mut RootedRbxInstance> {
        self.instances.get_mut(&id)
    }

    pub fn transplant(&mut self, source_tree: &mut RbxTree, source_id: RbxId, new_parent_id: RbxId) {
        let mut to_visit = vec![(source_id, new_parent_id)];

        loop {
            let (id, parent_id) = match to_visit.pop() {
                Some(id) => id,
                None => break,
            };

            let mut instance = source_tree.instances.remove(&id).unwrap();
            instance.parent = Some(parent_id);
            instance.children.clear();

            for child in &instance.children {
                to_visit.push((*child, id));
            }

            self.insert_instance_internal(instance);
        }
    }

    fn insert_instance_internal(&mut self, instance: RootedRbxInstance) {
        let parent_id = instance.parent
            .expect("Can not use insert_instance_internal on instances with no parent");

        {
            let parent = self.instances.get_mut(&parent_id)
                .expect("Cannot insert_instance_internal into an instance not in this tree");
            parent.children.push(instance.get_id());
        }

        self.instances.insert(instance.get_id(), instance);
    }

    pub fn insert_instance(&mut self, instance: RbxInstance, parent_id: RbxId) -> RbxId {
        let mut tree_instance = RootedRbxInstance::new(instance);
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
    pub fn descendants(&self, id: RbxId) -> Descendants {
        Descendants {
            tree: self,
            ids_to_visit: vec![id],
        }
    }
}

// Manually implement Clone to prevent accidental instance ID reuse.
impl Clone for RbxTree {
    fn clone(&self) -> RbxTree {
        #[inline]
        fn get_id(id_map: &mut HashMap<RbxId, RbxId>, source_id: RbxId) -> RbxId {
            if let Some(&new_id) = id_map.get(&source_id) {
                return new_id;
            }

            let new_id = RbxId::new();
            id_map.insert(source_id, new_id);

            new_id
        }

        let mut id_map = HashMap::new();
        let mut instances = HashMap::new();

        for (&id, instance) in &self.instances {
            let new_id = get_id(&mut id_map, id);
            let parent = instance.parent.map(|id| get_id(&mut id_map, id));
            let children = instance.children
                .iter()
                .map(|id| get_id(&mut id_map, *id))
                .collect();

            let mut new_instance = instance.clone_without_relations(new_id);
            new_instance.parent = parent;
            new_instance.children = children;

            instances.insert(new_id, new_instance);
        }

        let root_id = *id_map.get(&self.root_id).unwrap();

        RbxTree {
            instances,
            root_id,
        }
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