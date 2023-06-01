use std::{collections::HashMap, io::BufRead};

use quick_xml::{events::Event, Reader};

use rbx_dom_weak::{
    types::{Ref, SharedString, Variant},
    InstanceBuilder, WeakDom,
};
use rbx_reflection::ReflectionDatabase;

use super::{
    data_types,
    error::{DecodeError, ErrorKind},
    reader::{XmlData, XmlReader},
};

pub(crate) fn deserialize_file<R: BufRead>(
    mut reader: XmlReader<R>,
    config: DecodeConfig,
) -> Result<(), DecodeError> {
    log::trace!("beginning file deserialization");
    let mut roblox = reader
        .expect_start_with_name("roblox")
        .map_err(|_| ErrorKind::InvalidFile("did not open with Roblox element").err())?;
    let version = roblox.get_attribute("version")?;
    if version != "4" {
        // The error must return an owned string because we don't want to attach
        // a lifetime to errors
        return Err(ErrorKind::InvalidVersion(version.into()).err());
    }

    let mut state = XmlState::new();
    let root = InstanceBuilder::new("DataModel");
    let root_ref = root.referent();

    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => match name.as_str() {
                    "Meta" => deserialize_metadata(&mut reader, &mut state)?,
                    "SharedStrings" => deserialize_sstr(&mut reader, &mut state)?,
                    "Item" => deserialize_item(&mut reader, &mut state)?,
                    // `MetaBreakpoints` and `DebuggerManager` also exist in
                    // some XML files but we don't support those in any
                    // capacity right now.
                    "External" => {
                        log::trace!("skipping External element");
                        reader.skip_element()?
                    }
                    _ => {
                        log::trace!("unexpected element {name}");
                        // Rust's won't allow use to reference 'name` after
                        // `reader` is borrowed, so we simply don't. We needed
                        // to clone the name anyway (to deref it), so it works
                        // out.
                        let name_clone = name.clone();
                        reader.expect_next()?;
                        return Err(ErrorKind::UnknownElement(name_clone).err());
                    }
                },
                XmlData::ElementEnd { name } if name == "roblox" => {
                    log::trace!("finishing deserialization");
                    break;
                }
                event => {
                    log::trace!("unexpected event {event:?}");
                    reader.expect_next()?;
                    return Err(ErrorKind::UnexpectedToken.err());
                }
            },
            // This is safe to unwrap because `peek` guarantees we know
            // what `next` returns
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }

    log::debug!("Deserialized {} Instances", state.items.len());

    Ok(())
}

fn deserialize_metadata<R: BufRead>(
    reader: &mut XmlReader<R>,
    state: &mut XmlState,
) -> Result<(), DecodeError> {
    log::trace!("decoding Meta");
    let mut element = reader.expect_start_with_name("Meta")?;
    let name = element.get_attribute("name")?;
    let value = reader.eat_text()?;
    reader.expect_end_with_name("Meta")?;

    log::debug!("Found metadata {name} = {value}");
    state.metadata.insert(name, value);
    Ok(())
}

fn deserialize_sstr<R: BufRead>(
    reader: &mut XmlReader<R>,
    state: &mut XmlState,
) -> Result<(), DecodeError> {
    log::trace!("decoding SharedStrings");

    reader.expect_start_with_name("SharedStrings")?;
    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { .. } => {
                    let mut sstr = reader.expect_start_with_name("SharedString")?;
                    let hash = sstr.get_attribute("md5")?;
                    let value = base64::decode(reader.eat_text()?)?;
                    reader.expect_end_with_name("SharedString")?;

                    log::debug!("Found SharedString {hash} ({} bytes)", value.len());
                    state.shared_strings.insert(hash, SharedString::new(value));
                }
                XmlData::ElementEnd { name } if name == "SharedStrings" => {
                    log::trace!("finishing SharedStrings decoding");
                    reader.next();
                    return Ok(());
                }
                event => {
                    log::trace!("unexpected event {event:?}");
                    reader.expect_next()?;
                    return Err(ErrorKind::UnexpectedToken.err());
                }
            },
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
}

fn deserialize_item<R: BufRead>(
    reader: &mut XmlReader<R>,
    state: &mut XmlState,
) -> Result<(), DecodeError> {
    let mut item = reader.expect_start_with_name("Item")?;
    let class = item.get_attribute("class")?;
    // !!Change in behavior!!
    // Previously, `referent` wasn't required, it now is
    let read_ref = item.get_attribute("referent")?;
    log::trace!("decoding Instance {read_ref} of class {class}");
    let mut inst = InstanceBuilder::new(class);
    let real_ref = inst.referent();
    let mut properties = HashMap::new();

    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => match name.as_str() {
                    "Properties" => {
                        deserialize_properties(reader, state, real_ref, &mut properties)?
                    }

                    "Item" => deserialize_item(reader, state)?,
                    _ => {
                        log::trace!("unexpected element {name}");
                        let name_clone = name.clone();
                        reader.expect_next()?;
                        return Err(ErrorKind::UnknownElement(name_clone).err());
                    }
                },
                XmlData::ElementEnd { name } if name == "Item" => {
                    log::trace!("finishing Item decoding");
                    reader.next();
                    break;
                }
                event => {
                    log::trace!("unexpected event {event:?}");
                    reader.expect_next()?;
                    return Err(ErrorKind::UnexpectedToken.err());
                }
            },
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
    log::trace!("found {} properties", properties.len());
    inst.add_properties(properties);
    state.items.insert(read_ref, inst);

    Ok(())
}

