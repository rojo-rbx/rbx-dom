use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context};
use rbx_reflection::{
    DataType, PropertyDescriptor, PropertyKind, PropertyMigration, PropertySerialization,
    ReflectionDatabase, Scriptability,
};
use rbx_types::Variant;
use serde::Deserialize;

pub struct PatchSource {
    path: PathBuf,
    contents: String,
}

pub struct PatchSources {
    files: Vec<PatchSource>,
}

impl PatchSources {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            let contents = fs::read_to_string(&path)?;

            files.push(PatchSource { path, contents });
        }

        Ok(Self { files })
    }

    pub fn parse(&self) -> anyhow::Result<Patches<'_>> {
        let mut change = HashMap::new();
        let mut add = HashMap::new();

        for file in &self.files {
            let patch: Patch = yaml_serde::from_str(&file.contents)
                .with_context(|| format!("Error parsing patch file {}", file.path.display()))?;

            change.extend(patch.change);
            add.extend(patch.add);
        }

        Ok(Patches { change, add })
    }
}

pub struct Patches<'a> {
    change: HashMap<&'a str, HashMap<&'a str, PropertyChange<'a>>>,
    add: HashMap<&'a str, HashMap<&'a str, PropertyAdd<'a>>>,
}

impl<'a> Patches<'a> {
    pub fn apply_pre_default<'db>(
        &'db self,
        database: &mut ReflectionDatabase<'db>,
    ) -> anyhow::Result<()> {
        for (class_name, class_adds) in &self.add {
            let class = database.classes.get_mut(class_name).ok_or_else(|| {
                anyhow!(
                    "Class {} referenced in add patch does not exist in database",
                    class_name
                )
            })?;

            for (property_name, property_add) in class_adds {
                if class.properties.contains_key(property_name) {
                    bail!(
                        "Property {}.{} added in patch file already exists in database",
                        class_name,
                        property_name
                    );
                }

                let mut property =
                    PropertyDescriptor::new(property_name, property_add.data_type.clone());
                property.kind = property_add.kind();
                property.scriptability = property_add.scriptability;

                class.properties.insert(property_name, property);
            }
        }

        for (class_name, class_changes) in &self.change {
            let class = database.classes.get_mut(class_name).ok_or_else(|| {
                anyhow!(
                    "Class {} modified in patch file does not exist in database",
                    class_name
                )
            })?;

            for (property_name, property_change) in class_changes {
                let existing_property =
                    class.properties.get_mut(property_name).ok_or_else(|| {
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
                    .get(class_name)
                    // This is already validated pre-default application, so unwrap is fine
                    .unwrap()
                    .properties
                    .get(prop_name);
                if let Some(prop_data) = prop_data {
                    match (prop_data.data_type.ty(), default_value.ty()) {
                        (existing, new) if existing == new => {}
                        (expected, actual) => bail!(
                            "Bad type given for {class_name}.{prop_name}'s DefaultValue patch.\n\
                            Expected {expected:?}, got {actual:?}"
                        ),
                    }
                }
                let subclass_list = subclass_map.get(*class_name).ok_or_else(|| {
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
                        .insert(prop_name, default_value.clone());
                }
            }
        }

        for (class_name, class_adds) in &self.add {
            for (prop_name, prop_add) in class_adds {
                let default_value = match &prop_add.default_value {
                    Some(value) => value,
                    None => continue,
                };
                let prop_data = database
                    .classes
                    .get(class_name)
                    .unwrap()
                    .properties
                    .get(prop_name);
                if let Some(prop_data) = prop_data {
                    match (prop_data.data_type.ty(), default_value.ty()) {
                        (existing, new) if existing == new => {}
                        (expected, actual) => bail!(
                            "Bad type given for {class_name}.{prop_name}'s DefaultValue patch.\n\
                            Expected {expected:?}, got {actual:?}"
                        ),
                    }
                }
                let subclass_list = subclass_map.get(*class_name).ok_or_else(|| {
                    anyhow!(
                        "Class {} referenced in add patch does not exist in database",
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
                        .insert(prop_name, default_value.clone());
                }
            }
        }

        Ok(())
    }

    /// Validates that all migrations in this `Patches` point to valid
    /// properties.
    ///
    /// This could be done in `apply_pre_default` but it runs into issues with
    /// the borrow checker, so it is easier done after mutating the database
    /// is done.
    pub fn validate_migrations<'db>(
        &'db self,
        database: &ReflectionDatabase<'db>,
    ) -> anyhow::Result<()> {
        for (class_name, class_changes) in &self.change {
            let class = database.classes.get(class_name).ok_or_else(|| {
                anyhow!(
                    "Class {} referenced in change patch does not exist in database",
                    class_name
                )
            })?;
            for (prop_name, prop_change) in class_changes {
                let Some(new_serialization) = &prop_change.serialization else {
                    continue;
                };
                match new_serialization {
                    Serialization::Migrate(migration) => {
                        for new_name in migration.new_property_names() {
                            if !class.properties.contains_key(*new_name) {
                                bail!("Property {class_name}.{new_name} referenced in Migration for {class_name}.{prop_name} does not exist in database")
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Patch<'a> {
    #[serde(default)]
    #[serde(borrow)]
    change: HashMap<&'a str, HashMap<&'a str, PropertyChange<'a>>>,
    #[serde(default)]
    add: HashMap<&'a str, HashMap<&'a str, PropertyAdd<'a>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyChange<'a> {
    #[serde(with = "yaml_serde::with::singleton_map_recursive")]
    #[serde(default)]
    data_type: Option<DataType<'a>>,

    alias_for: Option<&'a str>,
    serialization: Option<Serialization<'a>>,
    scriptability: Option<Scriptability>,

    #[serde(with = "yaml_serde::with::singleton_map_recursive")]
    #[serde(default)]
    default_value: Option<Variant>,
}

impl<'a> PropertyChange<'a> {
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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyAdd<'a> {
    #[serde(with = "yaml_serde::with::singleton_map_recursive")]
    data_type: DataType<'a>,

    alias_for: Option<&'a str>,
    serialization: Option<Serialization<'a>>,
    scriptability: Scriptability,

    #[serde(with = "yaml_serde::with::singleton_map_recursive")]
    #[serde(default)]
    default_value: Option<Variant>,
}

impl<'a> PropertyAdd<'a> {
    fn kind<'db>(&'db self) -> PropertyKind<'db> {
        match (&self.alias_for, &self.serialization) {
            (Some(alias_for), None) => PropertyKind::Alias { alias_for },

            (None, Some(serialization)) => PropertyKind::Canonical {
                serialization: serialization.into(),
            },

            (Some(_), Some(_)) => {
                panic!("property additions cannot specify both AliasFor and Serialization")
            }

            (None, None) => {
                panic!("property additions must specify either AliasFor or Serialization")
            }
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "Type", rename_all = "PascalCase", deny_unknown_fields)]
enum Serialization<'a> {
    Serializes,
    DoesNotSerialize,
    #[serde(rename_all = "PascalCase")]
    SerializesAs {
        #[serde(rename = "As")]
        serializes_as: &'a str,
    },
    Migrate(PropertyMigration<'a>),
}

impl<'db, 'a> From<&'db Serialization<'a>> for PropertySerialization<'db> {
    fn from(value: &'db Serialization) -> Self {
        match value {
            Serialization::Serializes => PropertySerialization::Serializes,
            Serialization::DoesNotSerialize => PropertySerialization::DoesNotSerialize,
            Serialization::SerializesAs { serializes_as } => {
                PropertySerialization::SerializesAs(serializes_as)
            }
            Serialization::Migrate(migration) => PropertySerialization::Migrate(migration.clone()),
        }
    }
}
