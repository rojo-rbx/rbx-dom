use std::collections::HashMap;

use rbx_dom_weak::{RbxValue, RbxValueType};

use crate::{
    api_dump::Dump,
    canonical_properties::CanonicalPropertyDatabase,
};

pub enum ReflectionPropertyType {
    Data(RbxValueType),
    Enum(&'static str),
    UnimplementedType(&'static str),
}

bitflags! {
    // Tags found via:
    // jq '[.Classes | .[] | .Tags // empty] | add | unique' api-dump.json
    pub struct ReflectionClassTags: u8 {
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
    pub struct ReflectionPropertyTags: u8 {
        const DEPRECATED     = 0b00000001;
        const HIDDEN         = 0b00000010;
        const NOT_BROWSABLE  = 0b00000100;
        const NOT_REPLICATED = 0b00001000;
        const NOT_SCRIPTABLE = 0b00010000;
        const READ_ONLY      = 0b00100000;

        const CANONICAL      = 0b01000000;
    }
}

pub struct ReflectionClass {
    pub name: String,
    pub superclass: Option<String>,
    pub tags: ReflectionClassTags,
    pub properties: HashMap<String, ReflectionProperty>,
}

pub struct ReflectionProperty {
    pub name: String,
    pub value_type: ReflectionPropertyType,
    pub tags: ReflectionPropertyTags,

    pub canonical_name: Option<String>,
}

pub struct ReflectionDatabase {
    pub dump: Dump,
    pub default_properties: HashMap<String, HashMap<String, RbxValue>>,
    pub canonical_properties: &'static CanonicalPropertyDatabase,
    pub studio_version: [u32; 4],
}