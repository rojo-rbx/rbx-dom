use std::{
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
use rmp_serde::Serializer;
use serde::Serialize;
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
    #[clap(long)]
    pub patches: Option<PathBuf>,
    /// Where to output the reflection database. The output format is inferred
    /// from the file path and supports JSON (.json) and MessagePack (.msgpack).
    pub output: Vec<PathBuf>,
    /// Whether to pretty-print the JSON output. This has no effect on MessagePack.
    #[clap(long)]
    pub no_pretty: bool,
    /// Whether to serialize MessagePack in a human readable format. This has no effect on JSON.
    #[clap(long)]
    pub human_readable: bool,
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

        let patches = if let Some(patches_path) = &self.patches {
            Some(Patches::load(patches_path)?)
        } else {
            None
        };

        if let Some(patches) = &patches {
            patches.apply_pre_default(&mut database)?;
        }

        apply_defaults(&mut database, &defaults_place_path)?;

        if let Some(patches) = &patches {
            patches.apply_post_default(&mut database)?;
        }

        database.version = studio_info.version;

        for path in &self.output {
            let extension = path.extension().unwrap_or_default().to_str();

            let mut file = BufWriter::new(File::create(path)?);

            match extension {
                Some("json") => {
                    let result = if self.no_pretty {
                        serde_json::to_writer(&mut file, &database)
                    } else {
                        serde_json::to_writer_pretty(&mut file, &database)
                    };

                    result.context("Could not serialize reflection database as JSON")?;
                }
                Some("msgpack") => {
                    let buf = if self.human_readable {
                        let mut slice = Vec::with_capacity(128);
                        let mut serializer = Serializer::new(&mut slice)
                            .with_human_readable()
                            .with_struct_map();

                        database.serialize(&mut serializer).context(
                            "Could not serialize reflection database as human readable MessagePack",
                        )?;

                        slice
                    } else {
                        rmp_serde::to_vec(&database)
                            .context("Could not serialize reflection database as MessagePack")?
                    };

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

fn apply_dump<'db>(database: &mut ReflectionDatabase<'db>, dump: &'db Dump) -> anyhow::Result<()> {
    let mut ignored_properties = Vec::new();

    for dump_class in &dump.classes {
        let superclass = if dump_class.superclass == "<<<ROOT>>>" {
            None
        } else {
            Some(dump_class.superclass.as_str())
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
                    ValueCategory::Enum => DataType::Enum(type_name.as_str()),
                    ValueCategory::Primitive | ValueCategory::DataType => {
                        // variant_type_from_str returns None when passed a
                        // type name that does not have a corresponding
                        // VariantType. Exactly what we'd like to do about
                        // unimplemented data types like this depends on if the
                        // property serializes or not.
                        match (variant_type_from_str(type_name), &kind) {
                            // The happy path: the data type has a corresponding
                            // VariantType. We don't need to care about whether
                            // the data type is ever serialized, because it
                            // already has an implementation.
                            (Some(variant_type), _) => DataType::Value(variant_type),

                            // The data type does not have a corresponding
                            // VariantType, and it serializes. This is a case
                            // where we should fail. It means that we may need
                            // to implement the data type.
                            //
                            // There is a special exception for QDir and QFont,
                            // because these types serialize under a few
                            // different properties, but the properties are not
                            // normally present in place or model files. They
                            // are usually only present in Roblox Studio
                            // settings files. They are not used otherwise and
                            // can safely be ignored.
                            (
                                None,
                                PropertyKind::Canonical {
                                    serialization: PropertySerialization::Serializes,
                                },
                            ) if type_name != "QDir" && type_name != "QFont" => {
                                log::warn!(
                                "Property {}.{} serializes, but its data type ({}) is unimplemented",
                                dump_class.name, dump_property.name, type_name
                            );
                                continue;
                            }

                            // The data type does not have a corresponding a
                            // VariantType, and it does not serialize (with QDir
                            // and QFont as exceptions, noted above). We can
                            // safely ignore this case because rbx-dom doesn't
                            // need to know about data types that are never
                            // serialized.
                            (None, _) => {
                                ignored_properties.push((
                                    &dump_class.name,
                                    &dump_property.name,
                                    type_name,
                                ));
                                continue;
                            }
                        }
                    }
                    ValueCategory::Class => DataType::Value(VariantType::Ref),
                };

                let mut property = PropertyDescriptor::new(dump_property.name.as_str(), value_type);
                property.scriptability = scriptability;
                property.tags = tags;
                property.kind = kind;

                properties.insert(dump_property.name.as_str(), property);
            }
        }

        let mut class = ClassDescriptor::new(dump_class.name.as_str());
        class.superclass = superclass;
        class.tags = tags;
        class.properties = properties;

        database.classes.insert(dump_class.name.as_str(), class);
    }

    log::debug!("Skipped the following properties because their data types are not implemented, and do not need to serialize:");

    for (class_name, property_name, type_name) in ignored_properties {
        log::debug!("{class_name}.{property_name}: {type_name}");
    }

    for dump_enum in &dump.enums {
        let mut descriptor = EnumDescriptor::new(dump_enum.name.as_str());

        for dump_item in &dump_enum.items {
            descriptor
                .items
                .insert(dump_item.name.as_str(), dump_item.value);
        }

        database.enums.insert(dump_enum.name.as_str(), descriptor);
    }

    Ok(())
}

fn variant_type_from_str(type_name: &str) -> Option<VariantType> {
    Some(match type_name {
        "Axes" => VariantType::Axes,
        "BinaryString" => VariantType::BinaryString,
        "BrickColor" => VariantType::BrickColor,
        "CFrame" => VariantType::CFrame,
        "Color3" => VariantType::Color3,
        "Color3uint8" => VariantType::Color3uint8,
        "ColorSequence" => VariantType::ColorSequence,
        "Content" => VariantType::Content,
        "ContentId" => VariantType::ContentId,
        "Faces" => VariantType::Faces,
        "Font" => VariantType::Font,
        "Instance" => VariantType::Ref,
        "NetAssetRef" => VariantType::NetAssetRef,
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

        _ => return None,
    })
}