fn deserialize_properties<R: BufRead>(
    reader: &mut XmlReader<R>,
    state: &mut XmlState,
    referent: Ref,
    properties: &mut HashMap<String, Variant>,
) -> Result<(), DecodeError> {
    log::trace!("decoding Properties");
    reader.expect_start_with_name("Properties")?;
    let start = if log::log_enabled!(log::Level::Debug) {
        Some(std::time::Instant::now())
    } else {
        None
    };
    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => {
                    // We may be able to get around this later but for now
                    // we have to take ownership of the element name.
                    let prop_type = name.clone();

                    match prop_type.as_str() {
                        "bool" | "string" | "Ref" | "SharedString" | "float" | "double" | "int"
                        | "int64" => {
                            let mut element = reader.expect_start_with_name(&prop_type)?;
                            let prop_name = element.get_attribute("name")?;
                            // TODO filter parsing based on whether we care about unknown properties
                            log::trace!("decoding Property {prop_name} of type {prop_type}");

                            let variant = deserialize_property(reader, &prop_type)?;
                            if prop_type == "Ref" {
                                log::trace!("referent property {prop_name} = {variant:?}");
                                state.ref_properties.push((referent, prop_name.clone()));
                            } else if prop_type == "SharedString" {
                                log::trace!("SharedString property {prop_name} = {variant:?}");
                                state.sstr_properties.push((referent, prop_name.clone()));
                            }
                            properties.insert(prop_name, variant);
                            reader.expect_end_with_name(&prop_type)?;
                        }
                        _ => {
                            // TODO bail if we care about unknown types
                            log::trace!("unknown property type {prop_type}");
                            reader.skip_element()?;
                        }
                    };
                }
                XmlData::ElementEnd { name } if name == "Properties" => {
                    log::trace!("finished decoding properties");
                    reader.next();
                    if log::log_enabled!(log::Level::Debug) {
                        log::debug!(
                            "deserializing properties took {:02}s",
                            std::time::Instant::now()
                                .duration_since(start.unwrap())
                                .as_secs_f32()
                        )
                    }
                    return Ok(());
                }
                event => {
                    log::trace!("unexpected event {event:?}");
                    reader.expect_next()?;
                    return Err(ErrorKind::UnexpectedToken.err());
                }
            },
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
}

fn deserialize_property<R: BufRead>(
    reader: &mut XmlReader<R>,
    prop_type: &str,
) -> Result<Variant, DecodeError> {
    Ok(match prop_type {
        // We rewrite these later in the deserialization
        "Ref" | "SharedString" => Variant::String(data_types::string_deserializer(reader)?),
        "BinaryString" => Variant::BinaryString(data_types::binary_string_deserializer(reader)?),

        "bool" => Variant::Bool(data_types::bool_deserializer(reader)?),
        "string" | "ProtectedString" => Variant::String(data_types::string_deserializer(reader)?),
        "float" => Variant::Float32(data_types::f32_deserializer(reader)?),
        "double" => Variant::Float64(data_types::f64_deserializer(reader)?),
        "int" => Variant::Int32(data_types::i32_deserializer(reader)?),
        "int64" => Variant::Int64(data_types::i64_deserializer(reader)?),
        // we're checking this in the match for Properties
        _ => unreachable!(),
    })
}

#[derive(Debug, Default)]
struct XmlState {
    /// A map of metadata values deserialized from `Meta` elements
    metadata: HashMap<String, String>,
    /// A map of SharedString values deserialized from the file
    shared_strings: HashMap<String, SharedString>,
    /// A map of all Instances contained in the file to their read referents
    items: HashMap<String, InstanceBuilder>,
    /// A list of properties that point to other Instances in the file.
    /// They need to be revisited after we're done deserializing so they can be
    /// rewritten to use our actual referents.
    ///
    /// The tuple is `(InstanceBuilder Referent, Property Name)`
    ref_properties: Vec<(Ref, String)>,
    /// A list of properties that point to a SharedString. They need to be
    /// revisited after we're done deserializing so we can actually point to
    /// them.
    ///
    /// The tuple is `(Instance Referent, Property Name)`
    sstr_properties: Vec<(Ref, String)>,
}

