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
    pub value_type: PropertyType<'a>,

    /// A set of the tags that apply to this property.
    #[serde(serialize_with = "crate::serde_util::ordered_set")]
    pub tags: HashSet<PropertyTag>,

    /// Whether this property is directly serialized.
    pub serializes: bool,

    /// If this property is not the canonical property for this data, contains
    /// the name of the canonical descriptor.
    ///
    /// Properties that alias share their storage with each other.
    #[serde(default)]
    pub alias_for: Option<Cow<'a, str>>,

    /// If this property does not serialize, describes the name of the
    /// descriptor that should be serialized instead.
    #[serde(default)]
    pub serializes_as: Option<Cow<'a, str>>,
}

impl<'a> PropertyDescriptor<'a> {
    /// Creates a new `PropertyDescriptor` with the given name and type.
    pub fn new<S: Into<Cow<'a, str>>>(name: S, value_type: PropertyType<'a>) -> Self {
        Self {
            name: name.into(),
            scriptability: Scriptability::None,
            value_type,
            tags: HashSet::new(),
            serializes: true,
            alias_for: None,
            serializes_as: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "Type", content = "Name")]
#[non_exhaustive]
pub enum PropertyType<'a> {
    /// The property is a regular value of the given type.
    Data(VariantType),

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
