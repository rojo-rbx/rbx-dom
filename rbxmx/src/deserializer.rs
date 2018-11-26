use std::io::Read;

use rbx_tree::{RbxTree, RbxId};

use xml::ParserConfig;

/// Indicates an error trying to parse an rbxmx or rbxlx document
pub enum RbxmxParseError {
}

/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(_tree: &mut RbxTree, _parent_id: RbxId, source: R) -> Result<(), RbxmxParseError> {
    let _reader = ParserConfig::new()
        .create_reader(source);

    unimplemented!()
}