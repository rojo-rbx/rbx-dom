// This file is also pulled in by generate_reflection in order to make sure that
// types don't get out of sync.

use std::{borrow::Cow, collections::HashMap};

use bitflags::bitflags;
use rbx_dom_weak::{RbxValue, RbxValueType};

#[derive(Debug, PartialEq)]
pub struct RbxInstanceClass {
    pub name: Cow<'static, str>,
    pub superclass: Option<Cow<'static, str>>,
    pub tags: RbxInstanceTags,
    pub properties: HashMap<Cow<'static, str>, RbxInstanceProperty>,
    pub default_properties: HashMap<Cow<'static, str>, RbxValue>,
}

#[derive(Debug, PartialEq)]
pub struct RbxInstanceProperty {
    pub name: Cow<'static, str>,
    pub value_type: RbxPropertyType,
    pub tags: RbxPropertyTags,
}

#[derive(Debug, PartialEq)]
pub struct RbxEnum {
    pub name: Cow<'static, str>,
    pub items: HashMap<Cow<'static, str>, u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RbxPropertyType {
    Data(RbxValueType),
    Enum(Cow<'static, str>),

    UnimplementedType(Cow<'static, str>),
}

bitflags! {
    // Tags found via:
    // jq '[.Classes | .[] | .Tags // empty] | add | unique' api-dump.json
    pub struct RbxInstanceTags: u8 {
        const DEPRECATED        = 0b00000001;
        const NOT_BROWSABLE     = 0b00000010;
        const NOT_CREATABLE     = 0b00000100;
        const NOT_REPLICATED    = 0b00001000;
        const PLAYER_REPLICATED = 0b00010000;
        const SERVICE           = 0b00100000;
        const SETTINGS          = 0b01000000;
    }
}

bitflags! {
    // Tags found via:
    // jq '[.Classes | .[] | .Members | .[] | select(.MemberType == "Property") | .Tags // empty] | add | unique' api-dump.json
    pub struct RbxPropertyTags: u8 {
        const DEPRECATED     = 0b00000001;
        const HIDDEN         = 0b00000010;
        const NOT_BROWSABLE  = 0b00000100;
        const NOT_REPLICATED = 0b00001000;
        const NOT_SCRIPTABLE = 0b00010000;
        const READ_ONLY      = 0b00100000;
    }
}
