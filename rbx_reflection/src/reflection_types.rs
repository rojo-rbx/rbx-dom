// This file is also pulled in by generate_reflection in order to make sure that
// types don't get out of sync.

use std::{borrow::Cow, collections::HashMap};

use bitflags::bitflags;
use rbx_dom_weak::{RbxValue, RbxValueType};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct RbxClassDescriptor {
    pub(crate) name: Cow<'static, str>,
    pub(crate) superclass: Option<Cow<'static, str>>,
    pub(crate) tags: RbxInstanceTags,
    pub(crate) properties: HashMap<Cow<'static, str>, RbxPropertyDescriptor>,
    pub(crate) default_properties: HashMap<Cow<'static, str>, RbxValue>,
}

impl RbxClassDescriptor {
    pub fn name(&self) -> & str {
        &self.name
    }

    pub fn superclass(&self) -> Option<&str> {
        self.superclass.as_ref().map(|v| v.as_ref())
    }

    pub fn get_property_descriptor<'a>(&'a self, property_name: &str) -> Option<&'a RbxPropertyDescriptor> {
        self.properties.get(property_name)
    }

    pub fn get_default_value<'a>(&'a self, property_name: &str) -> Option<&'a RbxValue> {
        self.default_properties.get(property_name)
    }
}

#[derive(Debug, PartialEq)]
pub struct RbxPropertyDescriptor {
    pub(crate) name: Cow<'static, str>,
    pub(crate) value_type: RbxPropertyTypeDescriptor,
    pub(crate) tags: RbxPropertyTags,

    pub(crate) scriptability: RbxPropertyScriptability,
    pub(crate) is_canonical: bool,
    pub(crate) canonical_name: Option<Cow<'static, str>>,
    pub(crate) serialized_name: Option<Cow<'static, str>>,
    pub(crate) serializes: bool,
}

impl RbxPropertyDescriptor {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn property_type(&self) -> &RbxPropertyTypeDescriptor {
        &self.value_type
    }

    pub fn scriptability(&self) -> RbxPropertyScriptability {
        self.scriptability
    }

    pub fn is_canonical(&self) -> bool {
        self.is_canonical
    }

    pub fn canonical_name(&self) -> Option<&str> {
        self.canonical_name.as_ref().map(|v| v.as_ref())
    }

    pub fn serialized_name(&self) -> Option<&str> {
        self.serialized_name.as_ref().map(|v| v.as_ref())
    }

    pub fn serializes(&self) -> bool {
        self.serializes
    }
}

#[derive(Debug, PartialEq)]
pub struct RbxEnumDescriptor {
    pub(crate) name: Cow<'static, str>,
    pub(crate) items: HashMap<Cow<'static, str>, u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RbxPropertyTypeDescriptor {
    Data(RbxValueType),
    Enum(Cow<'static, str>),

    UnimplementedType(Cow<'static, str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RbxPropertyScriptability {
    None,
    ReadWrite,
    Read,
    Write,
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
        bitflags! {
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
    RbxInstanceTags + RbxInstanceTagsIntoIter: u32 {
        const DEPRECATED = 0x1;
        const NOT_BROWSABLE = 0x2;
        const NOT_CREATABLE = 0x4;
        const NOT_REPLICATED = 0x8;
        const PLAYER_REPLICATED = 0x10;
        const SERVICE = 0x20;
        const SETTINGS = 0x40;
    }
}

// Tags found via:
// jq '[.Classes | .[] | .Members | .[] | select(.MemberType == "Property") | .Tags // empty] | add | unique' api-dump.json
bitterflag! {
    RbxPropertyTags + RbxPropertyTagsIntoIter: u32 {
        const DEPRECATED = 0x1;
        const HIDDEN = 0x2;
        const NOT_BROWSABLE = 0x4;
        const NOT_REPLICATED = 0x8;
        const NOT_SCRIPTABLE = 0x10;
        const READ_ONLY = 0x20;
    }
}
