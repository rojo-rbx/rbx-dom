//! Pattern for declaring instances inline in code more easily.

use std::collections::{HashMap, VecDeque};

use rbx_dom_weak::{RbxId, RbxInstanceProperties, RbxTree, RbxValue};

#[derive(Debug, Clone)]
pub struct InstanceBuilder {
    name: String,
    class_name: String,
    properties: HashMap<String, RbxValue>,
    children: Vec<InstanceBuilder>,
}

impl InstanceBuilder {
    pub fn new<S: Into<String>>(class_name: S) -> Self {
        let class_name = class_name.into();

        Self {
            name: class_name.clone(),
            class_name,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn name<S: Into<String>>(self, name: S) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    pub fn property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<RbxValue>,
    {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn properties<K, V, I>(mut self, properties: I) -> Self
    where
        K: Into<String>,
        V: Into<RbxValue>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in properties {
            self.properties.insert(key.into(), value.into());
        }

        self
    }

    pub fn child<C: Into<Self>>(mut self, child: C) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children<C, I>(mut self, children: I) -> Self
    where
        C: Into<Self>,
        I: IntoIterator<Item = C>,
    {
        for child in children {
            self.children.push(child.into());
        }
        self
    }

    pub fn build(self) -> RbxTree {
        let (properties, children) = self.into_properties();
        let mut tree = RbxTree::new(properties);

        let mut to_build: VecDeque<(RbxId, Self)> = VecDeque::new();

        let root_id = tree.get_root_id();
        for child in children {
            to_build.push_back((root_id, child));
        }

        while let Some((parent_id, builder)) = to_build.pop_front() {
            let (properties, children) = builder.into_properties();
            let id = tree.insert_instance(properties, parent_id);

            for child in children {
                to_build.push_back((id, child));
            }
        }

        tree
    }

    fn into_properties(self) -> (RbxInstanceProperties, Vec<InstanceBuilder>) {
        let properties = RbxInstanceProperties {
            name: self.name,
            class_name: self.class_name,
            properties: self.properties,
        };

        (properties, self.children)
    }
}
