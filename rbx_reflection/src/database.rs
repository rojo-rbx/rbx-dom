// Creating a default reflection database implicitly doesn't really make sense
// for most cases.
#![allow(clippy::new_without_default)]

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use rbx_types::{Variant, VariantType};
use serde::{Deserialize, Serialize};

use crate::{ClassTag, PropertyMigration, PropertyTag};

/// Contains information extracted from Roblox to describe all known Instances
/// and enums.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct ReflectionDatabase<'a> {
    /// The Roblox release that this reflection database was generated from.
    pub version: [u32; 4],

    /// All of the the known classes in the database.
    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub classes: HashMap<Cow<'a, str>, ClassDescriptor<'a>>,

    /// All of the known enums in the database.
    #[serde(default, serialize_with = "crate::serde_util::ordered_map")]
    pub enums: HashMap<Cow<'a, str>, EnumDescriptor<'a>>,
}

impl<'a> ReflectionDatabase<'a> {
    /// Creates an empty `ReflectionDatabase` with a version number of 0.0.0.0.
    pub fn new() -> Self {
        Self {
            version: [0, 0, 0, 0],
            classes: HashMap::new(),
            enums: HashMap::new(),
        }
    }

    /// Returns a list of superclasses for the provided ClassDescriptor. This
    /// list will start with the provided class and end with `Instance`.
    pub fn superclasses(
        &'a self,
        descriptor: &'a ClassDescriptor<'a>,
    ) -> Option<Vec<&'a ClassDescriptor<'a>>> {
        // As of the time of writing (14 March 2024), the class with the most
        // superclasses has 6 of them.
        let mut list = Vec::with_capacity(6);
        let mut current_class = Some(descriptor);

        while let Some(class) = current_class {
            list.push(class);
            current_class = class.superclass.as_ref().and_then(|s| self.classes.get(s));
        }

        Some(list)
    }

    /// Returns an iterator of superclasses for the provided ClassDescriptor. This
    /// iterator will start with the provided class and end with `Instance`.
    pub fn superclasses_iter(
        &'a self,
        descriptor: &'a ClassDescriptor<'a>,
    ) -> impl Iterator<Item = &'a ClassDescriptor<'a>> {
        std::iter::successors(Some(descriptor), move |class| {
            class.superclass.as_ref().and_then(|s| self.classes.get(s))
        })
    }

    /// This mimics the behavior of the Roblox method `Instance:IsA(ClassName)`.
    /// Returns whether `superclass_descriptor` is a superclass of `descriptor`.
    pub fn has_superclass(
        &self,
        descriptor: &ClassDescriptor,
        superclass_descriptor: &ClassDescriptor,
    ) -> bool {
        self.superclasses_iter(descriptor)
            .any(|class_descriptor| class_descriptor.name == superclass_descriptor.name)
    }

    /// Finds the default value of a property given its name and a class that
    /// contains or inherits the property. Returns `Some(&Variant)` if a default
    /// value exists, None otherwise.
    pub fn find_default_property(
        &'a self,
        mut class: &'a ClassDescriptor<'a>,
        property_name: &str,
    ) -> Option<&'a Variant> {
        loop {
            match class.default_properties.get(property_name) {
                None => {
                    class = self
                        .classes
                        .get(class.superclass.as_ref()?)
                        .expect("superclass that is Some should exist in reflection database")
                }
                default_value => return default_value,
            }
        }
    }
}

/// Describes a class of Instance, its properties, and its relation to other
/// classes of Instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct ClassDescriptor<'a> {
    /// The name of the class, like "Folder" or "FlagStand".
    pub name: Cow<'a, str>,

    /// A set of all of the tags attached to this class.
    #[serde(serialize_with = "crate::serde_util::ordered_set")]
    pub tags: HashSet<ClassTag>,

    /// If this class descends from another class, contains the name of that
    /// class.
    #[serde(default)]
    pub superclass: Option<Cow<'a, str>>,

    /// A map of all of the properties available on this class.
    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub properties: HashMap<Cow<'a, str>, PropertyDescriptor<'a>>,

    /// A map of the default properties for this instance if a value is not
    /// defined in serialization or freshly inserted with `Instance.new`.
    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub default_properties: HashMap<Cow<'a, str>, Variant>,
}

impl<'a> ClassDescriptor<'a> {
    /// Creates a new `ClassDescriptor` with the given name.
    pub fn new<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Self {
            name: name.into(),
            tags: HashSet::new(),
            superclass: None,
            properties: HashMap::new(),
            default_properties: HashMap::new(),
        }
    }
}

/// Describes a property on an Instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct PropertyDescriptor<'a> {
    /// The name of the property, like "Position" or "heat_xml".
    pub name: Cow<'a, str>,

    /// The maximum access to this property available to Lua.
    pub scriptability: Scriptability,

    /// The type of the value described by this descriptor.
    pub data_type: DataType<'a>,

    /// A set of the tags that apply to this property.
    #[serde(serialize_with = "crate::serde_util::ordered_set")]
    pub tags: HashSet<PropertyTag>,

    /// The kind of property this is, including whether it is canonical.
    pub kind: PropertyKind<'a>,
}

impl<'a> PropertyDescriptor<'a> {
    /// Creates a new `PropertyDescriptor` with the given name and type.
    pub fn new<S: Into<Cow<'a, str>>>(name: S, data_type: DataType<'a>) -> Self {
        Self {
            name: name.into(),
            scriptability: Scriptability::None,
            data_type,
            tags: HashSet::new(),
            kind: PropertyKind::Canonical {
                serialization: PropertySerialization::Serializes,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PropertyKind<'a> {
    /// This property is canonical.
    #[serde(rename_all = "PascalCase")]
    Canonical {
        serialization: PropertySerialization<'a>,
    },

    /// This property is an alias to another property that is canonical.
    #[serde(rename_all = "PascalCase")]
    Alias { alias_for: Cow<'a, str> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PropertySerialization<'a> {
    /// The property serializes as itself.
    Serializes,

    /// The property does not serialize.
    DoesNotSerialize,

    /// The property aliases a property with the given name and should serialize
    /// from that property descriptor instead.
    SerializesAs(Cow<'a, str>),

    /// The property was originally serialized as itself, but should be migrated
    /// to a new property on deserialization. If the new property already
    /// exists, this property should be ignored.
    Migrate(PropertyMigration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DataType<'a> {
    /// The property is a regular value of the given type.
    Value(VariantType),

    /// The property is an enum with the given name.
    Enum(Cow<'a, str>),
}

/// Defines how Lua can access a property, if at all.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Scriptability {
    /// The property is not accessible to Lua scripts at all.
    None,

    /// The property can be read from or written to with regular assignments.
    ReadWrite,

    /// The property can only be read from.
    Read,

    /// The property can only be written to.
    Write,

    /// The property can only be modified indirectly by Lua scripts.
    ///
    /// A common example is the `Tags` property, which is readable and writable
    /// through methods on `CollectionService`.
    Custom,
}

/// Describes a Roblox enum and all of its items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnumDescriptor<'a> {
    /// The name of the enum, like "FormFactor" or "Material".
    pub name: Cow<'a, str>,

    /// All of the members of this enum, stored as a map from names to values.
    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub items: HashMap<Cow<'a, str>, u32>,
}

impl<'a> EnumDescriptor<'a> {
    /// Create a new `EnumDescriptor` with the given name and no items.
    pub fn new<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Self {
            name: name.into(),
            items: HashMap::new(),
        }
    }
}
