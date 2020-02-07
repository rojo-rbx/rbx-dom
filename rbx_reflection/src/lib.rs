#[macro_use]
mod tag_util;

mod serde_util;

use std::{borrow::Cow, collections::HashMap, str::FromStr};

use rbx_types::{Variant, VariantType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub struct ReflectionDatabase<'a> {
    /// The Roblox release that this reflection database was generated from.
    pub version: [u32; 4],

    /// All of the the known classes in the database.
    #[serde(serialize_with = "serde_util::ordered_map")]
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

    pub tags: ClassTags,

    #[serde(default)]
    pub superclass: Option<Cow<'a, str>>,

    #[serde(serialize_with = "serde_util::ordered_map")]
    pub properties: HashMap<Cow<'a, str>, PropertyDescriptor<'a>>,

    #[serde(serialize_with = "serde_util::ordered_map")]
    pub default_properties: HashMap<Cow<'a, str>, Variant>,
}

impl<'a> ClassDescriptor<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Self {
            name: name.into(),
            tags: ClassTags::empty(),
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
    pub tags: PropertyTags,

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
            tags: PropertyTags::empty(),
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

// Tags found via:
// jq '[.Classes | .[] | .Tags // empty] | add | unique' api-dump.json
bitterflag! {
    ClassTags + ClassTagsIntoIter: u32 {
        const DEPRECATED = 1;
        const NOT_BROWSABLE = 2;
        const NOT_CREATABLE = 4;
        const NOT_REPLICATED = 8;
        const PLAYER_REPLICATED = 16;
        const SERVICE = 32;
        const SETTINGS = 64;
        const USER_SETTINGS = 128;
    }
}

#[derive(Debug)]
pub struct ClassTagsFromStrError(String);

impl FromStr for ClassTags {
    type Err = ClassTagsFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Deprecated" => Self::DEPRECATED,
            "NotBrowsable" => Self::NOT_BROWSABLE,
            "NotCreatable" => Self::NOT_CREATABLE,
            "NotReplicated" => Self::NOT_REPLICATED,
            "PlayerReplicated" => Self::PLAYER_REPLICATED,
            "Service" => Self::SERVICE,
            "Settings" => Self::SETTINGS,
            "UserSettings" => Self::USER_SETTINGS,
            _ => return Err(ClassTagsFromStrError(value.to_owned())),
        })
    }
}

// Tags found via:
// jq '[.Classes | .[] | .Members | .[] | select(.MemberType == "Property") | .Tags // empty] | add | unique' api-dump.json
bitterflag! {
    PropertyTags + PropertyTagsIntoIter: u32 {
        const DEPRECATED = 1;
        const HIDDEN = 2;
        const NOT_BROWSABLE = 4;
        const NOT_REPLICATED = 8;
        const NOT_SCRIPTABLE = 16;
        const READ_ONLY = 32;
    }
}

#[derive(Debug)]
pub struct PropertyTagsFromStrError(String);

impl FromStr for PropertyTags {
    type Err = PropertyTagsFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Deprecated" => Self::DEPRECATED,
            "Hidden" => Self::HIDDEN,
            "NotBrowsable" => Self::NOT_BROWSABLE,
            "NotReplicated" => Self::NOT_REPLICATED,
            "NotScriptable" => Self::NOT_SCRIPTABLE,
            "ReadOnly" => Self::READ_ONLY,
            _ => return Err(PropertyTagsFromStrError(value.to_owned())),
        })
    }
}
