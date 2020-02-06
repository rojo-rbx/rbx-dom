use std::{borrow::Cow, collections::HashMap};

use rbx_reflection::{
    ClassDescriptor, InstanceTags, PropertyDescriptor, PropertyTags, PropertyType,
    ReflectionDatabase as Database, Scriptability,
};
use rbx_types::VariantType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

use crate::{
    api_dump::{Dump, DumpClassMember, ValueCategory, ValueType},
    property_patches::PropertyPatches,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReflectionDatabase(pub Database<'static>);

impl ReflectionDatabase {
    pub fn new() -> Self {
        Self(Database::new())
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

            let mut tags = InstanceTags::empty();
            for dump_tag in &dump_class.tags {
                tags |= dump_tag.parse().unwrap();
            }

            let mut properties = HashMap::new();

            for member in &dump_class.members {
                if let DumpClassMember::Property(dump_property) = member {
                    let mut tags = PropertyTags::empty();
                    for dump_tag in &dump_property.tags {
                        tags |= dump_tag.parse().unwrap();
                    }

                    let scriptability = if tags.contains(PropertyTags::NOT_SCRIPTABLE) {
                        Scriptability::None
                    } else if tags.contains(PropertyTags::READ_ONLY) {
                        Scriptability::Read
                    } else {
                        Scriptability::ReadWrite
                    };

                    let serializes = !dump_property.tags.iter().any(|v| v == "ReadOnly")
                        && dump_property.serialization.can_save;

                    let type_name = &dump_property.value_type.name;
                    let value_type = match dump_property.value_type.category {
                        ValueCategory::Enum => PropertyType::Enum(type_name.clone().into()),
                        ValueCategory::Primitive | ValueCategory::DataType => {
                            let variant_type = variant_type_from_str(type_name);
                            PropertyType::Data(variant_type)
                        }
                        ValueCategory::Class => PropertyType::Data(VariantType::Ref),
                    };

                    let mut property =
                        PropertyDescriptor::new(dump_property.name.clone(), value_type);
                    property.scriptability = scriptability;
                    property.tags = tags;
                    property.serializes = true;

                    properties.insert(Cow::Owned(dump_property.name.clone()), property);
                }
            }

            let mut class = ClassDescriptor::new(dump_class.name.clone());
            class.superclass = superclass;
            class.properties = properties;

            // tags

            self.0
                .classes
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
                .0
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

                if let Some(canonical_name) = &property_change.canonical_name {
                    existing_property.alias_for = Some(canonical_name.clone());
                    existing_property.serializes = false;
                }

                if let Some(serialized_name) = &property_change.serialized_name {
                    existing_property.serializes_as = Some(serialized_name.clone());
                    existing_property.serializes = true;
                }
            }
        }

        for (class_name, class_adds) in &property_patches.add {
            let class = self
                .0
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

                let mut property = PropertyDescriptor::new(name, value_type);
                property.alias_for = property_add.canonical_name.clone();
                property.serializes_as = property_add.serialized_name.clone();
                property.scriptability = property_add.scriptability;
                property.serializes = property_add.serializes;

                class
                    .properties
                    .insert(Cow::Owned(property_name.clone()), property);
            }
        }

        Ok(())
    }

    /// Validate that the state of the database makes sense.
    pub fn validate(&self) {
        // for (class_name, class) in &self.classes {
        //     for (property_name, property) in &class.properties {
        //         if let Some(canonical_name) = &property.canonical_name {
        //             let canonical_property = match class.properties.get(canonical_name) {
        //                 Some(value) => value,
        //                 None => panic!(
        //                     "Property {}.{} refers to canonical property ({}) that does not exist.",
        //                     class_name, property_name, canonical_name
        //                 ),
        //             };

        //             if !canonical_property.is_canonical {
        //                 panic!("Property {}.{} is marked as the canonical form of {}, but is not canonical!",
        //                     class_name, canonical_name, property_name);
        //             }
        //         }

        //         if let Some(serialized_name) = &property.serialized_name {
        //             let _serialized_property = match class.properties.get(serialized_name) {
        //                 Some(value) => value,
        //                 None => panic!(
        //                     "Property {}.{} refers to serialized property ({}) that does not exist.",
        //                     class_name, property_name, serialized_name
        //                 ),
        //             };
        //         }

        //         if property.is_canonical {
        //             let mut probably_mistake = false;

        //             if property_name.chars().next().unwrap().is_lowercase() {
        //                 probably_mistake = true;
        //             }

        //             if property_name.ends_with("_xml") {
        //                 probably_mistake = true;
        //             }

        //             if probably_mistake {
        //                 println!(
        //                     "Property {}.{} doesn't look canonical",
        //                     class_name, property_name
        //                 );
        //             }
        //         }
        //     }
        // }
    }
}

fn variant_type_from_str(value: &str) -> VariantType {
    match value {
        "Axes" => VariantType::Axes,
        "BinaryString" => VariantType::BinaryString,
        "BrickColor" => VariantType::BrickColor,
        "bool" => VariantType::Bool,
        "CFrame" => VariantType::CFrame,
        "Color3" => VariantType::Color3,
        "ColorSequence" => VariantType::ColorSequence,
        "Content" => VariantType::Content,
        "Faces" => VariantType::Faces,
        "float" => VariantType::Float32,
        "double" => VariantType::Float64,
        "int" => VariantType::Int32,
        "int64" => VariantType::Int64,
        "NumberRange" => VariantType::NumberRange,
        "NumberSequence" => VariantType::NumberSequence,
        "PhysicalProperties" => VariantType::PhysicalProperties,
        "Ray" => VariantType::Ray,
        "Rect" => VariantType::Rect,
        "Instance" => VariantType::Ref,
        "string" => VariantType::String,
        "UDim" => VariantType::UDim,
        "UDim2" => VariantType::UDim2,
        "Vector2" => VariantType::Vector2,
        "Vector2int16" => VariantType::Vector2int16,
        "Vector3" => VariantType::Vector3,
        "Vector3int16" => VariantType::Vector3int16,

        "ProtectedString" => VariantType::String,

        _ => panic!("Unknown type {}", value),
    }
}

#[derive(Debug, Snafu)]
pub enum Error {}

pub use Error as DatabaseError;
