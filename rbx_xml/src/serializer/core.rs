use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    io,
};

use base64::Engine;

use rbx_dom_weak::{
    types::{Ref, SharedString, Variant, VariantType},
    WeakDom,
};
use rbx_reflection::DataType;

use crate::{property_descriptor::find_serialized_property_descriptor, Config};

use super::{
    conversions, data_types,
    error::{EncodeError, ErrorKind},
};

use super::writer::XmlWriter;

pub fn serialize_dom<'a, W: io::Write>(
    output: &mut W,
    dom: &WeakDom,
    config: &'a Config,
) -> Result<(), EncodeError> {
    serialize_refs(output, dom, dom.root().children(), config)
}

pub fn serialize_refs<'a, W: io::Write>(
    output: &mut W,
    dom: &WeakDom,
    refs: &[Ref],
    config: &'a Config<'a>,
) -> Result<(), EncodeError> {
    let mut writer = XmlWriter::new(output, Some((b' ', 2)));

    writer
        .start_element("roblox")
        .attribute("version", "4")
        .finalize()?;

    // TODO add a way to preallocate this for the number of Instances in a file
    let mut state = EncodeState {
        shared_strings: HashSet::new(),
        ref_map: HashMap::new(),
        ref_strings: Vec::new(),
        next_ref: 0,
        config,
    };
    let mut prop_list = Vec::new();

    serialize_meta(&mut writer, "ExplicitAutoJoints", "true")?;

    for referent in refs {
        serialize_item(&mut writer, &mut state, dom, *referent, &mut prop_list)?;
    }

    log::trace!("serializing {} sharedstrings", state.shared_strings.len());
    if !state.shared_strings.is_empty() {
        writer.start_element("SharedStrings").finalize()?;

        let mut hash_container = String::with_capacity(base64::encoded_len(16, true).unwrap());
        for sstr in state.shared_strings {
            // For legacy reasons, we have to truncate the hash for shared strings
            base64::prelude::BASE64_STANDARD
                .encode_string(&sstr.hash().as_bytes()[0..16], &mut hash_container);
            writer
                .start_element("SharedString")
                .attribute("md5", &hash_container)
                .finalize()?;
            writer.write_base64(sstr.data())?;
            writer.end_element("SharedString")?;
        }

        writer.end_element("SharedStrings")?;
    }

    writer.end_element("roblox")?;

    Ok(())
}

pub fn serialize_meta<W: io::Write>(
    writer: &mut XmlWriter<W>,
    name: &str,
    value: &str,
) -> Result<(), EncodeError> {
    log::debug!("Writing metadata {name} = {value}");
    writer
        .start_element("Meta")
        .attribute("name", name)
        .finalize()?;
    writer.write_text(value)?;
    writer.end_element("Meta")?;

    Ok(())
}

