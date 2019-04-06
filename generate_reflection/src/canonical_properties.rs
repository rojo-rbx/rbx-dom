use std::collections::HashMap;

use serde_derive::Deserialize;

lazy_static::lazy_static! {
    static ref CANONICAL_PROPERTIES: CanonicalPropertyDatabase = {
        let source = include_str!("../canonical-properties.toml");
        toml::from_str(source)
            .expect("Couldn't parse canonical-properties.toml")
    };
}

pub fn get_canonical_properties() -> &'static CanonicalPropertyDatabase {
    &CANONICAL_PROPERTIES
}

pub type CanonicalPropertyDatabase = HashMap<String, HashMap<String, CanonicalProperty>>;

#[derive(Debug, Deserialize)]
pub struct CanonicalProperty {
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