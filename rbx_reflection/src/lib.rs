use std::{borrow::Cow, collections::HashMap, str::FromStr};

use rbx_types::{Variant, VariantType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[non_exhaustive]
pub struct ClassDescriptor<'a> {
    pub name: Cow<'a, str>,
    pub superclass: Option<Cow<'a, str>>,
    pub properties: HashMap<Cow<'a, str>, PropertyDescriptor<'a>>,
    pub default_properties: HashMap<Cow<'a, str>, Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[non_exhaustive]
pub struct PropertyDescriptor<'a> {
    pub name: Cow<'a, str>,
    pub scriptability: Scriptability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PropertyType<'a> {
    /// The property is a regular value of the given type.
    Data(VariantType),

    /// The property is an enum with the given name.
    Enum(Cow<'a, str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// The bitflags crate doesn't support iterating over the bits that are set in
/// the flag. In order to generate lists of flag names, we create a macro that
/// abstracts over the bitflags macro and additionally implements IntoIterator
/// on the type.
///
/// To avoid pulling in a dependency on either the `paste!` or `concat_idents!`
/// macros, the caller has to pass inthe name of the iterator type to define.
macro_rules! bitterflag {
    ($struct_name: ident + $iter_name: ident : $width: ident { $(const $const_name: ident = $const_value: expr;)* }) => {
        bitflags::bitflags! {
            pub struct $struct_name: $width {
                $(const $const_name = $const_value;)*
            }
        }

        pub struct $iter_name {
            inner: Box<dyn Iterator<Item = $struct_name>>,
        }

        impl Iterator for $iter_name {
            type Item = $struct_name;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl IntoIterator for $struct_name {
            type Item = Self;
            type IntoIter = $iter_name;

            fn into_iter(self) -> Self::IntoIter {
                static ALL_TAGS: &[$struct_name] = &[
                    $($struct_name::$const_name,)*
                ];

                $iter_name {
                    inner: Box::new(
                        ALL_TAGS
                            .iter()
                            .cloned()
                            .filter(move |flag| self.contains(*flag)),
                    ),
                }
            }
        }
    };
}

// Tags found via:
// jq '[.Classes | .[] | .Tags // empty] | add | unique' api-dump.json
bitterflag! {
    InstanceTags + InstanceTagsIntoIter: u32 {
        const DEPRECATED = 0x1;
        const NOT_BROWSABLE = 0x2;
        const NOT_CREATABLE = 0x4;
        const NOT_REPLICATED = 0x8;
        const PLAYER_REPLICATED = 0x10;
        const SERVICE = 0x20;
        const SETTINGS = 0x40;
    }
}

#[derive(Debug)]
pub struct InstanceTagsFromStrError;

impl FromStr for InstanceTags {
    type Err = InstanceTagsFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "Deprecated" => Self::DEPRECATED,
            "NotBrowsable" => Self::NOT_BROWSABLE,
            "NotCreatable" => Self::NOT_CREATABLE,
            "NotReplicated" => Self::NOT_REPLICATED,
            "PlayerReplicated" => Self::PLAYER_REPLICATED,
            "Service" => Self::SERVICE,
            "Settings" => Self::SETTINGS,
            _ => return Err(InstanceTagsFromStrError),
        })
    }
}

// Tags found via:
// jq '[.Classes | .[] | .Members | .[] | select(.MemberType == "Property") | .Tags // empty] | add | unique' api-dump.json
bitterflag! {
    PropertyTags + PropertyTagsIntoIter: u32 {
        const DEPRECATED = 0x1;
        const HIDDEN = 0x2;
        const NOT_BROWSABLE = 0x4;
        const NOT_REPLICATED = 0x8;
        const NOT_SCRIPTABLE = 0x10;
        const READ_ONLY = 0x20;
    }
}

#[derive(Debug)]
pub struct PropertyTagsFromStrError;

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
            _ => return Err(PropertyTagsFromStrError),
        })
    }
}
