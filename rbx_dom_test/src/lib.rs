use std::collections::{BTreeMap, HashMap};

use rbx_dom_weak::{RbxId, RbxTree, RbxValue};
use serde::{Deserialize, Serialize};

/// Contains state for viewing/redacting RbxTree objects, making them suitable
/// for viewing in a snapshot test.
pub struct TreeViewer {
    id_map: HashMap<RbxId, String>,
    next_id: usize,
}

impl TreeViewer {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn view(&mut self, tree: &RbxTree) -> ViewedInstance {
        let root_id = tree.get_root_id();
        self.populate_id_map(tree, root_id);
        self.view_instance(tree, root_id)
    }

    fn populate_id_map(&mut self, tree: &RbxTree, id: RbxId) {
        self.id_map.insert(id, format!("id-{}", self.next_id));
        self.next_id += 1;

        let instance = tree.get_instance(id).unwrap();
        for id in instance.get_children_ids() {
            self.populate_id_map(tree, *id);
        }
    }

    fn view_instance(&self, tree: &RbxTree, id: RbxId) -> ViewedInstance {
        let instance = tree.get_instance(id).unwrap();

        let children = instance
            .get_children_ids()
            .iter()
            .copied()
            .map(|id| self.view_instance(tree, id))
            .collect();

        let properties = instance
            .properties
            .iter()
            .map(|(key, value)| {
                let key = key.clone();
                let new_value = match value {
                    RbxValue::Ref {
                        value: Some(ref_id),
                    } => {
                        let id_str = self
                            .id_map
                            .get(ref_id)
                            .cloned()
                            .unwrap_or_else(|| "[unknown ID]".to_owned());
                        ViewedValue::Ref(id_str)
                    }
                    other => ViewedValue::Other(other.clone()),
                };

                (key, new_value)
            })
            .collect();

        ViewedInstance {
            id: self.id_map.get(&id).unwrap().clone(),
            name: instance.name.clone(),
            class_name: instance.class_name.clone(),
            properties,
            children,
        }
    }
}

/// A transformed view into an RbxTree or RbxInstance that has been redacted and
/// transformed to be more readable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewedInstance {
    id: String,
    name: String,
    class_name: String,
    properties: BTreeMap<String, ViewedValue>,
    children: Vec<ViewedInstance>,
}

/// Wrapper around RbxValue with refs replaced to be redacted, stable versions
/// of their original IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ViewedValue {
    Ref(String),
    Other(RbxValue),
}

#[cfg(test)]
mod test {
    use super::*;

    use rbx_dom_weak::RbxInstanceProperties;

    #[test]
    fn redact_single() {
        let tree = RbxTree::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class_name: "Folder".to_owned(),
            properties: HashMap::new(),
        });

        insta::assert_yaml_snapshot!(TreeViewer::new().view(&tree));
    }

    #[test]
    fn redact_multi() {
        let mut tree = RbxTree::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class_name: "Folder".to_owned(),
            properties: HashMap::new(),
        });

        let root_id = tree.get_root_id();

        for i in 0..4 {
            let name = format!("Child {}", i);
            let properties = RbxInstanceProperties {
                name,
                class_name: "Folder".to_owned(),
                properties: HashMap::new(),
            };

            tree.insert_instance(properties, root_id);
        }

        insta::assert_yaml_snapshot!(TreeViewer::new().view(&tree));
    }

    #[test]
    fn redact_values() {
        let mut tree = RbxTree::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class_name: "ObjectValue".to_owned(),
            properties: HashMap::new(),
        });

        let root_id = tree.get_root_id();
        let root_instance = tree.get_instance_mut(root_id).unwrap();

        root_instance.properties.insert(
            "Value".to_owned(),
            RbxValue::Ref {
                value: Some(root_id),
            },
        );

        insta::assert_yaml_snapshot!(TreeViewer::new().view(&tree));
    }
}
