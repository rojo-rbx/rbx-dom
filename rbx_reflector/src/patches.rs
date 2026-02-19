use std::{collections::HashMap, fs, path::Path};

use anyhow::{anyhow, bail, Context};
use rbx_reflection::{
    MigrationOperation, PropertyKind, PropertySerialization, ReflectionDatabase, Scriptability,
};
use rbx_types::{Variant, VariantType};
use serde::Deserialize;

pub struct Patches {
    change: HashMap<String, HashMap<String, PropertyChange>>,
}

impl Patches {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let mut change = HashMap::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let contents = fs::read_to_string(entry.path())?;
            let patch: Patch = serde_yaml::from_str(&contents)
                .with_context(|| format!("Error parsing patch file {}", entry.path().display()))?;

            change.extend(patch.change);
        }

        Ok(Self { change })
    }

    pub fn apply_pre_default<'db>(
        &'db self,
        database: &mut ReflectionDatabase<'db>,
    ) -> anyhow::Result<()> {
        for (class_name, class_changes) in &self.change {
            let class = database
                .classes
                .get_mut(class_name.as_str())
                .ok_or_else(|| {
                    anyhow!(
                        "Class {} modified in patch file does not exist in database",
                        class_name
                    )
                })?;

            for (property_name, property_change) in class_changes {
                let existing_property = class
                    .properties
                    .get_mut(property_name.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "Property {}.{} modified in patch file does not exist in database",
                            class_name,
                            property_name
                        )
                    })?;

                if let Some(data_type) = &property_change.data_type {
                    existing_property.data_type = data_type.into();
                }

                if let Some(kind) = property_change.kind() {
                    if let (
                        PropertyKind::Canonical { serialization },
                        PropertyKind::Canonical {
                            serialization: existing_serialization,
                        },
                    ) = (&kind, &existing_property.kind)
                    {
                        match (serialization, existing_serialization) {
                            (PropertySerialization::Serializes, PropertySerialization::Serializes)
                            | (PropertySerialization::DoesNotSerialize, PropertySerialization::DoesNotSerialize) => bail!("The serialization for property {class_name}.{property_name} was unchanged"),
                            _ => {}
                        };
                    }

                    existing_property.kind = kind;
                }

                if let Some(scriptability) = &property_change.scriptability {
                    match (existing_property.scriptability, scriptability) {
                        (Scriptability::Custom, Scriptability::Custom)
                        | (Scriptability::None, Scriptability::None)
                        | (Scriptability::Read, Scriptability::Read)
                        | (Scriptability::ReadWrite, Scriptability::ReadWrite)
                        | (Scriptability::Write, Scriptability::Write) => bail!("The scriptability for property {class_name}.{property_name} was unchanged"),
                        _ => {}
                    };

                    existing_property.scriptability = *scriptability;
                }
            }
        }

        Ok(())
    }

    pub fn apply_post_default<'db>(
        &'db self,
        database: &mut ReflectionDatabase<'db>,
    ) -> anyhow::Result<()> {
        // A map of every class to all subclasses, by name. This uses `String`
        // rather than some borrowed variant to get around borrowing `database`
        // as both mutable and immutable
        let mut subclass_map: HashMap<String, Vec<String>> =
            HashMap::with_capacity(database.classes.len());

        for (class_name, class_descriptor) in &database.classes {
            for superclass in database.superclasses(class_descriptor).unwrap() {
                subclass_map
                    .entry(superclass.name.to_string())
                    .or_default()
                    .push(class_name.to_string());
            }
        }

        for (class_name, class_changes) in &self.change {
            for (prop_name, prop_change) in class_changes {
                let default_value = match &prop_change.default_value {
                    Some(value) => value,
                    None => continue,
                };
                let prop_data = database
                    .classes
                    .get(class_name.as_str())
                    // This is already validated pre-default application, so unwrap is fine
                    .unwrap()
                    .properties
                    .get(prop_name.as_str());
                if let Some(prop_data) = prop_data {
                    match (prop_data.data_type.ty(), default_value.ty()) {
                        (existing, new) if existing == new => {}
                        (expected, actual) => bail!(
                            "Bad type given for {class_name}.{prop_name}'s DefaultValue patch.\n\
                            Expected {expected:?}, got {actual:?}"
                        ),
                    }
                }
                let subclass_list = subclass_map.get(class_name).ok_or_else(|| {
                    anyhow!(
                        "Class {} modified in patch file does not exist in database",
                        class_name
                    )
                })?;
                for descendant in subclass_list {
                    let class = database
                        .classes
                        .get_mut(descendant.as_str())
                        .expect("class listed in subclass map should exist");
                    class
                        .default_properties
                        .insert(prop_name.as_str(), default_value.clone());
                }
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Patch {
    #[serde(default)]
    change: HashMap<String, HashMap<String, PropertyChange>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyChange {
    data_type: Option<DataType>,
    alias_for: Option<String>,
    serialization: Option<Serialization>,
    scriptability: Option<Scriptability>,
    default_value: Option<Variant>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub enum DataType {
    /// The property is a regular value of the given type.
    Value(VariantType),

    /// The property is an enum with the given name.
    Enum(String),
}
impl<'db> From<&'db DataType> for rbx_reflection::DataType<'db> {
    fn from(value: &'db DataType) -> Self {
        match value {
            DataType::Value(variant_type) => rbx_reflection::DataType::Value(*variant_type),
            DataType::Enum(e) => rbx_reflection::DataType::Enum(e),
        }
    }
}

impl PropertyChange {
    fn kind(&self) -> Option<PropertyKind<'_>> {
        match (&self.alias_for, &self.serialization) {
            (Some(alias_for), None) => Some(PropertyKind::Alias { alias_for }),

            (None, Some(serialization)) => Some(PropertyKind::Canonical {
                serialization: serialization.into(),
            }),

            (None, None) => None,

            _ => panic!("property changes cannot specify AliasFor and Serialization"),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "Type", rename_all = "PascalCase", deny_unknown_fields)]
pub enum Serialization {
    Serializes,
    DoesNotSerialize,
    #[serde(rename_all = "PascalCase")]
    SerializesAs {
        #[serde(rename = "As")]
        serializes_as: String,
    },
    Migrate(PropertyMigration),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyMigration {
    #[serde(rename = "To")]
    new_property_name: String,
    migration: MigrationOperation,
}

impl<'db> From<&'db Serialization> for PropertySerialization<'db> {
    fn from(value: &'db Serialization) -> Self {
        match value {
            Serialization::Serializes => PropertySerialization::Serializes,
            Serialization::DoesNotSerialize => PropertySerialization::DoesNotSerialize,
            Serialization::SerializesAs { serializes_as } => {
                PropertySerialization::SerializesAs(serializes_as)
            }
            Serialization::Migrate(PropertyMigration {
                new_property_name,
                migration,
            }) => PropertySerialization::Migrate(rbx_reflection::PropertyMigration {
                new_property_name,
                migration: *migration,
            }),
        }
    }
}
