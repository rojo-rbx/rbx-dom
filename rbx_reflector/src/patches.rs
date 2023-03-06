use std::{borrow::Cow, collections::HashMap, fs, path::Path};

use anyhow::{anyhow, bail, Context};
use rbx_reflection::{
    DataType, PropertyDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase,
    Scriptability,
};
use serde::Deserialize;

pub struct Patches {
    change: HashMap<String, HashMap<String, PropertyChange>>,
    add: HashMap<String, HashMap<String, PropertyAdd>>,
}

impl Patches {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let mut change = HashMap::new();
        let mut add = HashMap::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let contents = fs::read_to_string(entry.path())?;
            let patch: Patch = serde_yaml::from_str(&contents)
                .with_context(|| format!("Error parsing patch file {}", entry.path().display()))?;

            change.extend(patch.change);
            add.extend(patch.add);
        }

        Ok(Self { change, add })
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

        for (class_name, class_adds) in &self.add {
            let class = database
                .classes
                .get_mut(class_name.as_str())
                .ok_or_else(|| {
                    anyhow!(
                        "Class {} modified in patch file does not exist in database",
                        class_name
                    )
                })?;

            for (property_name, property_add) in class_adds {
                if class.properties.contains_key(property_name.as_str()) {
                    bail!(
                        "Property {}.{} added in patch file was already present",
                        class_name,
                        property_name
                    );
                }

                let name = Cow::Owned(property_name.clone());
                let data_type = property_add.data_type.clone();

                let mut property = PropertyDescriptor::new(name, data_type);

                property.kind = property_add.kind();
                property.scriptability = property_add.scriptability;

                class
                    .properties
                    .insert(Cow::Owned(property_name.clone()), property);
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

    #[serde(default)]
    add: HashMap<String, HashMap<String, PropertyAdd>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyChange {
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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyAdd {
    data_type: DataType<'static>,
    alias_for: Option<String>,
    serialization: Option<Serialization>,
    scriptability: Scriptability,
}

impl PropertyAdd {
    fn kind(&self) -> PropertyKind<'static> {
        match (&self.alias_for, &self.serialization) {
            (Some(alias), None) => PropertyKind::Alias {
                alias_for: Cow::Owned(alias.clone()),
            },

            (None, Some(serialization)) => PropertyKind::Canonical {
                serialization: serialization.clone().into(),
            },

            _ => panic!("property additions must specify AliasFor xor Serialization"),
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
}

impl From<Serialization> for PropertySerialization<'_> {
    fn from(value: Serialization) -> Self {
        match value {
            Serialization::Serializes => PropertySerialization::Serializes,
            Serialization::DoesNotSerialize => PropertySerialization::DoesNotSerialize,
            Serialization::SerializesAs { serializes_as } => {
                PropertySerialization::SerializesAs(Cow::Owned(serializes_as))
            }
        }
    }
}
