use std::{
    io::Read,
    iter::{Filter, Peekable},
    collections::HashMap,
};

use failure::Fail;
use log::trace;
use rbx_reflection::RbxPropertyTypeDescriptor;
use rbx_dom_weak::{RbxTree, RbxId, RbxInstanceProperties, RbxValue, RbxValueType};
use xml::reader::{self, ParserConfig};

use crate::{
    core::find_canonical_property_descriptor,
    types::{read_value_xml, read_ref},
};

pub use xml::reader::XmlEvent as XmlReadEvent;

/// Indicates an error trying to parse an rbxmx or rbxlx document
#[derive(Debug, Fail, Clone, PartialEq)]
pub enum DecodeError {
    #[fail(display = "XML read error: {}", _0)]
    XmlError(#[fail(cause)] reader::Error),

    #[fail(display = "Float parse error: {}", _0)]
    ParseFloatError(#[fail(cause)] std::num::ParseFloatError),

    #[fail(display = "Int parse error: {}", _0)]
    ParseIntError(#[fail(cause)] std::num::ParseIntError),

    #[fail(display = "Base64 decode error: {}", _0)]
    DecodeBase64Error(#[fail(cause)] base64::DecodeError),

    // TODO: Switch to Cow<'static, str>?
    #[fail(display = "{}", _0)]
    Message(&'static str),

    #[fail(display = "Malformed document")]
    MalformedDocument,
}

impl From<reader::Error> for DecodeError {
    fn from(error: reader::Error) -> DecodeError {
        DecodeError::XmlError(error)
    }
}

impl From<std::num::ParseFloatError> for DecodeError {
    fn from(error: std::num::ParseFloatError) -> DecodeError {
        DecodeError::ParseFloatError(error)
    }
}

impl From<std::num::ParseIntError> for DecodeError {
    fn from(error: std::num::ParseIntError) -> DecodeError {
        DecodeError::ParseIntError(error)
    }
}

impl From<base64::DecodeError> for DecodeError {
    fn from(error: base64::DecodeError) -> DecodeError {
        DecodeError::DecodeBase64Error(error)
    }
}

/// A utility method to decode an XML-format model from a string.
pub fn decode_str(tree: &mut RbxTree, parent_id: RbxId, source: &str) -> Result<(), DecodeError> {
    decode(tree, parent_id, source.as_bytes())
}

/// Decodes source from the given buffer into the instance in the given tree.
///
/// Roblox model files can contain multiple instances at the top level. This
/// happens in the case of places as well as Studio users choosing multiple
/// objects when saving a model file.
pub fn decode<R: Read>(tree: &mut RbxTree, parent_id: RbxId, source: R) -> Result<(), DecodeError> {
    let mut iterator = XmlEventReader::from_source(source);
    let mut state = ParseState::new(tree);

    deserialize_root(&mut iterator, &mut state, parent_id)?;
    apply_id_rewrites(&mut state);

    Ok(())
}

/// Since this function type needs to be mentioned a couple times, we keep this
/// type alias around.
type EventFilterFn = fn(&Result<XmlReadEvent, xml::reader::Error>) -> bool;

fn filter_whitespace_events(event: &Result<XmlReadEvent, xml::reader::Error>) -> bool {
    match event {
        Ok(XmlReadEvent::Whitespace(_)) => false,
        _ => true,
    }
}

/// A wrapper around an XML event iterator created by xml-rs.
pub struct XmlEventReader<R: Read> {
    inner: Peekable<Filter<reader::Events<R>, EventFilterFn>>,
}

impl<R: Read> XmlEventReader<R> {
    /// Borrows the next element from the event stream without consuming it.
    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        self.inner.peek()
    }

    /// Constructs a new `XmlEventReader` from a source that implements `Read`.
    pub fn from_source(source: R) -> XmlEventReader<R> {
        let reader = ParserConfig::new()
            .ignore_comments(true)
            .create_reader(source);

        XmlEventReader {
            inner: reader.into_iter().filter(filter_whitespace_events as EventFilterFn).peekable(),
        }
    }

    /// Consumes the next event and returns `Ok(())` if it was an opening tag
    /// with the given name, otherwise returns an error.
    pub fn expect_start_with_name(&mut self, expected_name: &str) -> Result<(), DecodeError> {
        read_event!(self, XmlReadEvent::StartElement { name, .. } => {
            if name.local_name != expected_name {
                return Err(DecodeError::Message("Wrong opening tag"));
            }
        });

        Ok(())
    }

    /// Consumes the next event and returns `Ok(())` if it was a closing tag
    /// with the given name, otherwise returns an error.
    pub fn expect_end_with_name(&mut self, expected_name: &str) -> Result<(), DecodeError> {
        read_event!(self, XmlReadEvent::EndElement { name } => {
            if name.local_name != expected_name {
                return Err(DecodeError::Message("Wrong closing tag"));
            }
        });

        Ok(())
    }

    /// Reads one `Characters` or `CData` event if the next event is a
    /// `Characters` or `CData` event.
    ///
    /// If the next event in the stream is not a character event, this function
    /// will return `Ok(None)` and leave the stream untouched.
    ///
    /// This is the inner kernel of `read_characters`, which is the public
    /// version of a similar idea.
    fn read_one_characters_event(&mut self) -> Result<Option<String>, DecodeError> {
        // This pattern (peek + next) is pretty gnarly but is useful for looking
        // ahead without touching the stream.

        match self.peek() {
            // If the next event is a `Characters` or `CData` event, we need to
            // use `next` to take ownership over it (with some careful unwraps)
            // and extract the data out of it.
            //
            // We could also clone the borrowed data obtained from peek, but
            // some of the character events can contain several megabytes of
            // data, so a copy is really expensive.
            Some(Ok(XmlReadEvent::Characters(_))) | Some(Ok(XmlReadEvent::CData(_))) => {
                match self.next().unwrap().unwrap() {
                    XmlReadEvent::Characters(value) | XmlReadEvent::CData(value) => Ok(Some(value)),
                    _ => unreachable!()
                }
            }

            // Since we can't use `?` (we have a `&Result` instead of a `Result`)
            // we have to do something similar to what it would do.
            Some(Err(_)) => Err(self.next().unwrap().unwrap_err().into()),

            None | Some(Ok(_)) => Ok(None),
        }
    }

    /// Reads a contiguous sequence of zero or more `Characters` and `CData`
    /// events from the event stream.
    ///
    /// Normally, consumers of xml-rs shouldn't need to do this since the
    /// combination of `cdata_to_characters` and `coalesce_characters` does
    /// something very similar. Because we want to support CDATA sequences that
    /// contain only whitespace, we have two options:
    ///
    /// 1. Every time we want to read an XML event, use a loop and skip over all
    ///    `Whitespace` events
    ///
    /// 2. Turn off `cdata_to_characters` in `ParserConfig` and use a regular
    ///    iterator filter to strip `Whitespace` events
    ///
    /// For complexity, performance, and correctness reasons, we switched from
    /// #1 to #2. However, this means we need to coalesce `Characters` and
    /// `CData` events ourselves.
    pub fn read_characters(&mut self) -> Result<String, DecodeError> {
        let mut buffer = match self.read_one_characters_event()? {
            Some(buffer) => buffer,
            None => return Ok(String::new()),
        };

        loop {
            match self.read_one_characters_event()? {
                Some(piece) => buffer.push_str(&piece),
                None => break,
            }
        }

        Ok(buffer)
    }

    /// Reads a tag completely and returns its text content. This is intended
    /// for parsing simple tags where we don't care about the attributes or
    /// children, only the text value, for Vector3s and such, which are encoded
    /// like:
    ///
    /// <Vector3>
    ///     <X>0</X>
    ///     <Y>0</Y>
    ///     <Z>0</Z>
    /// </Vector3>
    pub fn read_tag_contents(&mut self, expected_name: &str) -> Result<String, DecodeError> {
        read_event!(self, XmlReadEvent::StartElement { name, .. } => {
            if name.local_name != expected_name {
                return Err(DecodeError::Message("Got wrong tag name"));
            }
        });

        let contents = self.read_characters()?;

        read_event!(self, XmlReadEvent::EndElement { .. } => {});

        Ok(contents)
    }

    /// Consume events from the iterator until we reach the end of the next tag.
    pub fn eat_unknown_tag(&mut self) -> Result<(), DecodeError> {
        let mut depth = 0;

        trace!("Starting unknown block");

        loop {
            match self.next().ok_or(DecodeError::Message("Unexpected EOF"))?? {
                XmlReadEvent::StartElement { name, .. } => {
                    trace!("Eat unknown start: {:?}", name);
                    depth += 1;
                },
                XmlReadEvent::EndElement { name } => {
                    trace!("Eat unknown end: {:?}", name);
                    depth -= 1;

                    if depth == 0 {
                        trace!("Reached end of unknown block");
                        break;
                    }
                },
                other => {
                    trace!("Eat unknown: {:?}", other);
                },
            }
        }

        Ok(())
    }
}

impl<R: Read> Iterator for XmlEventReader<R> {
    type Item = reader::Result<XmlReadEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
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

fn deserialize_metadata<R: Read>(reader: &mut XmlEventReader<R>, state: &mut ParseState) -> Result<(), DecodeError> {
    // TODO: Strongly type metadata instead?

    let name = read_event!(reader, XmlReadEvent::StartElement { name, mut attributes, .. } => {
        assert_eq!(name.local_name, "Meta");

        let mut name = None;

        for attribute in attributes.drain(..) {
            match attribute.name.local_name.as_str() {
                "name" => name = Some(attribute.value),
                _ => {},
            }
        }

        name.ok_or(DecodeError::Message("Meta missing 'name' field"))?
    });

    let value = read_event!(reader, XmlReadEvent::Characters(value) => value);

    read_event!(reader, XmlReadEvent::EndElement { name, .. } => {
        if name.local_name != "Meta" {
            return Err(DecodeError::Message("Incorrect closing tag, expected 'Meta'"));
        }
    });

    trace!("Metadata: {} = {}", name, value);

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
    read_event!(reader, XmlReadEvent::StartElement { name, .. } => {
        assert_eq!(name.local_name, "Properties");
    });

    let class_name = state.tree.get_instance(instance_id)
        .expect("Couldn't find instance to deserialize properties into")
        .class_name.clone();

    'property_loop: loop {
        let (property_type, xml_property_name) = loop {
            match reader.peek().ok_or(DecodeError::Message("Unexpected EOF"))? {
                Ok(XmlReadEvent::StartElement { name, attributes, .. }) => {
                    let mut xml_property_name = None;

                    for attribute in attributes {
                        if attribute.name.local_name.as_str() == "name" {
                            xml_property_name = Some(attribute.value.to_owned());
                            break;
                        }
                    }

                    let xml_property_name = xml_property_name
                        .ok_or(DecodeError::Message("Missing 'name' for property tag"))?;

                    break (name.local_name.to_owned(), xml_property_name)
                },
                Ok(XmlReadEvent::EndElement { name }) => {
                    if name.local_name == "Properties" {
                        reader.next().unwrap()?;
                        return Ok(())
                    } else {
                        trace!("Unexpected end element {:?}, expected Properties", name);
                        return Err(DecodeError::Message("Unexpected end element, expected Properties"))
                    }
                },
                Ok(_) | Err(_) => return Err(DecodeError::Message("Unexpected thing in Properties section")),
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
                        Some(value) => value.into_owned(),
                        None => xml_value,
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