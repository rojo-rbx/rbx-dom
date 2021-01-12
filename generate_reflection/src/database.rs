use std::{borrow::Cow, collections::HashMap};

use snafu::Snafu;

use crate::{
    api_dump::{Dump, DumpClassMember},
    property_patches::PropertyPatches,
    reflection_types::{
        RbxClassDescriptor, RbxInstanceTags, RbxPropertyDescriptor, RbxPropertyScriptability,
        RbxPropertyTags, RbxPropertyTypeDescriptor,
    },
};

pub struct ReflectionDatabase {
    pub studio_version: [u32; 4],
    pub classes: HashMap<Cow<'static, str>, RbxClassDescriptor>,
}

impl ReflectionDatabase {
    pub fn new() -> Self {
        Self {
            studio_version: [0, 0, 0, 0],
            classes: HashMap::new(),
        }
    }

    /// Adds all of the classes from the given API dump to the reflection
    /// database.
    pub fn populate_from_dump(&mut self, dump: &Dump) -> Result<(), Error> {
        for dump_class in &dump.classes {
            let superclass = if dump_class.superclass == "<<<ROOT>>>" {
                None
            } else {
                Some(Cow::Owned(dump_class.superclass.clone()))
            };

            let tags = RbxInstanceTags::from_dump_tags(&dump_class.tags);

            let mut properties = HashMap::new();

            for member in &dump_class.members {
                if let DumpClassMember::Property(dump_property) = member {
                    let tags = RbxPropertyTags::from_dump_tags(&dump_property.tags);

                    let scriptability = if tags.contains(RbxPropertyTags::NOT_SCRIPTABLE) {
                        RbxPropertyScriptability::None
                    } else if tags.contains(RbxPropertyTags::READ_ONLY) {
                        RbxPropertyScriptability::Read
                    } else {
                        RbxPropertyScriptability::ReadWrite
                    };

                    let serializes = !dump_property.tags.iter().any(|v| v == "ReadOnly")
                        && dump_property.serialization.can_save;

                    let property = RbxPropertyDescriptor {
                        name: Cow::Owned(dump_property.name.clone()),
                        value_type: RbxPropertyTypeDescriptor::from(&dump_property.value_type),
                        tags,

                        is_canonical: true,
                        canonical_name: None,
                        serialized_name: None,
                        scriptability,
                        serializes,
                    };

                    properties.insert(Cow::Owned(dump_property.name.clone()), property);
                }
            }

            let class = RbxClassDescriptor {
                name: Cow::Owned(dump_class.name.clone()),
                superclass,
                tags,
                properties,
                default_properties: HashMap::new(),
            };

            self.classes
                .insert(Cow::Owned(dump_class.name.clone()), class);
        }

        Ok(())
    }

    /// Add and update information based on rbx-dom's hand-written property
    /// patch file.
    pub fn populate_from_patches(
        &mut self,
        property_patches: &PropertyPatches,
    ) -> Result<(), Error> {
        for (class_name, class_changes) in &property_patches.change {
            let class = self
                .classes
                .get_mut(class_name.as_str())
                .unwrap_or_else(|| {
                    panic!("Class {} defined in patch file wasn't present", class_name)
                });

            for (property_name, property_change) in class_changes {
                let existing_property = class
                    .properties
                    .get_mut(property_name.as_str())
                    .unwrap_or_else(|| {
                        panic!(
                            "Property {}.{} did not exist in dump",
                            class_name, property_name
                        )
                    });

                println!("{}.{} changed", class_name, property_name);

                existing_property.is_canonical = property_change.canonical_name.is_none();

                if let Some(canonical_name) = &property_change.canonical_name {
                    existing_property.canonical_name = Some(canonical_name.clone());
                    existing_property.serializes = false;
                }

                if let Some(serialized_name) = &property_change.serialized_name {
                    existing_property.serialized_name = Some(serialized_name.clone());
                    existing_property.serializes = true;
                }

                if let Some(scriptability) = property_change.scriptability {
                    existing_property.scriptability = scriptability;
                }
            }
        }

        for (class_name, class_adds) in &property_patches.add {
            let class = self
                .classes
                .get_mut(class_name.as_str())
                .unwrap_or_else(|| {
                    panic!("Class {} defined in patch file wasn't present", class_name)
                });

            for (property_name, property_add) in class_adds {
                if class.properties.contains_key(property_name.as_str()) {
                    panic!(
                        "Property {}.{} marked for addition in patch was already present",
                        class_name, property_name
                    );
                }

                println!("{}.{} added", class_name, property_name);

                let name = Cow::Owned(property_name.clone());
                let value_type = property_add.property_type.clone();
                let is_canonical = property_add.canonical_name.is_none();
                let canonical_name = property_add.canonical_name.clone();
                let serialized_name = property_add.serialized_name.clone();
                let scriptability = property_add.scriptability;
                let serializes = property_add.serializes;

                let property = RbxPropertyDescriptor {
                    name,
                    value_type,
                    is_canonical,
                    canonical_name,
                    serialized_name,
                    scriptability,
                    serializes,

                    tags: RbxPropertyTags::empty(),
                };

                class
                    .properties
                    .insert(Cow::Owned(property_name.clone()), property);
            }
        }

        Ok(())
    }

    /// Validate that the state of the database makes sense.
    pub fn validate(&self) {
        for (class_name, class) in &self.classes {
            for (property_name, property) in &class.properties {
                if let Some(canonical_name) = &property.canonical_name {
                    let canonical_property = match class.properties.get(canonical_name) {
                        Some(value) => value,
                        None => panic!(
                            "Property {}.{} refers to canonical property ({}) that does not exist.",
                            class_name, property_name, canonical_name
                        ),
                    };

                    if !canonical_property.is_canonical {
                        panic!("Property {}.{} is marked as the canonical form of {}, but is not canonical!",
                            class_name, canonical_name, property_name);
                    }
                }

                if let Some(serialized_name) = &property.serialized_name {
                    let _serialized_property = match class.properties.get(serialized_name) {
                        Some(value) => value,
                        None => panic!(
                            "Property {}.{} refers to serialized property ({}) that does not exist.",
                            class_name, property_name, serialized_name
                        ),
                    };
                }

                if property.is_canonical {
                    let mut probably_mistake = false;

                    if property_name.chars().next().unwrap().is_lowercase() {
                        probably_mistake = true;
                    }

                    if property_name.ends_with("_xml") {
                        probably_mistake = true;
                    }

                    if probably_mistake {
                        println!(
                            "Property {}.{} doesn't look canonical",
                            class_name, property_name
                        );
                    }
                }
            }
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {}

pub use Error as DatabaseError;
