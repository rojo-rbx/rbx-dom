use rbx_tree::RbxTree;

/// Indicates an error trying to parse an rbxmx or rbxlx document
pub enum RbxmxParseError {
}

pub fn decode(_tree: &mut RbxTree, _source: &[u8]) -> Result<(), RbxmxParseError> {
    unimplemented!()
}