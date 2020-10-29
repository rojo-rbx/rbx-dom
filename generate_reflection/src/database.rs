use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use rbx_reflection::{
    ClassDescriptor, DataType, PropertyDescriptor, PropertyKind, PropertySerialization,
    PropertyTag, ReflectionDatabase as Database, Scriptability,
};
use rbx_types::VariantType;
use serde::{Deserialize, Serialize};

use crate::api_dump::{Dump, DumpClassMember, ValueCategory};

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReflectionDatabase(pub Database<'static>);

impl ReflectionDatabase {
    pub fn new() -> Self {
        Self(Database::new())
    }

    /// Adds all of the classes from the given API dump to the reflection
    /// database.
    pub fn populate_from_dump(&mut self, dump: &Dump) -> anyhow::Result<()> {
        for dump_class in &dump.classes {
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

                    let scriptability = if tags.contains(&PropertyTag::NotScriptable) {
                        Scriptability::None
                    } else if tags.contains(&PropertyTag::ReadOnly) {
                        Scriptability::Read
                    } else {
                        Scriptability::ReadWrite
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

                    // FIXME
                    // property.kind = kind;

                    properties.insert(Cow::Owned(dump_property.name.clone()), property);
                }
            }

            let mut class = ClassDescriptor::new(dump_class.name.clone());
            class.tags = tags;
            class.superclass = superclass;
            class.properties = properties;

            self.0
                .classes
                .insert(Cow::Owned(dump_class.name.clone()), class);
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

        // These types are not generally implemented right now.
        "QDir" | "QFont" => return None,

        _ => panic!("Unknown type {}", value),
    })
}
