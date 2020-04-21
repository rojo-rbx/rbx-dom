use std::collections::{BTreeMap, HashMap};

use crate::{
    types::{Ref, Variant},
    WeakDom,
};
use serde::{Deserialize, Serialize};

/// Contains state for viewing/redacting WeakDom objects, making them suitable
/// for viewing in a snapshot test.
pub struct DomViewer {
    referent_map: HashMap<Ref, String>,
    next_referent: usize,
}

impl DomViewer {
    pub fn new() -> Self {
        Self {
            referent_map: HashMap::new(),
            next_referent: 0,
        }
    }

    pub fn view(&mut self, dom: &WeakDom) -> ViewedInstance {
        let root_referent = dom.root_ref();
        self.populate_referent_map(dom, root_referent);
        self.view_instance(dom, root_referent)
    }

    pub fn view_children(&mut self, dom: &WeakDom) -> Vec<ViewedInstance> {
        let root_instance = dom.root();
        let children = root_instance.children();

        for &referent in children {
            self.populate_referent_map(dom, referent);
        }

        children
            .iter()
            .map(|&referent| self.view_instance(dom, referent))
            .collect()
    }

    fn populate_referent_map(&mut self, dom: &WeakDom, referent: Ref) {
        self.referent_map
            .insert(referent, format!("referent-{}", self.next_referent));
        self.next_referent += 1;

        let instance = dom.get_by_ref(referent).unwrap();
        for referent in instance.children() {
            self.populate_referent_map(dom, *referent);
        }
    }

    fn view_instance(&self, dom: &WeakDom, referent: Ref) -> ViewedInstance {
        let instance = dom.get_by_ref(referent).unwrap();

        let children = instance
            .children()
            .iter()
            .copied()
            .map(|referent| self.view_instance(dom, referent))
            .collect();

        let properties = instance
            .properties
            .iter()
            .map(|(key, value)| {
                let key = key.clone();
                let new_value = match value {
                    Variant::Ref(ref_referent) => {
                        let referent_str = self
                            .referent_map
                            .get(ref_referent)
                            .cloned()
                            .unwrap_or_else(|| "[unknown ID]".to_owned());

                        ViewedValue::Ref(referent_str)
                    }
                    other => ViewedValue::Other(other.clone()),
                };

                (key, new_value)
            })
            .collect();

        ViewedInstance {
            referent: self.referent_map.get(&referent).unwrap().clone(),
            name: instance.name.clone(),
            class: instance.class.clone(),
            properties,
            children,
        }
    }
}

/// A transformed view into an WeakDom or RbxInstance that has been redacted and
/// transformed to be more readable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewedInstance {
    referent: String,
    name: String,
    class: String,
    properties: BTreeMap<String, ViewedValue>,
    children: Vec<ViewedInstance>,
}

/// Wrapper around Variant with refs replaced to be redacted, stable versions of
/// their original IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ViewedValue {
    Ref(String),
    Other(Variant),
}

#[cfg(test)]
mod test {
    use super::*;

    use rbx_dom_weak::RbxInstanceProperties;

    #[test]
    fn redact_single() {
        let dom = WeakDom::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class: "Folder".to_owned(),
            properties: HashMap::new(),
        });

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }

    #[test]
    fn redact_multi() {
        let mut dom = WeakDom::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class: "Folder".to_owned(),
            properties: HashMap::new(),
        });

        let root_referent = dom.root_ref();

        for i in 0..4 {
            let name = format!("Child {}", i);
            let properties = RbxInstanceProperties {
                name,
                class: "Folder".to_owned(),
                properties: HashMap::new(),
            };

            dom.insert_instance(properties, root_referent);
        }

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }

    #[test]
    fn redact_values() {
        let mut dom = WeakDom::new(RbxInstanceProperties {
            name: "Root".to_owned(),
            class: "ObjectValue".to_owned(),
            properties: HashMap::new(),
        });

        let root_instance = dom.root(root_referent);

        root_instance.properties.insert(
            "Value".to_owned(),
            Variant::Ref {
                value: Some(root_referent),
            },
        );

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }
}
