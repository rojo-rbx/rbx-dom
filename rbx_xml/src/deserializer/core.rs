use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use rbx_dom_weak::{
    types::{Ref, SharedString, Variant, VariantType},
    InstanceBuilder, WeakDom,
};
use rbx_reflection::DataType;

use crate::{
    deserializer::conversions, property_descriptor::find_canonical_property_descriptor, Config,
};

use super::{
    data_types,
    error::{DecodeError, ErrorKind},
    reader::{XmlData, XmlReader},
};

pub(crate) fn deserialize_file<'a, R: BufRead>(
    mut reader: XmlReader<R>,
    config: &'a Config<'a>,
) -> Result<WeakDom, DecodeError> {
    log::trace!("beginning file deserialization");
    let mut roblox = reader.expect_start_with_name("roblox")?;
    let version = roblox.get_attribute("version")?;
    if version != "4" {
        // The error must return an owned string because we don't want to attach
        // a lifetime to errors
        return Err(ErrorKind::InvalidVersion(version).err());
    }

    let root = InstanceBuilder::new("DataModel");
    let root_ref = root.referent();
    let mut state = XmlState {
        dom: WeakDom::new(root),
        metadata: HashMap::new(),
        shared_strings: HashMap::new(),
        read_refs: HashMap::new(),
        ref_properties: Vec::new(),
        sstr_properties: Vec::new(),
        unknown_types: HashSet::new(),
        config,
    };

    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => match name.as_str() {
                    "Meta" => deserialize_metadata(&mut reader, &mut state)?,
                    "SharedStrings" => deserialize_sstr(&mut reader, &mut state)?,
                    "Item" => deserialize_item(&mut reader, &mut state, root_ref)?,
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
                    return Err(ErrorKind::UnexpectedToken(reader.offset()).err());
                }
            },
            // This is safe to unwrap because `peek` guarantees we know
            // what `next` returns
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
    log::debug!("Deserialized {} Instances", state.read_refs.len());

    log::debug!("Rewriting Referent properties");
    for (inst_referent, prop_name, prop_value) in state.ref_properties {
        log::trace!("rewriting {inst_referent}.{prop_name}");
        let inst = state.dom.get_by_ref_mut(inst_referent).unwrap();

        if inst.properties.get(&prop_name).is_none() {
            if let Some(value) = state.read_refs.get(&prop_value) {
                inst.properties.insert(prop_name, Variant::Ref(*value));
            }
        } else {
            return Err(ErrorKind::DuplicateProperty(prop_name).err());
        }
    }

    log::debug!("Rewriting SharedString properties");
    for (inst_referent, prop_name, prop_value) in state.sstr_properties {
        log::trace!("rewriting {inst_referent}.{prop_name}");
        let inst = state.dom.get_by_ref_mut(inst_referent).unwrap();

        if inst.properties.get(&prop_name).is_none() {
            if let Some(value) = state.shared_strings.get(&prop_value) {
                inst.properties
                    .insert(prop_name, Variant::SharedString(value.clone()));
            }
        } else {
            return Err(ErrorKind::DuplicateProperty(prop_name).err());
        }
    }

    for unknown_type in state.unknown_types {
        log::warn!("Unknown property type {unknown_type}");
    }

    Ok(state.dom)
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
                    let value = reader.eat_base64()?;
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
                    return Err(ErrorKind::UnexpectedToken(reader.offset()).err());
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
    parent: Ref,
) -> Result<(), DecodeError> {
    let mut item = reader.expect_start_with_name("Item")?;
    let class = item.get_attribute("class")?;
    // !!Change in behavior!!
    // Previously, `referent` wasn't required, it now is
    let read_ref = item.get_attribute("referent")?;

    if state.config.strict_class_names {
        if let Some(database) = state.config.database {
            if !database.classes.contains_key(class.as_str()) {
                return Err(ErrorKind::UnknownClass(class, read_ref).err());
            }
        } else {
            return Err(ErrorKind::StrictWithoutDatabase("class names").err());
        }
    }
    log::debug!("Attempting to deserialize Instance {read_ref} of class {class}");

    let real_ref = state.dom.insert(parent, InstanceBuilder::new(class));
    let mut properties = HashMap::new();

    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => match name.as_str() {
                    "Properties" => {
                        deserialize_properties(reader, state, real_ref, &mut properties)?
                    }

                    "Item" => deserialize_item(reader, state, real_ref)?,
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
                    return Err(ErrorKind::UnexpectedToken(reader.offset()).err());
                }
            },
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
    log::trace!("found {} properties", properties.len());
    let inst = state.dom.get_by_ref_mut(real_ref).unwrap();

    if let Some(value) = properties.remove("Name") {
        if let Variant::String(name) = value {
            inst.name = name
        } else {
            return Err(ErrorKind::NameNotString(value.ty()).err());
        }
    }

    // TODO make this... less strict?
    // This fails on impossible conversions, which is actually more restrictive
    // than what the old rbx_xml did. We'll have to either establish more
    // migrations or simply ignore any that can't happen.
    if state.config.strict_data_types || state.config.strict_property_names {
        if let Some(database) = state.config.database {
            for (prop_name, value) in properties.iter_mut() {
                let class_name = &inst.class;
                let canonical = find_canonical_property_descriptor(database, class_name, prop_name);
                match canonical {
                    Some(descriptor) => {
                        let canonical_type = match &descriptor.data_type {
                            DataType::Value(ty) => *ty,
                            DataType::Enum(_) => VariantType::Enum,
                            _ => unimplemented!(),
                        };
                        if canonical_type != value.ty() {
                            log::debug!("Attempting to convert {class_name}.{prop_name} to be of type {canonical_type:?} (currently {:?})", value.ty());
                            if let Err(error) = conversions::convert(value, canonical_type) {
                                log::error!("Could not convert {value:?} to {canonical_type:?} because {error}");
                                // return Err(ErrorKind::ConversionFail {
                                //     class: class_name.clone(),
                                //     name: prop_name.clone(),
                                //     from: value.ty(),
                                //     to: canonical_type,
                                //     error,
                                // }
                                // .err());
                            }
                        }
                    }
                    None => {
                        if state.config.strict_property_names {
                            return Err(ErrorKind::UnknownProperty(
                                class_name.to_owned(),
                                prop_name.to_owned(),
                            )
                            .err());
                        }
                    }
                }
            }
        } else {
            return Err(ErrorKind::StrictWithoutDatabase("properties").err());
        }
    }

    inst.properties = properties;

    state.read_refs.insert(read_ref, real_ref);

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

    loop {
        match reader.peek() {
            Some(Ok(event)) => match event {
                XmlData::ElementStart { name, .. } => {
                    // We may be able to get around this later but for now
                    // we have to take ownership of the element name.
                    let prop_type = name.clone();
                    if data_types::is_known_type(&prop_type) {
                        let mut element = reader.expect_start_with_name(&prop_type)?;
                        let prop_name = element.get_attribute("name")?;
                        if log::log_enabled!(log::Level::Debug) {
                            let class_name = &state.dom.get_by_ref(referent).unwrap().class;
                            log::debug!("Attempting to deserialize property {class_name}.{prop_name} of type {prop_type}");
                        }

                        let data_offset = reader.offset();
                        let variant = match data_types::attempt_deserialization(reader, &prop_type)
                        {
                            Ok(v) => v,
                            Err(error) => {
                                return Err(ErrorKind::PropertyNotReadable {
                                    name: prop_name,
                                    offset: data_offset,
                                    message: error.to_string(),
                                }
                                .err())
                            }
                        };

                        if prop_type == "Ref" {
                            log::trace!("referent property {prop_name} = {variant:?}");
                            state.ref_properties.push((
                                referent,
                                prop_name,
                                match variant {
                                    Variant::String(str) => str,
                                    _ => unreachable!(),
                                },
                            ));
                        } else if prop_type == "SharedString" {
                            log::trace!("SharedString property {prop_name} = {variant:?}");
                            state.sstr_properties.push((
                                referent,
                                prop_name,
                                match variant {
                                    Variant::String(str) => str,
                                    _ => unreachable!(),
                                },
                            ));
                        } else if properties.get(&prop_name).is_none() {
                            properties.insert(prop_name, variant);
                        } else {
                            return Err(ErrorKind::DuplicateProperty(prop_name).err());
                        }

                        reader.expect_end_with_name(&prop_type)?;
                    } else if state.config.ignore_new_types {
                        state.unknown_types.insert(prop_type);
                        reader.skip_element()?;
                    } else {
                        log::error!("Unknown property type {prop_type}");
                        return Err(ErrorKind::UnknownType(prop_type).err());
                    }
                }
                XmlData::ElementEnd { name } if name == "Properties" => {
                    log::trace!("finished decoding properties");
                    reader.next();
                    return Ok(());
                }
                event => {
                    log::trace!("unexpected event {event:?}");
                    reader.expect_next()?;
                    return Err(ErrorKind::UnexpectedToken(reader.offset()).err());
                }
            },
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ErrorKind::UnexpectedEof.err()),
        }
    }
}

