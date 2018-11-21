extern crate rbx_tree;
extern crate quick_xml;

use rbx_tree::{RbxTree, RbxValue};

fn serialize_value(value: &RbxValue) {
    unimplemented!()
}

fn deserialize_value() -> RbxValue {
    unimplemented!()
}

pub enum RbxmxParseError {
}

pub fn from_str(source: &str) -> Result<RbxTree, RbxmxParseError> {
    unimplemented!()
}

pub fn to_string(tree: &RbxTree) -> String {
    unimplemented!()
}