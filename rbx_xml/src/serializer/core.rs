use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    io,
};

use base64::Engine;

use rbx_dom_weak::{
    types::{Ref, SharedString, SharedStringHash, Variant, VariantType},
    WeakDom,
};
use rbx_reflection::DataType;

use crate::property_descriptor::find_serialized_property_descriptor;

use super::{
    conversions, data_types,
    error::{EncodeError, ErrorKind},
    EncodeOptions,
};

use super::writer::XmlWriter;

pub fn serialize_dom<W: io::Write>(
    output: W,
    dom: &WeakDom,
    options: EncodeOptions,
) -> Result<(), EncodeError> {
    serialize_refs(output, dom, dom.root().children(), options)
}

pub fn serialize_refs<W: io::Write>(
    output: W,
    dom: &WeakDom,
    refs: &[Ref],
    options: EncodeOptions,
) -> Result<(), EncodeError> {
    let mut writer = XmlWriter::new(output, Some((b' ', 2)));

    writer
        .start_element("roblox")
        .attribute("version", "4")
        .finalize()?;

    // TODO add a way to preallocate this for the number of Instances in a file
    let mut state = EncodeState {
        shared_strings: BTreeMap::new(),
        ref_map: HashMap::new(),
        ref_strings: Vec::new(),
        next_ref: 0,
        options,
    };
    let mut prop_list = Vec::new();

    //TODO determine if we even need to do this anymore
    // serialize_meta(&mut writer, "ExplicitAutoJoints", "true")?;

    for referent in refs {
        serialize_item(&mut writer, &mut state, dom, *referent, &mut prop_list)?;
    }

    log::trace!("serializing {} sharedstrings", state.shared_strings.len());
    if !state.shared_strings.is_empty() {
        writer.start_element("SharedStrings").finalize()?;

        let mut hash_container = String::with_capacity(base64::encoded_len(16, true).unwrap());
        for (hash, sstr) in state.shared_strings {
            // For legacy reasons, we have to truncate the hash for shared strings
            base64::prelude::BASE64_STANDARD
                .encode_string(&hash.as_bytes()[0..16], &mut hash_container);
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

    // TODO check Instance class names?
    log::debug!("Attempting to serialize Instance {}", referent);

    if state.options.use_reflection() {
        let database = state.options.database.unwrap();
        for (canon_name, canon_value) in &instance.properties {
            log::trace!("visiting {canon_name} with the reflection database");
            if let Some(descriptor) =
                find_serialized_property_descriptor(database, class_name, canon_name)
            {
                let serial_name = if canon_name.as_str() != descriptor.name {
                    log::trace!("renaming {canon_name} to {}", descriptor.name);
                    descriptor.name.clone()
                } else {
                    Cow::Borrowed(canon_name.as_str())
                };
                let canon_type = canon_value.ty();
                let serial_type = match descriptor.data_type {
                    DataType::Value(ty) => ty,
                    DataType::Enum(_) => VariantType::Enum,
                    _ => unimplemented!(),
                };
                if canon_type != serial_type {
                    log::trace!(
                        "converting {class_name}.{serial_name} from {canon_type:?} to {serial_type:?}"
                    );
                    match conversions::convert(Cow::Borrowed(canon_value), serial_type) {
                        Ok(serial_value) => prop_list.push((serial_name, serial_value)),
                        Err(error) => {
                            return Err(ErrorKind::ConversionFail {
                                class: class_name.into(),
                                name: serial_name.to_string(),
                                from: canon_type,
                                to: serial_type,
                                error,
                            }
                            .err())
                        }
                    }
                } else {
                    prop_list.push((serial_name, Cow::Borrowed(canon_value)))
                }
            } else if !state.options.ignore_unknown() {
                return Err(
                    ErrorKind::UnknownProperty(class_name.into(), canon_name.clone()).err(),
                );
            } else {
                log::trace!("unknown property {canon_name}");
                prop_list.push((
                    Cow::Borrowed(canon_name.as_str()),
                    Cow::Borrowed(canon_value),
                ));
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
                state.shared_strings.insert(sstr.hash(), sstr.clone());
                data_types::serialize_shared_string(writer, prop_name, sstr.hash().as_bytes())?
            }
            _ => {
                if data_types::is_known_type(prop_value) {
                    data_types::attempt_serialization(writer, prop_name, prop_value)?
                } else if state.options.unknown_type_err {
                    return Err(
                        ErrorKind::UnknownType(prop_name.to_string(), prop_value.ty()).err(),
                    );
                }
            }
        }
    }
    writer.end_element("Properties")?;

    prop_list.clear();

    log::trace!("Serializing children of Instance {}", instance.referent());
    for child_ref in instance.children() {
        serialize_item(writer, state, dom, *child_ref, prop_list)?;
    }

    writer.end_element("Item")?;

    Ok(())
}

struct EncodeState<'db> {
    shared_strings: BTreeMap<SharedStringHash, SharedString>,
    ref_map: HashMap<Ref, usize>,
    ref_strings: Vec<String>,
    next_ref: usize,
    options: EncodeOptions<'db>,
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
        let dom = crate::from_str_default(document)
            .map_err(|e| panic!("could not decode: {}", e))
            .unwrap();
        let mut out = Vec::new();
        if let Err(error) = serialize_dom(
            &mut out,
            &dom,
            EncodeOptions::new().database(rbx_reflection_database::get()),
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
        if let Err(error) = serialize_dom(
            &mut out,
            &dom,
            EncodeOptions::new().database(rbx_reflection_database::get()),
        ) {
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
        // insta::assert_yaml_snapshot!("deserialize crossroads", DomViewer::new().view(&dom));
        let mut out: Vec<u8> = Vec::new();
        match serialize_dom(
            &mut out,
            &dom,
            EncodeOptions::new().database(rbx_reflection_database::get()),
        ) {
            Err(err) => panic!("{}", err),
            Ok(_) => {
                std::fs::write("crossroads_ser.rbxlx", &out).unwrap();
                let dom2 = crate::from_reader_default(out.as_slice()).unwrap();
                // insta::assert_yaml_snapshot!("serialize crossroads", DomViewer::new().view(&dom2))
            }
        }
    }
}
