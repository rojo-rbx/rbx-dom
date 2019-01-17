use std::{
    io::Read,
    iter::Peekable,
    collections::HashMap,
};

use log::trace;
use rbx_tree::{RbxTree, RbxId, RbxInstanceProperties, RbxValue};

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

struct ParseState<'a> {
    environment: ParseEnvironment<'a>,
    referents: HashMap<String, RbxId>,
    tree: &'a mut RbxTree,
}

enum ParseEnvironment<'a> {
    Root {
        root_parent: RbxId,
    },
    Instance {
        parent: RbxId,
    },
    Properties {
        props: &'a mut HashMap<String, RbxValue>,
    },
    // Individual property tags
    UDim2,
    Rect2D,
    Rect2DMin,
    Rect2DMax,
    Vector2,
    Vector3,
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
        .trim_whitespace(true)
        .create_reader(source);

    let mut iterator = EventIterator::from_reader(reader);
    let mut parse_state = ParseState {
        environment: ParseEnvironment::Root {
            root_parent: parent_id,
        },
        referents: HashMap::new(),
        tree,
    };

    deserialize_next(&mut iterator, &mut parse_state)
}

fn deserialize_next<R: Read>(reader: &mut EventIterator<R>, state: &mut ParseState) -> Result<(), DecodeError> {
    match &state.environment {
        ParseEnvironment::Root { root_parent, .. } => {
            deserialize_root(reader, state, *root_parent)
        },
        ParseEnvironment::Instance { parent, .. } => {
            deserialize_instance(reader, state, *parent)
        },
        _ => unimplemented!(),
    }
}

fn deserialize_root<R: Read>(reader: &mut EventIterator<R>, state: &mut ParseState, parent_id: RbxId) -> Result<(), DecodeError> {
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
                    state.environment = ParseEnvironment::Instance {
                        parent: parent_id,
                    };

                    deserialize_next(reader, state)?;
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

fn deserialize_instance<R: Read>(reader: &mut EventIterator<R>, state: &mut ParseState, parent_id: RbxId) -> Result<(), DecodeError> {
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

    // TODO: Collect children

    // we have to collect properties in order to create the instance
    // name will be captured in this map and extracted later; XML doesn't store it separately
    let mut property_map: HashMap<String, RbxValue> = HashMap::new();

    let mut depth = 1;

    loop {
        match reader.next().ok_or(DecodeError::MalformedDocument)?? {
            XmlEvent::StartElement { name, .. } => {
                depth += 1;

                match name.local_name.as_str() {
                    "Properties" => {
                        println!("start prop parsing for inst class {}, referent {:?}", class, referent);
                        deserialize_properties(reader, &mut property_map)?;
                    },
                    _ => unimplemented!(),
                }
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
    let instance_name = match property_map.remove("Name") {
        Some(value) => match value {
            RbxValue::String { value } => value,
            _ => return Err(DecodeError::Message("Name must be a string")),
        },
        // TODO: is this actually an invariant of the XML format?
        _ => return Err(DecodeError::Message("All instances must be named")),
    };

    let instance_props = RbxInstanceProperties {
        class_name: class,
        name: instance_name,
        properties: property_map,
    };

    let instance_id = state.tree.insert_instance(instance_props, parent_id);

    if referent.is_some() {
        state.referents.insert(referent.unwrap(), instance_id);
    }

    Ok(())
}

fn deserialize_properties<R: Read>(reader: &mut EventIterator<R>, props: &mut HashMap<String, RbxValue>) -> Result<(), DecodeError> {
    loop {
        let (property_type, property_name) = match reader.next().ok_or(DecodeError::MalformedDocument)?? {
            XmlEvent::StartElement { name, mut attributes, .. } => {
                let mut property_name = None;

                for attribute in attributes.drain(..) {
                    match attribute.name.local_name.as_str() {
                        "name" => property_name = Some(attribute.value),
                        _ => {},
                    }
                }

                let property_name = property_name.ok_or(DecodeError::Message("Missing 'name' for property tag"))?;
                (name.local_name, property_name)
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "Properties" {
                    return Ok(())
                }
                else {
                    unreachable!()
                }
            },
            XmlEvent::Characters(chars) => panic!("Characters {:?}", chars),
            XmlEvent::CData(data) => panic!("cdata {:?}", data),
            XmlEvent::Whitespace(_) => panic!("Whitespace"),
            XmlEvent::Comment(comment) => panic!("Comment {:?}", comment),
            _ => unreachable!(),
        };

        let value = match property_type.as_str() {
            "bool" => RbxValue::Bool {
                value: deserialize_bool(reader)?,
            },
            "string" => RbxValue::String {
                value:deserialize_string(reader)?,
            },
            _ => return Err(DecodeError::Message("don't know how to decode this prop type")),
        };

        props.insert(property_name, value);

        // continue until we find the matching close tag (after deserialization)
        loop {
            match reader.next().ok_or(DecodeError::MalformedDocument)?? {
                XmlEvent::EndElement { name } => {
                    if name.local_name == property_type {
                        break
                    }
                },
                _ => {},
            }
        }
    }
}

fn deserialize_bool<R: Read>(reader: &mut EventIterator<R>) -> Result<bool, DecodeError> {
    match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlEvent::Characters(content) => {
            match content.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(DecodeError::Message("invalid boolean value, expected true or false")),
            }
        },
        _ => Err(DecodeError::MalformedDocument),
    }
}

fn deserialize_string<R: Read>(reader: &mut EventIterator<R>) -> Result<String, DecodeError> {
    match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlEvent::Characters(content) => Ok(content),
        _ => Err(DecodeError::MalformedDocument),
    }
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

    #[test]
    fn with_bool() {
        let _ = env_logger::try_init();
        let document = r#"
            <roblox version="4">
                <Item class="BoolValue" referent="hello">
                    <Properties>
                        <string name="Name">Test</string>
                        <bool name="Value">true</bool>
                    </Properties>
                </Item>
            </roblox>
        "#;

        let mut tree = new_data_model();
        let root_id = tree.get_root_id();

        decode_str(&mut tree, root_id, document).expect("should work D:");

        let descendant = tree.descendants(root_id).nth(1).unwrap();
        assert_eq!(descendant.name, "Test");
        assert_eq!(descendant.class_name, "BoolValue");
        assert_eq!(descendant.properties.get("Value"), Some(&RbxValue::Bool { value: true }));

        // TODO: Check that an instance got made
    }
}