#[derive(Debug)]
struct XmlState<'db> {
    /// The internal WeakDom used by the decoder
    dom: WeakDom,
    /// A map of metadata values deserialized from `Meta` elements
    metadata: HashMap<String, String>,
    /// A map of SharedString values deserialized from the file
    shared_strings: HashMap<String, SharedString>,
    /// A map of all read Instance referents to their real referents
    read_refs: HashMap<String, Ref>,
    /// A list of properties that point to other Instances in the file.
    /// They need to be revisited after we're done deserializing so they can be
    /// rewritten to use our actual referents.
    ///
    /// The tuple is `(InstanceBuilder Referent, Property Name, Read Value)`
    ref_properties: Vec<(Ref, String, String)>,
    /// A list of properties that point to a SharedString. They need to be
    /// revisited after we're done deserializing so we can actually point to
    /// them.
    ///
    /// The tuple is `(Instance Referent, Property Name, Read Value)`
    sstr_properties: Vec<(Ref, String, String)>,
    /// A set of unknown data types encountered while deserializing the file.
    /// This is utilized to ensure an error isn't emitted more than one time
    /// per unknown type.
    unknown_types: HashSet<String>,
    config: &'db Config<'db>,
}

#[cfg(test)]
mod tests {
    use rbx_dom_weak::DomViewer;

