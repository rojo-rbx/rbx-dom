use std::io::Read;

use rbx_tree::{RbxTree, RbxId};

use xml::{ParserConfig, EventReader, reader::{self, XmlEvent}};

/// Indicates an error trying to parse an rbxmx or rbxlx document
pub enum RbxmxParseError {
    XmlError(reader::Error),
    MalformedDocument,
}

impl From<reader::Error> for RbxmxParseError {
    fn from(error: reader::Error) -> RbxmxParseError {
        RbxmxParseError::XmlError(error)
    }
}

/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(tree: &mut RbxTree, parent_id: RbxId, source: R) -> Result<(), RbxmxParseError> {
    let mut reader = ParserConfig::new()
        .create_reader(source);

    deserialize_root(tree, parent_id, &mut reader)
}

#[allow(unused)]
fn deserialize_root<R: Read>(tree: &mut RbxTree, parent_id: RbxId, reader: &mut EventReader<R>) -> Result<(), RbxmxParseError> {
    match reader.next()? {
        XmlEvent::StartDocument { .. } => {},
        _ => return Err(RbxmxParseError::MalformedDocument),
    }

    match reader.next()? {
        XmlEvent::StartElement { name, attributes, .. } => {
            if name.local_name != "roblox" {
                return Err(RbxmxParseError::MalformedDocument);
            }

            let mut found_version = false;
            for attribute in &attributes {
                if attribute.name.local_name == "version" {
                    found_version = true;

                    if attribute.value != "4" {
                        return Err(RbxmxParseError::MalformedDocument);
                    }
                }
            }

            if !found_version {
                return Err(RbxmxParseError::MalformedDocument);
            }
        },
        _ => return Err(RbxmxParseError::MalformedDocument),
    }

    // TODO: A bunch of stuff

    Ok(())
}