//! Defines changes and additions to the reflection dump that add and fix up
//! information.
//!
//! See `property-patches.toml` for the input.

use std::{borrow::Cow, collections::HashMap};

use rbx_reflection::{PropertyType, Scriptability};
use serde_derive::Deserialize;

static PATCH_SOURCE: &str = include_str!("../property-patches.toml");

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
    pub property_type: PropertyType<'static>,
    pub serialized_name: Option<Cow<'static, str>>,
    pub canonical_name: Option<Cow<'static, str>>,
    pub scriptability: Scriptability,
    pub serializes: bool,
}

pub fn load_property_patches() -> PropertyPatches {
    toml::from_str(PATCH_SOURCE).expect("Couldn't parse property-patches.toml")
}
