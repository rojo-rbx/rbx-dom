use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use rbx_types::{Variant, VariantType};
use serde::{Deserialize, Serialize};

use crate::{ClassTag, PropertyTag};

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
}

impl<'a> ReflectionDatabase<'a> {
    /// Creates an empty `ReflectionDatabase` with a version number of 0.0.0.0.
    pub fn new() -> Self {
        Self {
            version: [0, 0, 0, 0],
            classes: HashMap::new(),
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
#[serde(tag = "Type", rename_all = "PascalCase")]
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
#[serde(tag = "Type", content = "Name")]
#[non_exhaustive]
pub enum PropertySerialization<'a> {
    /// The property serializes as itself.
    Serializes,

    /// The property does not serialize.
    DoesNotSerialize,

    /// The property aliases a property with the given name and should serialize
    /// from that property descriptor instead.
    SerializesAs(Cow<'a, str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type", content = "Name")]
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
