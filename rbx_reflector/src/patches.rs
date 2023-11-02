use std::{borrow::Cow, collections::HashMap, fs, path::Path};

use anyhow::{anyhow, bail, Context};
use rbx_reflection::{
    DataType, PropertyKind, PropertyMigration, PropertySerialization, ReflectionDatabase,
    Scriptability,
};
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

    pub fn apply(self, database: &mut ReflectionDatabase) -> anyhow::Result<()> {
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
                    existing_property.data_type = data_type.clone();
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
    data_type: Option<DataType<'static>>,
    alias_for: Option<String>,
    serialization: Option<Serialization>,
    scriptability: Option<Scriptability>,
}

impl PropertyChange {
    fn kind(&self) -> Option<PropertyKind<'static>> {
        match (&self.alias_for, &self.serialization) {
            (Some(alias), None) => Some(PropertyKind::Alias {
                alias_for: Cow::Owned(alias.clone()),
            }),

            (None, Some(serialization)) => Some(PropertyKind::Canonical {
                serialization: serialization.clone().into(),
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

impl From<Serialization> for PropertySerialization<'_> {
    fn from(value: Serialization) -> Self {
        match value {
            Serialization::Serializes => PropertySerialization::Serializes,
            Serialization::DoesNotSerialize => PropertySerialization::DoesNotSerialize,
            Serialization::SerializesAs { serializes_as } => {
                PropertySerialization::SerializesAs(Cow::Owned(serializes_as))
            }
            Serialization::Migrate(migration) => PropertySerialization::Migrate(migration),
        }
    }
}