fn serialize_item<'db, W: io::Write>(
    writer: &mut XmlWriter<W>,
    state: &mut EncodeState<'db>,
    dom: &'db WeakDom,
    referent: Ref,
    prop_list: &mut Vec<(Cow<'db, str>, Cow<'db, Variant>)>,
) -> Result<(), EncodeError> {
    let instance = dom
        .get_by_ref(referent)
        .ok_or_else(|| ErrorKind::RefNotInDom(referent).err())?;
    let class_name = instance.class.as_str();

    if state.config.strict_class_names {
        if let Some(database) = state.config.database {
            if !database.classes.contains_key(class_name) {
                log::error!("Unknown class: {}", class_name);
                return Err(ErrorKind::UnknownClass(class_name.to_owned()).err());
            }
        } else {
            return Err(ErrorKind::StrictWithoutDatabase("class names").err());
        }
    }
    log::debug!("Attempting to serialize Instance {}", referent);

    if state.config.strict_data_types || state.config.strict_property_names {
        if let Some(database) = state.config.database {
            for (canon_name, canon_value) in &instance.properties {
                if let Some(descriptor) =
                    find_serialized_property_descriptor(database, class_name, canon_name)
                {
                    if state.config.strict_data_types {
                        let serial_type = match descriptor.data_type {
                            DataType::Value(ty) => ty,
                            DataType::Enum(_) => VariantType::Enum,
                            _ => unimplemented!(),
                        };
                        if serial_type == canon_value.ty() {
                            log::debug!(
                                "Translated {class_name}.{canon_name} to {}",
                                descriptor.name
                            );
                            prop_list.push((descriptor.name.clone(), Cow::Borrowed(canon_value)))
                        } else {
                            log::debug!(
                                "Converting {class_name}.{canon_name} from {:?} to {serial_type:?}",
                                canon_value.ty()
                            );
                            match conversions::convert(canon_value, serial_type) {
                                Ok(serial_value) => prop_list
                                    .push((descriptor.name.clone(), Cow::Owned(serial_value))),
                                Err(error) => {
                                    log::error!("Could not convert {canon_value:?} to {serial_type:?} because {error}");
                                    // return Err(ErrorKind::ConversionFail {
                                    //     class: instance.class.clone(),
                                    //     name: canon_name.clone(),
                                    //     from: canon_value.ty(),
                                    //     to: serial_type,
                                    //     error,
                                    // }
                                    // .err())
                                }
                            }
                        }
                    }
                } else {
                    // this could indicate there's no rewrite needed
                    // or that we don't know about it so unfortunately
                    // we can't assume anything :(
                    prop_list.push((Cow::Borrowed(canon_name), Cow::Borrowed(canon_value)))
                }
            }
        }
    } else {
        for (canon_name, canon_value) in &instance.properties {
            prop_list.push((Cow::Borrowed(canon_name), Cow::Borrowed(canon_value)))
        }
    }

    prop_list.sort_unstable_by_key(|(name, _)| name.clone());

    writer
        .start_element("Item")
        .attribute("class", &instance.class)
        .attribute("referent", state.map_or_set_ref(&referent))
        .finalize()?;

    log::debug!("Attempting to serialize {} properties", prop_list.len());
    writer.start_element("Properties").finalize()?;

    // We have to clone the name of the instance, unfortunately.
    // The other option is taking ownership of the Dom which isn't an option.
    data_types::attempt_serialization(writer, "Name", &Variant::String(instance.name.clone()))?;

    for (prop_name, prop_value) in prop_list.iter() {
        log::trace!("trying to serialize {prop_name}");
        match prop_value {
            Cow::Borrowed(Variant::Ref(referent)) => {
                if referent.is_some() {
                    if log::log_enabled!(log::Level::Trace) {
                        log::trace!(
                            "referent property {prop_name} = {referent}, setting to {}",
                            state.map_or_set_ref(referent)
                        );
                    }
                    data_types::serialize_ref(writer, prop_name, state.map_or_set_ref(referent))?
                } else {
                    log::trace!("referent property {prop_name} = null");
                    data_types::serialize_ref(writer, prop_name, "null")?
                }
            }
            Cow::Borrowed(Variant::SharedString(sstr)) => {
                log::trace!(
                    "sstr property {prop_name} {:?} ({} bytes)",
                    sstr.hash(),
                    sstr.data().len()
                );
                // TODO account for potential collisions here
                // We only use the first 16 bytes of the hash for SSTR,
                // which means it's not impossible we could have a collision
                // one day.
                state.shared_strings.insert(sstr.clone());
                data_types::serialize_shared_string(writer, prop_name, sstr.hash().as_bytes())?
            }
            _ => data_types::attempt_serialization(writer, prop_name, prop_value)?,
        }
    }
    writer.end_element("Properties")?;

    writer.end_element("Item")?;

    prop_list.clear();

    log::trace!("Serializing children of Instance {}", instance.referent());
    for child_ref in instance.children() {
        serialize_item(writer, state, dom, *child_ref, prop_list)?;
    }

    Ok(())
}

