use std::{
    borrow::Cow,
    collections::HashMap,
};

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
    pub is_canonical: Option<bool>,
    #[serde(rename = "type")]
    pub property_type: Option<RbxPropertyType>,
    pub serialized_name: Option<Cow<'static, str>>,
    pub canonical_name: Option<Cow<'static, str>>,
    pub scriptability: Option<Scriptability>,
}

#[derive(Debug, Deserialize)]
pub enum Scriptability {
    None,
    ReadWrite,
    Custom,
    Read,
    Write,
}