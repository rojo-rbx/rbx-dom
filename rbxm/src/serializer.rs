use std::io::Write;

use rbx_tree::{RbxTree, RbxId};

/// Serialize the instances denoted by `ids` from `tree` to XML.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) {
    unimplemented!()
}