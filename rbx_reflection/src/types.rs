use std::collections::HashMap;

use rbx_tree::RbxValueType;

#[derive(Debug)]
pub struct RbxInstanceClass {
    pub name: &'static str,
    pub superclass: Option<&'static str>,
    pub properties: HashMap<&'static str, RbxInstanceProperty>,
}

#[derive(Debug)]
pub struct RbxInstanceProperty {
    pub name: &'static str,
    pub value_type: RbxPropertyType,
}

#[derive(Debug)]
pub struct RbxEnum {
    pub name: &'static str,
    pub items: HashMap<&'static str, u32>,
}

#[derive(Debug, Clone, Copy)]
pub enum RbxPropertyType {
    Data(RbxValueType),
    Enum(&'static str),
    InstanceRef(&'static str),

    UnimplementedType(&'static str),
}