    use super::*;

    #[test]
    fn decode_test() {
        #![allow(unused_must_use)]
        env_logger::try_init();
        let document = r#"
            <roblox version="4">
                <External>TestExternal</External>
                <Meta name="TestMetadata">TestValue</Meta>
                <Item class="TestClass" referent="TestReferent">
                    <Properties>
                        <string name = "Name">"Test Name"</string>
                        <Ref name = "RefTest">null</Ref>
                        <SharedString name="TestSharedString">Test Shared String Key</SharedString>
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
                        <Vector3 name="TestVector3">
                            <X>1337</X>
                            <Y>123456789.10</Y>
                            <Z>-4276993775</Z>
                        </Vector3>
                        <Axes name="TestAxes1">
                            <axes>0</axes>
                        </Axes>
                        <Axes name="TestAxes2">
                            <axes>4</axes>
                        </Axes>
                        <Axes name="TestAxes3">
                            <axes>7</axes>
                        </Axes>
                    </Properties>
                    <Item class="TestClass2" referent="TestReferent2">
                        <Properties>
                            <Ref name = "RefTest">TestReferent</Ref>
                            <SharedString name="TestSharedString">Test Shared String Key</SharedString>
                            <int name = "TestInt1">10</int>
                            <int name = "TestInt2">-10</int>
                            <int64 name = "TestInt64_1">20</int64>
                            <int64 name = "TestInt64_2">-20</int64>
                            <ProtectedString name = "TestProtectedString"><![CDATA[   Protected String   ]]></ProtectedString>
                            <Ray name="TestRay">
                                <origin>
                                    <X>1</X>
                                    <Y>2</Y>
                                    <Z>3</Z>
                                </origin>
                                <direction>
                                    <X>-4</X>
                                    <Y>-5</Y>
                                    <Z>-6</Z>
                                </direction>
                            </Ray>
                            <CoordinateFrame name="TestCFrame">
                                <X>1</X>
                                <Y>2</Y>
                                <Z>3</Z>
                                <R00>INF</R00>
                                <R01>NAN</R01>
                                <R02>-INF</R02>
                                <R10>-1</R10>
                                <R11>-2</R11>
                                <R12>-3</R12>
                                <R20>0.15625</R20>
                                <R21>-0.5</R21>
                                <R22>0</R22>
                            </CoordinateFrame>
                            <Color3 name="TestColor3">
                                <R>1</R>
                                <G>0.70588237</G>
                                <B>0.0784313753</B>
                            </Color3>
                            <Color3uint8 name="TestColor3uint8">4288914085</Color3uint8>
                            <Faces name="TestFaces1">
                                <faces>0</faces>
                            </Faces>
                            <Faces name="TestFaces2">
                                <faces>31</faces>
                            </Faces>
                            <Faces name="TestFaces3">
                                <faces>55</faces>
                            </Faces>
                        </Properties>
                    </Item>
                </Item>
                <SharedStrings>
                    <SharedString md5="Test Shared String Key">Q1NHSzg1MTYxZjdlOWNmZjMyNTlhNmU1NmE2NGJjZmNjMzJh</SharedString>
                </SharedStrings>
            </roblox>
        "#;

        match deserialize_file(XmlReader::from_str(document), &Default::default()) {
            Err(err) => panic!("{}", err),
            Ok(dom) => {
                insta::assert_yaml_snapshot!(
                    "deserializer feature list",
                    DomViewer::new().view(&dom)
                )
            }
        }
    }

