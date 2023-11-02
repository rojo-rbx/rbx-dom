use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{bail, Context};
use clap::Parser;
use rbx_reflection::{
    ClassDescriptor, DataType, EnumDescriptor, PropertyDescriptor, PropertyKind,
    PropertySerialization, PropertyTag, ReflectionDatabase, Scriptability,
};
use rbx_types::VariantType;
use tempfile::tempdir;

use crate::{
    api_dump::{Dump, DumpClassMember, Security, Tag, ValueCategory},
    defaults::apply_defaults,
    patches::Patches,
};

use super::{defaults_place::DefaultsPlaceSubcommand, dump::DumpSubcommand};

/// Generate a reflection database from the system's Roblox Studio installation
/// and write it to disk.
#[derive(Debug, Parser)]
pub struct GenerateSubcommand {
    #[clap(long = "patches")]
    pub patches: Option<PathBuf>,
    /// Where to output the reflection database. The output format is inferred
    /// from the file path and supports JSON (.json) and MessagePack (.msgpack).
    pub output: Vec<PathBuf>,
}

impl GenerateSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let api_dump_path = temp_dir.path().join("api-dump.json");
        let defaults_place_path = temp_dir.path().join("defaults-place.rbxlx");

        DumpSubcommand {
            output: api_dump_path.clone(),
        }
        .run()?;

        let contents = fs::read_to_string(&api_dump_path).context("Could not read API dump")?;
        let dump = serde_json::from_str(&contents).context("Invalid API dump")?;

        let studio_info = DefaultsPlaceSubcommand {
            api_dump: api_dump_path,
            output: defaults_place_path.clone(),
        }
        .run()?;

        let mut database = ReflectionDatabase::new();

        apply_dump(&mut database, &dump)?;

        if let Some(patches_path) = &self.patches {
            let patches = Patches::load(patches_path)?;
            patches.apply(&mut database)?;
        }

        apply_defaults(&mut database, &defaults_place_path)?;

        database.version = studio_info.version;

        for path in &self.output {
            let extension = path.extension().unwrap_or_default().to_str();

            let mut file = BufWriter::new(File::create(path)?);

            match extension {
                Some("json") => {
                    serde_json::to_writer_pretty(&mut file, &database)
                        .context("Could not serialize reflection database as JSON")?;
                }
                Some("msgpack") => {
                    let buf = rmp_serde::to_vec(&database)
                        .context("Could not serialize reflection database as MessagePack")?;

                    file.write_all(&buf)?;
                }
                _ => bail!(
                    "Unknown output for path {} -- \
                    Supported formats are JSON (.json) and MessagePack (.msgpack)",
                    path.display()
                ),
            }

            file.flush()?;
        }

        Ok(())
    }
}

fn apply_dump(database: &mut ReflectionDatabase, dump: &Dump) -> anyhow::Result<()> {
    for dump_class in &dump.classes {
        let superclass = if dump_class.superclass == "<<<ROOT>>>" {
            None
        } else {
            Some(Cow::Owned(dump_class.superclass.clone()))
        };

        let mut tags = HashSet::new();
        for dump_tag in &dump_class.tags {
            if let Tag::Regular(tag) = dump_tag {
                tags.insert(tag.parse().unwrap());
            }
        }

        let mut properties = HashMap::new();

        for member in &dump_class.members {
            if let DumpClassMember::Property(dump_property) = member {
                let mut tags = HashSet::new();
                for dump_tag in &dump_property.tags {
                    if let Tag::Regular(tag) = dump_tag {
                        tags.insert(tag.parse().unwrap());
                    }
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

                let can_serialize =
                    !tags.contains(&PropertyTag::ReadOnly) && dump_property.serialization.can_save;

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
                        match variant_type_from_str(type_name)? {
                            Some(variant_type) => DataType::Value(variant_type),
                            None => {
                                log::debug!("Skipping property {}.{} because it was of unsupported type '{type_name}'", dump_class.name, dump_property.name);
                                continue;
                            }
                        }
                    }
                    ValueCategory::Class => DataType::Value(VariantType::Ref),
                };

                let mut property = PropertyDescriptor::new(dump_property.name.clone(), value_type);
                property.scriptability = scriptability;
                property.tags = tags;
                property.kind = kind;

                properties.insert(Cow::Owned(dump_property.name.clone()), property);
            }
        }

        let mut class = ClassDescriptor::new(dump_class.name.clone());
        class.superclass = superclass;
        class.tags = tags;
        class.properties = properties;

        database
            .classes
            .insert(Cow::Owned(dump_class.name.clone()), class);
    }

    for dump_enum in &dump.enums {
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

fn variant_type_from_str(value: &str) -> anyhow::Result<Option<VariantType>> {
    Ok(Some(match value {
        "Axes" => VariantType::Axes,
        "BinaryString" => VariantType::BinaryString,
        "BrickColor" => VariantType::BrickColor,
        "CFrame" => VariantType::CFrame,
        "Color3" => VariantType::Color3,
        "Color3uint8" => VariantType::Color3uint8,
        "ColorSequence" => VariantType::ColorSequence,
        "Content" => VariantType::Content,
        "Faces" => VariantType::Faces,
        "Font" => VariantType::Font,
        "Instance" => VariantType::Ref,
        "NumberRange" => VariantType::NumberRange,
        "NumberSequence" => VariantType::NumberSequence,
        "OptionalCoordinateFrame" => VariantType::OptionalCFrame,
        "PhysicalProperties" => VariantType::PhysicalProperties,
        "Ray" => VariantType::Ray,
        "Rect" => VariantType::Rect,
        "Region3" => VariantType::Region3,
        "Region3int16" => VariantType::Region3int16,
        "SecurityCapabilities" => VariantType::SecurityCapabilities,
        "SharedString" => VariantType::SharedString,
        "UDim" => VariantType::UDim,
        "UDim2" => VariantType::UDim2,
        "UniqueId" => VariantType::UniqueId,
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
        "TweenInfo" => return Ok(None),

        // While DateTime is possible to Serialize, the only use it has as a
        // DataType is for the TextChatMessage class, which cannot be serialized
        // (at least not saved to file as it is locked to nil parent)
        "DateTime" => return Ok(None),

        // These types are not generally implemented right now.
        "QDir" | "QFont" | "SystemAddress" | "CSGPropertyData" => return Ok(None),

        _ => bail!("Unknown type {}", value),
    }))
}