impl XmlState {
    fn new() -> Self {
        XmlState::default()
    }
}

/// A struct configuring the behavior of the deserializer.
/// By default, this uses no database. To add one, use `set_database`.
pub struct DecodeConfig<'db> {
    /// What database if any to use for decoding properties and classes.
    pub(crate) database: Option<ReflectionDatabase<'db>>,
    /// When `true`, class names be checked against the database and
    /// an error will be raised when an unknown class is encountered.
    pub(crate) strict_class_names: bool,
    /// When `true`, property types will be checked against the database and
    /// an error will be raised when a type mismatch is encountered.
    pub(crate) strict_data_types: bool,
    /// When `true`, property names will be checked against the database and
    /// an error will be raised when unknown properties are encountered.
    pub(crate) strict_property_names: bool,
    /// When `true`, any new property data types will be skipped.
    /// Otherwise, an error will be raised when a new data type is encountered.
    pub(crate) ignore_new_types: bool,
}

impl<'db> Default for DecodeConfig<'db> {
    fn default() -> Self {
        Self {
            database: None,
            strict_class_names: false,
            strict_data_types: false,
            strict_property_names: false,
            // This is why we manually implement `Default`!
            ignore_new_types: true,
        }
    }
}

impl<'db> DecodeConfig<'db> {
    /// Creates a new `DecodeConfig` with the default options. This means
    /// no database is used and unknown data types are skipped during
    /// deserialization.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `DecodeConfig` with the given database. By default,
    /// class names, property names, and property types are checked.
    /// Additionally, new data types are ignored.
    pub fn with_database(database: ReflectionDatabase<'db>) -> Self {
        Self {
            database: Some(database),
            strict_class_names: true,
            strict_data_types: true,
            strict_property_names: true,
            ignore_new_types: true,
        }
    }

    /// Sets the deserializer to use the given `ReflectionDatabase`.
    pub fn database(mut self, database: ReflectionDatabase<'db>) -> Self {
        self.database = Some(database);
        self
    }

    /// Sets whether class names are checked against the database. If `true`,
    /// an error will be raised during deserialization if an unknown class
    /// is encountered.
    pub fn strict_class_names(mut self, ignore: bool) -> Self {
        self.strict_class_names = ignore;
        self
    }

    /// Sets whether property data types are checked against the database.
    /// If `true`, an error will be raised during deserialization if a
    /// property's type does not match in the database.
    pub fn strict_data_types(mut self, ignore: bool) -> Self {
        self.strict_data_types = ignore;
        self
    }

    /// Sets whether property names are checked against the database.
    /// If `true`, an error will be raised during deserialization if a
    /// property's type does not match in the database.
    pub fn strict_property_names(mut self, ignore: bool) -> Self {
        self.strict_property_names = ignore;
        self
    }

    /// Sets whether unknown property data types are ignored during
    /// deserialization. If `true`, any property of an unknown type will be
    /// skipped.
    pub fn ignore_new_types(mut self, ignore: bool) -> Self {
        self.ignore_new_types = ignore;
        self
    }
}

#[test]
fn decode_test() {
    env_logger::try_init().unwrap();
    let document = r#"
    <roblox version="4">
        <External>TestExternal</External>
        <Meta name="TestMetadata">TestValue</Meta>
        <Item class="TestClass" referent="TestReferent">
            <Properties>
                <SharedString name="TestSharedString">Test Shared String Key</SharedString>
                <UniqueId name="UniqueId">44b188dace632b4702e9c68d004815fc</UniqueId>
                <bool name="TestBool1">true</bool>
                <bool name="TestBool2">false</bool>
                <string name="TestString">Test Value</string>
                <float name="TestFloat1">-0.15625</float>
                <float name="TestFloat2">INF</float>
                <float name="TestFloat3">-INF</float>
                <float name="TestFloat4">NAN</float>
                <double name="TestDouble1">-0.15625</double>
                <double name="TestDouble2">INF</double>
                <double name="TestDouble3">-INF</double>
                <double name="TestDouble4">NAN</double>
            </Properties>
        </Item>
        <SharedStrings>
		    <SharedString md5="Test Shared String Key">Q1NHSzg1MTYxZjdlOWNmZjMyNTlhNmU1NmE2NGJjZmNjMzJh</SharedString>
        </SharedStrings>
    </roblox>
"#;

    let _ = deserialize_file(XmlReader::from_str(document), Default::default()).unwrap();
}

#[test]
fn crossroads_decode() {
    env_logger::try_init().unwrap();
    let file = std::fs::File::open("benches/crossroads.rbxlx").unwrap();

    let reader = XmlReader::from_reader(std::io::BufReader::new(file));

    let _ = deserialize_file(reader, Default::default()).unwrap();
}
