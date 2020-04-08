use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use rbx_types::{Variant, VariantType};
use serde::{Deserialize, Serialize};

use crate::{ClassTag, PropertyTag};

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
    pub fn new() -> Self {
        Self {
            version: [0, 0, 0, 0],
            classes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct ClassDescriptor<'a> {
    pub name: Cow<'a, str>,

    #[serde(serialize_with = "crate::serde_util::ordered_set")]
    pub tags: HashSet<ClassTag>,

    #[serde(default)]
    pub superclass: Option<Cow<'a, str>>,

    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub properties: HashMap<Cow<'a, str>, PropertyDescriptor<'a>>,

    #[serde(serialize_with = "crate::serde_util::ordered_map")]
    pub default_properties: HashMap<Cow<'a, str>, Variant>,
}

impl<'a> ClassDescriptor<'a> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct PropertyDescriptor<'a> {
    pub name: Cow<'a, str>,
    pub scriptability: Scriptability,
    pub value_type: PropertyType<'a>,

    #[serde(serialize_with = "crate::serde_util::ordered_set")]
    pub tags: HashSet<PropertyTag>,

    pub serializes: bool,

    #[serde(default)]
    pub alias_for: Option<Cow<'a, str>>,

    #[serde(default)]
    pub serializes_as: Option<Cow<'a, str>>,
}

impl<'a> PropertyDescriptor<'a> {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Scriptability {
    /// The property is not scriptable at all.
    None,

    /// The property can be read from or written to with regular assignments.
    ReadWrite,

    /// The property can only be read from.
    Read,

    /// The property can only be written to.
    Write,

    /// The property can only be modified indirectly.
    ///
    /// A common example is the `Tags` property, which is writable through
    /// methods on `CollectionService`.
    Custom,
}
