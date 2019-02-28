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
        const Deprecated       = 0b00000001;
        const NotBrowsable     = 0b00000010;
        const NotCreatable     = 0b00000100;
        const NotReplicated    = 0b00001000;
        const PlayerReplicated = 0b00010000;
        const Service          = 0b00100000;
        const Settings         = 0b01000000;
    }
}
