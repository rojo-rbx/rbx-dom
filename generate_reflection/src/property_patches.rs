use std::collections::HashMap;

use serde_derive::Deserialize;

lazy_static::lazy_static! {
    static ref PROPERTY_PATCHES: PropertyPatchDatabase = {
        let source = include_str!("../property-patches.toml");
        toml::from_str(source)
            .expect("Couldn't parse property-patches.toml")
    };
}

pub type PropertyPatchDatabase = HashMap<String, HashMap<String, CanonicalProperty>>;

pub fn get_property_patches() -> &'static PropertyPatchDatabase {
    &PROPERTY_PATCHES
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