    #[test]
    fn crossroads_decode() {
        #![allow(unused_must_use)]
        env_logger::try_init();
        let file = std::fs::File::open("benches/crossroads.rbxlx").unwrap();

        let reader = XmlReader::from_reader(std::io::BufReader::new(file));
        if let Err(err) = deserialize_file(reader, &Default::default()) {
            panic!("{}", err)
        }
    }

    #[test]
    fn crossroads_strict() {
        #![allow(unused_must_use)]
        env_logger::try_init();
        let file = std::fs::File::open("benches/crossroads.rbxlx").unwrap();

        let reader = XmlReader::from_reader(std::io::BufReader::new(file));
        if let Err(err) = deserialize_file(
            reader,
            &Config::with_database(rbx_reflection_database::get()),
        ) {
            panic!("{}", err)
        }
    }

    #[test]
    fn decode_strict() {
        #![allow(unused_must_use)]
        env_logger::try_init();
        let document = r#"
            <roblox version="4">
                <Item class="Workspace" referent="TestReferent">
                    <Properties>
                        <string name = "Name">Test Workspace</string>
                    </Properties>
                    <Item class="Part" referent="TestReferent2">
                        <Properties>
                            <string name = "Name">Test Part</string>
                            <int name = "BrickColor">137</int>
                        </Properties>
                    </Item>
                </Item>
            </roblox>
        "#;

        match deserialize_file(
            XmlReader::from_str(document),
            &Config::with_database(rbx_reflection_database::get()),
        ) {
            Err(err) => panic!("{}", err),
            Ok(dom) => {
                insta::assert_yaml_snapshot!(
                    "deserialize with database",
                    DomViewer::new().view(&dom)
                )
            }
        }
    }
}