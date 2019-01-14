use std::{
    io::Read,
    iter::Peekable,
};

use log::trace;
use rbx_tree::{RbxTree, RbxId};

use xml::{
    ParserConfig,
    EventReader,
    reader::{self, XmlEvent},
};

/// Indicates an error trying to parse an rbxmx or rbxlx document
#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    XmlError(reader::Error),
    Message(&'static str),
    MalformedDocument,
}

impl From<reader::Error> for DecodeError {
    fn from(error: reader::Error) -> DecodeError {
        DecodeError::XmlError(error)
    }
}

struct EventIterator<R: Read> {
    inner: Peekable<reader::Events<R>>,
}

impl<R: Read> EventIterator<R> {
    fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.inner.peek()
    }

    fn from_reader(reader: EventReader<R>) -> EventIterator<R> {
        EventIterator {
            inner: reader.into_iter().peekable(),
        }
    }
}

impl<R: Read> Iterator for EventIterator<R> {
    type Item = reader::Result<XmlEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// INCOMPLETE: This function does not finish constructing instances.
///
/// A utility method to decode an XML-format model from a string.
pub fn decode_str(tree: &mut RbxTree, parent_id: RbxId, source: &str) -> Result<(), DecodeError> {
    decode(tree, parent_id, source.as_bytes())
}

/// INCOMPLETE: This function does not finish constructing instances.
///
/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(tree: &mut RbxTree, parent_id: RbxId, source: R) -> Result<(), DecodeError> {
    let reader = ParserConfig::new()
        .create_reader(source);

    let mut iterator = EventIterator::from_reader(reader);

    deserialize_root(tree, parent_id, &mut iterator)
}

fn deserialize_root<R: Read>(tree: &mut RbxTree, parent_id: RbxId, reader: &mut EventIterator<R>) -> Result<(), DecodeError> {
    match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlEvent::StartDocument { .. } => {},
        _ => return Err(DecodeError::MalformedDocument),
    }

    match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlEvent::StartElement { name, attributes, .. } => {
            if name.local_name != "roblox" {
                return Err(DecodeError::Message("Missing <roblox>"));
            }

            let mut found_version = false;
            for attribute in &attributes {
                if attribute.name.local_name == "version" {
                    found_version = true;

                    if attribute.value != "4" {
                        return Err(DecodeError::Message("Not version 4"));
                    }
                }
            }

            if !found_version {
                return Err(DecodeError::Message("No version field"));
            }
        },
        _ => return Err(DecodeError::Message("Unexpected stuff before <roblox>")),
    }

    loop {
        match reader.peek().ok_or(DecodeError::MalformedDocument)? {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "Item" {
                    deserialize_instance(tree, parent_id, reader)?;
                } else {
                    eat_unknown_tag(reader)?;
                }
            },
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name == "roblox" {
                    break;
                } else {
                    return Err(DecodeError::Message("Unexpected closing tag"));
                }
            },
            Ok(XmlEvent::EndDocument) => break,
            Ok(_) => {
                let _ = reader.next();
            },
            Err(_) => {
                match reader.next().unwrap() {
                    Err(e) => return Err(e.into()),
                    Ok(_) => unreachable!(),
                }
            },
        }
    }

    Ok(())
}

/// Consume events from the iterator until we reach the end of the next tag.
fn eat_unknown_tag<R: Read>(reader: &mut EventIterator<R>) -> Result<(), DecodeError> {
    let mut depth = 0;

    loop {
        match reader.next().ok_or(DecodeError::MalformedDocument)?? {
            XmlEvent::StartElement { .. } => {
                depth += 1;
            },
            XmlEvent::EndElement { .. } => {
                depth -= 1;

                if depth == 0 {
                    break;
                }
            },
            _ => {},
        }
    }

    Ok(())
}

fn deserialize_instance<R: Read>(tree: &mut RbxTree, parent_id: RbxId, reader: &mut EventIterator<R>) -> Result<(), DecodeError> {
    let (class, referent) = match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlEvent::StartElement { name, mut attributes, .. } => {
            assert_eq!(name.local_name, "Item");

            let mut class = None;
            let mut referent = None;

            for attribute in attributes.drain(..) {
                match attribute.name.local_name.as_str() {
                    "class" => class = Some(attribute.value),
                    "referent" => referent = Some(attribute.value),
                    _ => {},
                }
            }

            let class = class.ok_or(DecodeError::Message("Missing 'class'"))?;

            (class, referent)
        },
        _ => unreachable!(),
    };

    // TODO: Collect properties
    // TODO: Construct instance and insert it into the tree
    // TODO: Collect children

    let mut depth = 1;

    loop {
        match reader.next().ok_or(DecodeError::MalformedDocument)?? {
            XmlEvent::StartElement { .. } => {
                depth += 1;
            },
            XmlEvent::EndElement { name, .. } => {
                depth -= 1;

                if depth == 0 {
                    break;
                }
            },
            _ => {},
        }
    }

    trace!("Class {} with referent {:?}", class, referent);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use rbx_tree::RbxInstanceProperties;

    use super::*;

    fn new_data_model() -> RbxTree {
        let root = RbxInstanceProperties {
            name: "DataModel".to_string(),
            class_name: "DataModel".to_string(),
            properties: HashMap::new(),
        };

        RbxTree::new(root)
    }

    #[test]
    fn empty_document() {
        let _ = env_logger::try_init();
        let document = r#"<roblox version="4"></roblox>"#;
        let mut tree = new_data_model();
        let root_id = tree.get_root_id();

        decode_str(&mut tree, root_id, document).unwrap();
    }

    #[test]
    fn just_garbage() {
        let _ = env_logger::try_init();
        let document = r#"
            <roblox version="4">
                <!-- hello there! -->
                <meta name="trash" />
                <foo></foo>
            </roblox>
        "#;

        let mut tree = new_data_model();
        let root_id = tree.get_root_id();

        decode_str(&mut tree, root_id, document).unwrap();
    }

    #[test]
    fn empty_instance() {
        let _ = env_logger::try_init();
        let document = r#"
            <roblox version="4">
                <Item class="Folder" referent="hello">
                </Item>
            </roblox>
        "#;

        let mut tree = new_data_model();
        let root_id = tree.get_root_id();

        decode_str(&mut tree, root_id, document).unwrap();

        // TODO: Check that an instance got made
    }
}