struct EncodeState<'db> {
    shared_strings: HashSet<SharedString>,
    ref_map: HashMap<Ref, usize>,
    ref_strings: Vec<String>,
    next_ref: usize,
    config: &'db Config<'db>,
}

impl<'db> EncodeState<'db> {
    fn map_or_set_ref(&mut self, referent: &Ref) -> &str {
        match self.ref_map.get(referent) {
            Some(index) => &self.ref_strings[*index],
            None => {
                let n = self.next_ref;
                let str = n.to_string();
                self.next_ref += 1;
                self.ref_map.insert(*referent, n);
                self.ref_strings.push(str);
                &self.ref_strings[n]
            }
        }
    }

    fn map_ref(&self, referent: Ref) -> Option<&str> {
        self.ref_map
            .get(&referent)
            .map(|i| self.ref_strings[*i].as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbx_dom_weak::DomViewer;

    #[test]
    fn dom_migration() {
        let _ = env_logger::try_init();
        let document = r#"
        <roblox version="4">
            <External>TestExternal</External>
            <Meta name="TestMetadata">TestValue</Meta>
            <Item class="Part" referent="foo">
                <Properties>
                    <string name="Name">Hello, world!</string>
                    <int name = "Color">197</int>
                    <Vector3 name="Position">
                        <X>10</X>
                        <Y>20</Y>
                        <Z>30</Z>
                    </Vector3>
                </Properties>
            </Item>
        </roblox>
    "#;
        let dom = crate::from_str(
            document,
            Config::with_database(rbx_reflection_database::get()),
        )
        .map_err(|e| panic!("could not decode: {}", e))
        .unwrap();
        let mut out = Vec::new();
        if let Err(error) = serialize_dom(
            &mut out,
            &dom,
            &Config::with_database(rbx_reflection_database::get()),
        ) {
            panic!("{}", error);
        }

        println!("{}", String::from_utf8(out).unwrap());
    }

    #[test]
    fn dom_test() {
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
                </Properties>
                <Item class="TestClass2" referent="TestReferent2">
                    <Properties>
                        <Ref name = "RefTest">TestReferent</Ref>
                        <SharedString name="TestSharedString">Test Shared String Key</SharedString>
                        <int name = "TestInt1">10</int>
                        <int name = "TestInt2">-10</int>
                        <int64 name = "TestInt64_1">20</int64>
                        <int64 name = "TestInt64_2">-20</int64>
                        <ProtectedString name = "Test"><![CDATA[   Protected String   ]]></ProtectedString>
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
                    </Properties>
                </Item>
            </Item>
            <SharedStrings>
                <SharedString md5="Test Shared String Key">Q1NHSzg1MTYxZjdlOWNmZjMyNTlhNmU1NmE2NGJjZmNjMzJh</SharedString>
            </SharedStrings>
        </roblox>
    "#;
        let dom = crate::from_str_default(document).unwrap();
        let mut out = Vec::new();
        if let Err(error) = serialize_dom(&mut out, &dom, &Config::new()) {
            panic!("{}", error);
        }

        println!("{}", String::from_utf8(out).unwrap());
    }

    #[test]
    fn dom_round_trip() {
        #![allow(unused_must_use)]
        env_logger::try_init();
        let file = std::fs::File::open("benches/crossroads.rbxlx").unwrap();

        let dom = crate::from_reader_default(file).unwrap();
        let mut out: Vec<u8> = Vec::new();
        match serialize_dom(&mut out, &dom, &Config::default()) {
            Err(err) => panic!("{}", err),
            Ok(_) => {
                let dom2 = crate::from_reader_default(out.as_slice()).unwrap();
                // insta::assert_yaml_snapshot!("serialize crossroads", DomViewer::new().view(&dom2))
            }
        }
    }
}