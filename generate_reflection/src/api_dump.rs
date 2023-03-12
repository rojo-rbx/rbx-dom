//! Interface for dealing with Roblox Studio's JSON API Dump. Isn't specific to
//! this crate and could probably turn into a separate crate.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::process::Command;

use anyhow::Context;
use rbx_dom_weak::types::VariantType;
use rbx_reflection::{
    ClassDescriptor, DataType, EnumDescriptor, PropertyDescriptor, PropertyKind,
    PropertySerialization, PropertyTag, ReflectionDatabase, Scriptability,
};
use roblox_install::RobloxStudio;
use serde::Deserialize;
use tempfile::tempdir;

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
    pub tags: Vec<String>,
    pub members: Vec<DumpClassMember>,
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
    pub security: PropertySecurity,

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

#[derive(Debug, Clone, Copy, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum Security {
    None,
    LocalUserSecurity,
    PluginSecurity,
    RobloxScriptSecurity,
    NotAccessibleSecurity,
    RobloxSecurity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertySecurity {
    read: Security,
    write: Security,
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
    pub fn read() -> anyhow::Result<Dump> {
        let studio_install =
            RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

        let dir = tempdir()?;
        let dump_path = dir.path().join("api-dump.json");

        Command::new(studio_install.application_path())
            .arg("-API")
            .arg(&dump_path)
            .status()?;

        let contents = fs::read_to_string(&dump_path)?;
        let dump: Dump =
            serde_json::from_str(&contents).context("Roblox Studio produced an invalid dump")?;

        Ok(dump)
    }

    /// Adds all of the classes from the given API dump to the reflection
    /// database.
    pub fn apply(&self, database: &mut ReflectionDatabase) -> anyhow::Result<()> {
        for dump_class in &self.classes {
            let superclass = if dump_class.superclass == "<<<ROOT>>>" {
                None
            } else {
                Some(Cow::Owned(dump_class.superclass.clone()))
            };

            let mut tags = HashSet::new();
            for dump_tag in &dump_class.tags {
                tags.insert(dump_tag.parse().unwrap());
            }

            let mut properties = HashMap::new();

            for member in &dump_class.members {
                if let DumpClassMember::Property(dump_property) = member {
                    let mut tags = HashSet::new();
                    for dump_tag in &dump_property.tags {
                        tags.insert(dump_tag.parse().unwrap());
                    }

                    let read_scriptability = match dump_property.security.read {
                        Security::None | Security::PluginSecurity => Scriptability::Read,
                        _ => Scriptability::None,
                    };

                    let write_scriptability = if tags.contains(&PropertyTag::ReadOnly) {
                        Scriptability::None
                    } else {
                        match dump_property.security.write {
                            Security::None | Security::PluginSecurity => Scriptability::Write,
                            _ => Scriptability::None,
                        }
                    };

                    let scriptability = if tags.contains(&PropertyTag::NotScriptable) {
                        Scriptability::None
                    } else {
                        match (read_scriptability, write_scriptability) {
                            (Scriptability::Read, Scriptability::Write) => Scriptability::ReadWrite,
                            (Scriptability::Read, Scriptability::None) => Scriptability::Read,
                            (Scriptability::None, Scriptability::Write) => Scriptability::Write,
                            _ => Scriptability::None,
                        }
                    };

                    let can_serialize = !tags.contains(&PropertyTag::ReadOnly)
                        && dump_property.serialization.can_save;

                    let serialization = if can_serialize {
                        PropertySerialization::Serializes
                    } else {
                        PropertySerialization::DoesNotSerialize
                    };

                    // We assume that all properties are canonical by default,
                    // since most properties are. Properties are updated by
                    // patches later on in the database generation process.
                    let kind = PropertyKind::Canonical { serialization };

                    let type_name = &dump_property.value_type.name;
                    let value_type = match dump_property.value_type.category {
                        ValueCategory::Enum => DataType::Enum(type_name.clone().into()),
                        ValueCategory::Primitive | ValueCategory::DataType => {
                            match variant_type_from_str(type_name) {
                                Some(variant_type) => DataType::Value(variant_type),
                                None => continue,
                            }
                        }
                        ValueCategory::Class => DataType::Value(VariantType::Ref),
                    };

                    let mut property =
                        PropertyDescriptor::new(dump_property.name.clone(), value_type);
                    property.scriptability = scriptability;
                    property.tags = tags;
                    property.kind = kind;

                    properties.insert(Cow::Owned(dump_property.name.clone()), property);
                }
            }

            let mut class = ClassDescriptor::new(dump_class.name.clone());
            class.tags = tags;
            class.superclass = superclass;
            class.properties = properties;

            database
                .classes
                .insert(Cow::Owned(dump_class.name.clone()), class);
        }

        for dump_enum in &self.enums {
            let mut descriptor = EnumDescriptor::new(dump_enum.name.clone());

            for dump_item in &dump_enum.items {
                descriptor
                    .items
                    .insert(Cow::Owned(dump_item.name.clone()), dump_item.value);
            }

            database
                .enums
                .insert(Cow::Owned(dump_enum.name.clone()), descriptor);
        }

        Ok(())
    }
}

fn variant_type_from_str(value: &str) -> Option<VariantType> {
    Some(match value {
        "Axes" => VariantType::Axes,
        "BinaryString" => VariantType::BinaryString,
        "BrickColor" => VariantType::BrickColor,
        "CFrame" => VariantType::CFrame,
        "Color3" => VariantType::Color3,
        "ColorSequence" => VariantType::ColorSequence,
        "Content" => VariantType::Content,
        "Faces" => VariantType::Faces,
        "Font" => VariantType::Font,
        "Instance" => VariantType::Ref,
        "NumberRange" => VariantType::NumberRange,
        "NumberSequence" => VariantType::NumberSequence,
        "PhysicalProperties" => VariantType::PhysicalProperties,
        "Ray" => VariantType::Ray,
        "Rect" => VariantType::Rect,
        "Region3" => VariantType::Region3,
        "Region3int16" => VariantType::Region3int16,
        "UDim" => VariantType::UDim,
        "UDim2" => VariantType::UDim2,
        "Vector2" => VariantType::Vector2,
        "Vector2int16" => VariantType::Vector2int16,
        "Vector3" => VariantType::Vector3,
        "Vector3int16" => VariantType::Vector3int16,
        "bool" => VariantType::Bool,
        "double" => VariantType::Float64,
        "float" => VariantType::Float32,
        "int" => VariantType::Int32,
        "int64" => VariantType::Int64,
        "string" => VariantType::String,

        // ProtectedString is handled as the same as string
        "ProtectedString" => VariantType::String,

        // TweenInfo is not supported by rbx_types yet
        "TweenInfo" => return None,

        // While DateTime is possible to Serialize, the only use it has as a
        // DataType is for the TextChatMessage class, which cannot be serialized
        // (at least not saved to file as it is locked to nil parent)
        "DateTime" => return None,

        // These types are not generally implemented right now.
        "QDir" | "QFont" => return None,

        _ => panic!("Unknown type {}", value),
    })
}
