use std::collections::HashMap;

use serde_derive::Deserialize;

use crate::reflection_types::RbxPropertyType;

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

#[derive(Debug, Deserialize)]
pub struct CanonicalProperty {
    is_canonical: Option<bool>,
    #[serde(rename = "type")]
    property_type: Option<RbxPropertyType>,
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