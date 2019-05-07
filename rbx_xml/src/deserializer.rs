use std::{
    io::Read,
    collections::HashMap,
};

use log::trace;
use rbx_reflection::RbxPropertyTypeDescriptor;
use rbx_dom_weak::{RbxTree, RbxId, RbxInstanceProperties, RbxValue, RbxValueType, RbxValueConversion};

use crate::{
    core::find_canonical_property_descriptor,
    types::{read_value_xml, read_ref},
    error::{DecodeError as NewDecodeError, DecodeErrorKind},
};

// TODO: remove
pub use crate::deserializer_core::*;

/// A utility method to decode an XML-format model from a string.
pub fn decode_str(tree: &mut RbxTree, parent_id: RbxId, source: &str) -> Result<(), NewDecodeError> {
    decode(tree, parent_id, source.as_bytes())
}

/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(tree: &mut RbxTree, parent_id: RbxId, source: R) -> Result<(), NewDecodeError> {
    let mut iterator = XmlEventReader::from_source(source);
    let mut state = ParseState::new(tree);

    deserialize_root(&mut iterator, &mut state, parent_id)?;
    apply_id_rewrites(&mut state);

    Ok(())
}

struct IdPropertyRewrite {
    pub id: RbxId,
    pub property_name: String,
    pub referent_value: String,
}

/// The state needed to deserialize an XML model into an `RbxTree`.
pub struct ParseState<'a> {
    referents: HashMap<String, RbxId>,
    metadata: HashMap<String, String>,
    rewrite_ids: Vec<IdPropertyRewrite>,
    tree: &'a mut RbxTree,
}

impl<'a> ParseState<'a> {
    fn new(tree: &mut RbxTree) -> ParseState {
        ParseState {
            referents: HashMap::new(),
            metadata: HashMap::new(),
            rewrite_ids: Vec::new(),
            tree,
        }
    }

    /// Marks that a property on this instance needs to be rewritten once we
    /// have a complete view of how referents map to RbxId values.
    ///
    /// This is used to deserialize non-null Ref values correctly.
    pub fn add_id_rewrite(&mut self, id: RbxId, property_name: String, referent_value: String) {
        self.rewrite_ids.push(IdPropertyRewrite {
            id,
            property_name,
            referent_value,
        });
    }
}

fn apply_id_rewrites(state: &mut ParseState) {
    for rewrite in &state.rewrite_ids {
        let new_value = match state.referents.get(&rewrite.referent_value) {
            Some(id) => *id,
            None => continue
        };

        let instance = state.tree.get_instance_mut(rewrite.id)
            .expect("rbx_xml bug: had ID in referent map that didn't end up in the tree");

        instance.properties.insert(rewrite.property_name.clone(), RbxValue::Ref {
            value: Some(new_value),
        });
    }
}

fn deserialize_root<R: Read>(
    reader: &mut XmlEventReader<R>,
    state: &mut ParseState,
    parent_id: RbxId
) -> Result<(), DecodeError> {
    match reader.next().ok_or(DecodeError::MalformedDocument)?? {
        XmlReadEvent::StartDocument { .. } => {},
        _ => return Err(DecodeError::MalformedDocument),
    }

    read_event!(reader, XmlReadEvent::StartElement { name, attributes, .. } => {
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
    });

    loop {
        match reader.peek().ok_or(DecodeError::MalformedDocument)? {
            Ok(XmlReadEvent::StartElement { name, .. }) => {
                match name.local_name.as_str() {
                    "Item" => {
                        deserialize_instance(reader, state, parent_id)?;
                    },
                    "External" => {
                        // This tag is always meaningless, there's nothing to do
                        // here except skip it.
                        reader.eat_unknown_tag()?;
                    },
                    "Meta" => {
                        deserialize_metadata(reader, state)?;
                    },
                    _ => return Err(DecodeError::Message("Unexpected top-level start tag")),
                }
            },
            Ok(XmlReadEvent::EndElement { name, .. }) => {
                if name.local_name == "roblox" {
                    break;
                } else {
                    return Err(DecodeError::Message("Unexpected closing tag"));
                }
            },
            Ok(XmlReadEvent::EndDocument) => break,
            Ok(_) => return Err(DecodeError::Message("Unexpected top-level stuff")),
            Err(_) => {
                reader.next().unwrap()?;
            },
        }
    }

    Ok(())
}

fn deserialize_metadata<R: Read>(reader: &mut XmlEventReader<R>, state: &mut ParseState) -> Result<(), NewDecodeError> {
    let name = {
        let attributes = reader.expect_start_with_name("Meta")?;

        let mut name = None;

        for attribute in attributes.into_iter() {
            match attribute.name.local_name.as_str() {
                "name" => name = Some(attribute.value),
                _ => {}
            }
        }

        name.ok_or_else(|| reader.error(DecodeErrorKind::MissingAttribute("name")))?
    };

    let value = reader.read_characters()?;
    reader.expect_end_with_name("Meta")?;

    state.metadata.insert(name, value);
    Ok(())
}

