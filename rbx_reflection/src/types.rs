use std::collections::HashMap;

use rbx_dom_weak::RbxValueType;

#[derive(Debug, PartialEq)]
pub struct RbxInstanceClass {
    pub name: &'static str,
    pub superclass: Option<&'static str>,
    pub properties: HashMap<&'static str, RbxInstanceProperty>,
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
