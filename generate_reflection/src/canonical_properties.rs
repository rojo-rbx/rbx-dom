use std::collections::HashMap;

use serde_derive::Deserialize;

lazy_static::lazy_static! {
    static ref CANONICAL_PROPERTIES: CanonicalPropertyDatabase = {
        let source = include_str!("../canonical-properties.toml");
        let inner = toml::from_str(source)
            .expect("Couldn't parse canonical-properties.toml");

        CanonicalPropertyDatabase { inner }
    };
}

pub fn get_canonical_properties() -> &'static CanonicalPropertyDatabase {
    &CANONICAL_PROPERTIES
}

pub struct CanonicalPropertyDatabase {
    inner: HashMap<String, HashMap<String, CanonicalProperty>>,
}

impl CanonicalPropertyDatabase {
    pub fn query_class<'a>(&'a self, class_name: &str) -> Option<&'a HashMap<String, CanonicalProperty>> {
        self.inner.get(class_name)
    }

    pub fn query_property<'a>(&'a self, class_name: &str, property_name: &str) -> Option<&'a CanonicalProperty> {
        let class = self.query_class(class_name)?;
        class.get(property_name)
    }
}

const fn nope() -> bool { false }

#[derive(Debug, Deserialize)]
pub struct CanonicalProperty {
    #[serde(default = "nope")]
    is_canonical: bool,

    serialized_name: Option<String>,
    canonical_name: Option<String>,
    scriptability: Option<Scriptability>,
}

#[derive(Debug, Deserialize)]
pub enum Scriptability {
    None,
    ReadWrite,
    Custom,
    Read,
    Write,
}