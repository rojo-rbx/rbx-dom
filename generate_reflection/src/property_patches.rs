use std::{
    borrow::Cow,
    collections::HashMap,
};

use serde_derive::Deserialize;

use crate::reflection_types::RbxPropertyType;

lazy_static::lazy_static! {
    static ref PROPERTY_PATCHES: PropertyPatches = {
        let source = include_str!("../property-patches.toml");
        toml::from_str(source)
            .expect("Couldn't parse property-patches.toml")
    };
}

#[derive(Debug, Deserialize)]
pub struct PropertyPatches {
    pub change: HashMap<String, HashMap<String, PropertyChange>>,
    pub add: HashMap<String, HashMap<String, PropertyAdd>>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyChange {
    pub serialized_name: Option<Cow<'static, str>>,
    pub canonical_name: Option<Cow<'static, str>>,
    pub scriptability: Option<Scriptability>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyAdd {
    #[serde(rename = "type")]
    pub property_type: RbxPropertyType,
    pub serialized_name: Option<Cow<'static, str>>,
    pub canonical_name: Option<Cow<'static, str>>,
    pub scriptability: Scriptability,
}

pub fn get_property_patches() -> &'static PropertyPatches {
    &PROPERTY_PATCHES
}

#[derive(Debug, Deserialize)]
pub enum Scriptability {
    None,
    ReadWrite,
    Read,
    Write,
    Custom,
}