fn deserialize_instance<R: Read>(
    reader: &mut XmlEventReader<R>,
    state: &mut ParseState,
    parent_id: RbxId,
) -> Result<(), DecodeError> {
    let (class_name, referent) = read_event!(reader, XmlReadEvent::StartElement { name, mut attributes, .. } => {
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
    });

    trace!("Class {} with referent {:?}", class_name, referent);

    let instance_props = RbxInstanceProperties {
        class_name,
        name: String::new(),
        properties: HashMap::new(),
    };

    let instance_id = state.tree.insert_instance(instance_props, parent_id);

    if let Some(referent) = referent {
        state.referents.insert(referent, instance_id);
    }

    // we have to collect properties in order to create the instance
    // name will be captured in this map and extracted later; XML doesn't store it separately
    let mut properties: HashMap<String, RbxValue> = HashMap::new();

    loop {
        match reader.peek().ok_or(DecodeError::Message("Unexpected EOF"))? {
            Ok(XmlReadEvent::StartElement { name, .. }) => match name.local_name.as_str() {
                "Properties" => {
                    deserialize_properties(reader, state, instance_id, &mut properties)?;
                },
                "Item" => {
                    deserialize_instance(reader, state, instance_id)?;
                }
                _ => return Err(DecodeError::Message("Unexpected tag inside instance")),
            },
            Ok(XmlReadEvent::EndElement { name }) => {
                if name.local_name != "Item" {
                    return Err(DecodeError::Message("Unexpected closing tag, expected Item"));
                }

                reader.next();
                break;
            },
            unexpected => panic!("Unexpected XmlReadEvent {:?}", unexpected),
        }
    }

    let instance = state.tree.get_instance_mut(instance_id).unwrap();

    instance.name = match properties.remove("Name") {
        Some(value) => match value {
            RbxValue::String { value } => value,
            _ => return Err(DecodeError::Message("Name must be a string")),
        },
        None => instance.class_name.clone(),
    };

    instance.properties = properties;

    Ok(())
}

fn deserialize_properties<R: Read>(
    reader: &mut XmlEventReader<R>,
    state: &mut ParseState,
    instance_id: RbxId,
    props: &mut HashMap<String, RbxValue>,
) -> Result<(), DecodeError> {
    reader.expect_start_with_name("Properties")?;

    let class_name = state.tree.get_instance(instance_id)
        .expect("Couldn't find instance to deserialize properties into")
        .class_name.clone();

    'property_loop: loop {
        let (property_type, xml_property_name) = loop {
            match reader.expect_peek()? {
                XmlReadEvent::StartElement { name, attributes, .. } => {
                    let mut xml_property_name = None;

                    for attribute in attributes {
                        if attribute.name.local_name.as_str() == "name" {
                            xml_property_name = Some(attribute.value.to_owned());
                            break;
                        }
                    }

                    let xml_property_name = match xml_property_name {
                        Some(value) => value,
                        None => return Err(reader.error(DecodeErrorKind::MissingAttribute("name")).into())
                    };

                    break (name.local_name.to_owned(), xml_property_name)
                },
                XmlReadEvent::EndElement { name } => {
                    if name.local_name == "Properties" {
                        reader.expect_next()?;
                        return Ok(())
                    } else {
                        let err = DecodeErrorKind::UnexpectedXmlEvent(reader.expect_next()?);
                        return Err(reader.error(err).into());
                    }
                },
                _ => {
                    let err = DecodeErrorKind::UnexpectedXmlEvent(reader.expect_next()?);
                    return Err(reader.error(err).into());
                }
            };
        };

        if let Some(descriptor) = find_canonical_property_descriptor(&class_name, &xml_property_name) {
            let value = match property_type.as_str() {
                "Ref" => {
                    // Refs need lots of additional state that we don't want to pass to
                    // other property types unnecessarily, so we special-case it here.

                    read_ref(reader, instance_id, descriptor.name(), state)?
                }
                _ => {
                    let xml_value = read_value_xml(reader, &property_type)?;

                    let value_type = match descriptor.property_type() {
                        RbxPropertyTypeDescriptor::Data(value_type) => *value_type,
                        RbxPropertyTypeDescriptor::Enum(_enum_name) => RbxValueType::Enum,
                        RbxPropertyTypeDescriptor::UnimplementedType(_) => xml_value.get_type(),
                    };

                    let value = match xml_value.try_convert_ref(value_type) {
                        RbxValueConversion::Converted(value) => value,
                        RbxValueConversion::Unnecessary | RbxValueConversion::Failed => xml_value,
                    };

                    value
                }
            };

            props.insert(descriptor.name().to_string(), value);
        } else {
            // We don't care about this property, read it into the void.
            read_value_xml(reader, &property_type)?;
        }
    }
}