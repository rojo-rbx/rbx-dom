//! Defines changes and additions to the reflection dump that add and fix up
//! information.
//!
//! See the `patches/` directory for input.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, bail, Context};
use rbx_reflection::{
    DataType, PropertyDescriptor, PropertyKind, ReflectionDatabase, Scriptability,
};
use serde::Deserialize;

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
    pub alias_for: Option<String>,
    pub serialization: Option<PropertySerialization>,
    pub scriptability: Option<Scriptability>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PropertyAdd {
    pub data_type: DataType<'static>,
    pub alias_for: Option<String>,
    pub serialization: Option<PropertySerialization>,
    pub scriptability: Scriptability,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "Type", rename_all = "PascalCase", deny_unknown_fields)]
pub enum PropertySerialization {
    Serializes,
    DoesNotSerialize,
    #[serde(rename_all = "PascalCase")]
    SerializesAs {
        #[serde(rename = "As")]
        serializes_as: String,
    },
}

impl From<PropertySerialization> for rbx_reflection::PropertySerialization<'_> {
    fn from(value: PropertySerialization) -> Self {
        match value {
            PropertySerialization::Serializes => rbx_reflection::PropertySerialization::Serializes,
            PropertySerialization::DoesNotSerialize => {
                rbx_reflection::PropertySerialization::DoesNotSerialize
            }
            PropertySerialization::SerializesAs { serializes_as } => {
                rbx_reflection::PropertySerialization::SerializesAs(Cow::Owned(serializes_as))
            }
        }
    }
}

impl PropertyPatches {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let mut all_patches = PropertyPatches::default();

        for entry in fs_err::read_dir(dir)? {
            let entry = entry?;
            let contents = fs_err::read_to_string(entry.path())?;
            let parsed: PropertyPatches = serde_yaml::from_str(&contents)
                .with_context(|| format!("Error parsing patch file {}", entry.path().display()))?;

            all_patches.change.extend(parsed.change);
            all_patches.add.extend(parsed.add);
        }

        Ok(all_patches)
    }

    pub fn apply(self, database: &mut ReflectionDatabase<'static>) -> anyhow::Result<()> {
        for (class_name, class_changes) in &self.change {
            let class = database
                .classes
                .get_mut(class_name.as_str())
                .ok_or_else(|| {
                    anyhow!(
                        "Class {} modified in patch file did not exist in database",
                        class_name
                    )
                })?;

            for (property_name, property_change) in class_changes {
                let existing_property = class
                    .properties
                    .get_mut(property_name.as_str())
                    .ok_or_else(|| {
                        anyhow!(
                            "Property {}.{} modified in patch file did not exist in database",
                            class_name,
                            property_name
                        )
                    })?;

                log::debug!("Property {}.{} changed", class_name, property_name);

                if let Some(kind) = property_change.kind() {
                    existing_property.kind = kind;
                }

                if let Some(scriptability) = &property_change.scriptability {
                    existing_property.scriptability = *scriptability;
                }
            }
        }

        for (class_name, class_adds) in &self.add {
            let class = database
                .classes
                .get_mut(class_name.as_str())
                .ok_or_else(|| {
                    anyhow!("Class {} modified in patch file wasn't present", class_name)
                })?;

            for (property_name, property_add) in class_adds {
                if class.properties.contains_key(property_name.as_str()) {
                    bail!(
                        "Property {}.{} added in patch file was already present",
                        class_name,
                        property_name
                    );
                }

                log::debug!("Property {}.{} added", class_name, property_name);

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
