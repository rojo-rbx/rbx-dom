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

use crate::deserializer_core::{XmlEventReader, XmlReadEvent};

/// Options available for deserializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct DecodeOptions {
    use_reflection: bool,
}

impl DecodeOptions {
    /// Constructs a `DecodeOptions` with all values set to their defaults.
    pub fn new() -> DecodeOptions {
        DecodeOptions {
            use_reflection: true,
        }
    }

    /// Enabled by default.
    ///
    /// Sets whether to use the reflection database to canonicalize fields and
    /// value types.
    ///
    /// If disabled, the deserialized tree will have exactly the properties and
    /// types present in the model/place file instead of ones modified for
    /// consumption. This leaks details of the format, but can be useful for
    /// debugging.
    // TODO: Make this public once this setting actually does anything.
    #[allow(unused)]
    fn use_reflection(self, use_reflection: bool) -> DecodeOptions {
        DecodeOptions {
            use_reflection,
            ..self
        }
    }
}

impl Default for DecodeOptions {
    fn default() -> DecodeOptions {
        DecodeOptions::new()
    }
}

pub fn decode_internal<R: Read>(source: R, _options: DecodeOptions) -> Result<RbxTree, NewDecodeError> {
    let mut tree = RbxTree::new(RbxInstanceProperties {
        class_name: "DataModel".to_owned(),
        name: "DataModel".to_owned(),
        properties: HashMap::new(),
    });

    let root_id = tree.get_root_id();

    let mut iterator = XmlEventReader::from_source(source);
    let mut state = ParseState::new(&mut tree);

    deserialize_root(&mut iterator, &mut state, root_id)?;
    apply_id_rewrites(&mut state);

    Ok(tree)
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
) -> Result<(), NewDecodeError> {
    match reader.expect_next()? {
        XmlReadEvent::StartDocument { .. } => {},
        _ => unreachable!(),
    }

    {
        let attributes = reader.expect_start_with_name("roblox")?;

        let mut found_version = false;
        for attribute in attributes.into_iter() {
            if attribute.name.local_name == "version" {
                found_version = true;

                if attribute.value != "4" {
                    return Err(reader.error(DecodeErrorKind::WrongDocVersion(attribute.value)));
                }
            }
        }

        if !found_version {
            return Err(reader.error(DecodeErrorKind::MissingAttribute("version")));
        }
    }

    loop {
        match reader.expect_peek()? {
            XmlReadEvent::StartElement { name, .. } => {
                match name.local_name.as_str() {
                    "Item" => {
                        deserialize_instance(reader, state, parent_id)?;
                    }
                    "External" => {
                        // This tag is always meaningless, there's nothing to do
                        // here except skip it.
                        reader.eat_unknown_tag()?;
                    }
                    "Meta" => {
                        deserialize_metadata(reader, state)?;
                    }
                    _ => {
                        let event = reader.expect_next().unwrap();
                        return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
                    }
                }
            }
            XmlReadEvent::EndElement { name } => {
                if name.local_name == "roblox" {
                    reader.expect_next().unwrap();
                    break;
                } else {
                    let event = reader.expect_next().unwrap();
                    return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
                }
            }
            XmlReadEvent::EndDocument => break,
            _ => {
                let event = reader.expect_next().unwrap();
                return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
            }
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
) -> Result<(), NewDecodeError> {
    let (class_name, referent) = {
        let attributes = reader.expect_start_with_name("Item")?;

        let mut class = None;
        let mut referent = None;

        for attribute in attributes.into_iter() {
            match attribute.name.local_name.as_str() {
                "class" => class = Some(attribute.value),
                "referent" => referent = Some(attribute.value),
                _ => {},
            }
        }

        let class = class
            .ok_or_else(|| reader.error(DecodeErrorKind::MissingAttribute("class")))?;

        (class, referent)
    };

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
        match reader.expect_peek()? {
            XmlReadEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "Properties" => {
                    deserialize_properties(reader, state, instance_id, &mut properties)?;
                }
                "Item" => {
                    deserialize_instance(reader, state, instance_id)?;
                }
                _ => {
                    let event = reader.expect_next().unwrap();
                    return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
                }
            }
            XmlReadEvent::EndElement { name } => {
                if name.local_name != "Item" {
                    let event = reader.expect_next().unwrap();
                    return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
                }

                reader.expect_next().unwrap();

                break;
            }
            _ => {
                let event = reader.expect_next().unwrap();
                return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
            }
        }
    }

    let instance = state.tree.get_instance_mut(instance_id).unwrap();

    instance.name = match properties.remove("Name") {
        Some(value) => match value {
            RbxValue::String { value } => value,
            _ => return Err(reader.error(DecodeErrorKind::NameMustBeString(value.get_type()))),
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
) -> Result<(), NewDecodeError> {
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