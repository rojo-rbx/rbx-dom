use std::collections::HashMap;

pub struct RbxInstanceClass {
    pub name: &'static str,
    pub properties: HashMap<&'static str, RbxInstanceProperty>,
}

pub struct RbxInstanceProperty {
    pub name: &'static str,
    pub value_type: &'static str,
}
