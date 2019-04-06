use std::collections::HashMap;

use bitflags::bitflags;
use rbx_dom_weak::{RbxValue, RbxValueType};

#[derive(Debug, PartialEq)]
pub struct RbxInstanceClass {
    pub name: &'static str,
    pub superclass: Option<&'static str>,
    pub tags: RbxInstanceTags,
    pub properties: HashMap<&'static str, RbxInstanceProperty>,
    pub default_properties: HashMap<&'static str, RbxValue>,
}

#[derive(Debug, PartialEq)]
pub struct RbxInstanceProperty {
    pub name: &'static str,
    pub value_type: RbxPropertyType,
    pub tags: RbxPropertyTags,
    pub canonical_name: Option<String>,
    pub serialized_name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct RbxEnum {
    pub name: &'static str,
    pub items: HashMap<&'static str, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RbxPropertyType {
    Data(RbxValueType),
    Enum(&'static str),
    InstanceRef(&'static str),

    UnimplementedType(&'static str),
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
