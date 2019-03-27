use std::{
    io,
    fs,
    process::Command,
};

use serde_derive::Deserialize;
use tempfile::tempdir;

use crate::roblox_install::RobloxStudio;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<DumpClass>,
    pub enums: Vec<DumpEnum>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClass {
    pub name: String,
    pub superclass: String,

    #[serde(default)]
    pub tags: Vec<DumpClassTag>,
    pub members: Vec<DumpClassMember>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum DumpClassTag {
    Deprecated,
    NotBrowsable,
    NotCreatable,
    NotReplicated,
    PlayerReplicated,
    Service,
    Settings,
}

impl DumpClassTag {
    pub fn name(&self) -> &'static str {
        match self {
            DumpClassTag::Deprecated => "Deprecated",
            DumpClassTag::NotBrowsable => "NotBrowsable",
            DumpClassTag::NotCreatable => "NotCreatable",
            DumpClassTag::NotReplicated => "NotReplicated",
            DumpClassTag::PlayerReplicated => "PlayerReplicated",
            DumpClassTag::Service => "Service",
            DumpClassTag::Settings => "Settings",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "MemberType")]
pub enum DumpClassMember {
    Property(DumpClassProperty),

    #[serde(rename_all = "PascalCase")]
    Function {
        name: String,
    },

    #[serde(rename_all = "PascalCase")]
    Event {
        name: String,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassProperty {
    pub name: String,
    pub value_type: ValueType,
    pub serialization: Serialization,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValueType {
    pub name: String,
    pub category: ValueCategory,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ValueCategory {
    /// Lua primitives like float or string
    Primitive,

    /// Roblox data types like Vector3 or CFrame
    DataType,

    /// Roblox enum like FormFactor or Genre
    Enum,

    /// An instance reference
    Class,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Serialization {
    pub can_save: bool,
    pub can_load: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnum {
    pub name: String,
    pub items: Vec<DumpEnumItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnumItem {
    pub name: String,
    pub value: u32,
}

impl Dump {
    pub fn read_with_source() -> io::Result<(String, Dump)> {
        let studio_install = RobloxStudio::locate()?;

        let dir = tempdir()?;
        let dump_path = dir.path().join("api-dump.json");

        Command::new(studio_install.exe_path())
            .arg("-API")
            .arg(&dump_path)
            .status()?;

        let contents = fs::read_to_string(&dump_path)?;
        let mut dump: Dump = serde_json::from_str(&contents)
            .expect("Roblox Studio produced an invalid dump");

        dump.classes.sort_by_key(|class| class.name.clone());
        dump.enums.sort_by_key(|rbx_enum| rbx_enum.name.clone());

        Ok((contents, dump))
    }
}