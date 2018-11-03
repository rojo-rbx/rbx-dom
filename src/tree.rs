use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::{
    id::RbxId,
    instance::RbxInstance,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RbxTreeInstance {
    #[serde(flatten)]
    inner: RbxInstance,

    /// The unique ID of the instance
    id: RbxId,

    /// All of the children of this instance. Order is relevant to preserve!
    children: Vec<RbxId>,

    /// The parent of the instance, if there is one.
    parent: Option<RbxId>,
}

impl RbxTreeInstance {
    fn new(instance: RbxInstance, parent: RbxId) -> RbxTreeInstance {
        RbxTreeInstance {
            inner: instance,
            id: RbxId::new(),
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    fn new_without_parent(instance: RbxInstance) -> RbxTreeInstance {
        RbxTreeInstance {
            inner: instance,
            id: RbxId::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get_id(&self) -> RbxId {
        self.id
    }

    pub fn get_parent_id(&self) -> Option<RbxId> {
        self.parent
    }

    pub fn get_children_ids(&self) -> &[RbxId] {
        &self.children
    }
}

impl std::ops::Deref for RbxTreeInstance {
    type Target = RbxInstance;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RbxTree {
    instances: HashMap<RbxId, RbxTreeInstance>,
    root_id: RbxId,
}

impl RbxTree {
    pub fn new(root: RbxInstance) -> RbxTree {
        let root = RbxTreeInstance::new_without_parent(root);
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

    pub fn get_instance(&self, id: RbxId) -> Option<&RbxTreeInstance> {
        self.instances.get(&id)
    }

    pub fn get_instance_mut(&mut self, id: RbxId) -> Option<&mut RbxTreeInstance> {
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

    fn insert_instance_internal(&mut self, instance: RbxTreeInstance) {
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
        let tree_instance = RbxTreeInstance::new(instance, parent_id);
        let id = tree_instance.get_id();

        self.insert_instance_internal(tree_instance);

        id
    }

    /// Given an ID, remove the instance from the tree with that ID, along with
    /// all of its descendants.
    pub fn unroot(&mut self, _id: RbxId) -> Option<RbxTree> {
        unimplemented!()
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

pub struct Descendants<'a> {
    tree: &'a RbxTree,
    ids_to_visit: Vec<RbxId>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = &'a RbxTreeInstance;

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