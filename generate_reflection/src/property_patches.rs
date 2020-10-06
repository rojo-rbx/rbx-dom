//! Defines changes and additions to the reflection dump that add and fix up
//! information.
//!
//! See the `patches/` directory for input.

use std::collections::HashMap;

use rbx_reflection::{DataType, PropertyKind, Scriptability};
use serde::Deserialize;

const PATCHES: &[&str] = &[
    include_str!("../patches/body-movers.yml"),
    // include_str!("../patches/camera.toml"),
    // include_str!("../patches/fire-and-smoke.toml"),
    // include_str!("../patches/instance.toml"),
    // include_str!("../patches/joint-instance.toml"),
    // include_str!("../patches/localization-table.toml"),
    // include_str!("../patches/parts.toml"),
    // include_str!("../patches/players.toml"),
    // include_str!("../patches/sound.toml"),
    // include_str!("../patches/workspace.toml"),
];

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PropertyPatches {
    #[serde(default)]
    pub change: HashMap<String, HashMap<String, PropertyChange>>,

    #[serde(default)]
    pub add: HashMap<String, HashMap<String, PropertyAdd>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PropertyChange {
    pub kind: Option<PropertyKind<'static>>,
    pub scriptability: Option<Scriptability>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PropertyAdd {
    pub data_type: DataType<'static>,
    pub kind: PropertyKind<'static>,
    pub scriptability: Scriptability,
}

pub fn load_property_patches() -> PropertyPatches {
    let mut all_patches = PropertyPatches::default();

    for patch_source in PATCHES {
        let parsed: PropertyPatches =
            serde_yaml::from_str(patch_source).expect("Couldn't parse property patch file");

        all_patches.change.extend(parsed.change);
        all_patches.add.extend(parsed.add);
    }

    all_patches
}
