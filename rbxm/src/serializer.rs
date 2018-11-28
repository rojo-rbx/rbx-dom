use std::io::Write;

use rbx_tree::{RbxTree, RbxId};

// Format information taken from:
// http://www.classy-studios.com/Downloads/RobloxFileSpec.pdf

const FILE_HEADER = b"<roblox!\x89\xff\x0d\x0a\x1a\x0a\0\0";

/// Serialize the instances denoted by `ids` from `tree` to XML.
pub fn encode<W: Write>(tree: &RbxTree, ids: &[RbxId], output: W) {
    unimplemented!()
}