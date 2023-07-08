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

    let mut state = EncodeState {
        shared_strings: BTreeMap::new(),
        ref_map: HashMap::with_capacity(refs.len()),
        ref_strings: Vec::with_capacity(refs.len()),
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
            hash_container.clear();
        }

        writer.end_element("SharedStrings")?;
    }

    writer.end_element("roblox")?;

    Ok(())
}

// TODO serialize metadata (see issue #3)

// pub fn serialize_meta<W: io::Write>(
//     writer: &mut XmlWriter<W>,
//     name: &str,
//     value: &str,
// ) -> Result<(), EncodeError> {
//     log::debug!("Writing metadata {name} = {value}");
//     writer
//         .start_element("Meta")
//         .attribute("name", name)
//         .finalize()?;
//     writer.write_text(value)?;
//     writer.end_element("Meta")?;

//     Ok(())
// }

fn serialize_item<'a, 'db, W: io::Write>(
    writer: &mut XmlWriter<W>,
    state: &mut EncodeState<'db>,
    dom: &'a WeakDom,
    referent: Ref,
    prop_list: &mut Vec<(&'a String, &'a Variant)>,
) -> Result<(), EncodeError> {
    let instance = dom
        .get_by_ref(referent)
        .ok_or_else(|| ErrorKind::RefNotInDom(referent).err())?;
    let class_name = instance.class.as_str();

    // TODO check Instance class names?
    log::debug!("Attempting to serialize Instance {}", referent);
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

    prop_list.extend(&instance.properties);
    prop_list.sort_unstable_by_key(|(name, _)| name.clone());

    for (name, value) in prop_list.drain(..) {
        let mut real_name = Cow::Borrowed(name.as_str());
        let mut real_value = Cow::Borrowed(value);

        if state.options.use_reflection() {
            let database = state.options.database.unwrap();
            log::trace!("visiting {name} with reflection database");
            if let Some(descriptor) =
                find_serialized_property_descriptor(database, class_name, name)
            {
                if name.as_str() != descriptor.name {
                    // TODO check if the new property name already exists
                    log::trace!("renaming {name} to {}", descriptor.name);
                    real_name = descriptor.name.clone();
                }
                let current_ty = value.ty();
                let real_ty = match descriptor.data_type {
                    DataType::Value(ty) => ty,
                    DataType::Enum(_) => VariantType::Enum,
                    _ => unimplemented!(),
                };
                if current_ty != real_ty {
                    log::trace!("converting {real_name} from {current_ty:?} to {real_ty:?}");
                    match conversions::convert(value, real_ty) {
                        Ok(new) => real_value = new,
                        Err(error) => {
                            return Err(ErrorKind::ConversionFail {
                                class: class_name.into(),
                                name: real_name.into(),
                                from: current_ty,
                                to: real_ty,
                                error,
                            }
                            .into())
                        }
                    }
                }
            } else if !state.options.ignore_unknown() {
                return Err(ErrorKind::UnknownProperty(class_name.into(), name.into()).err());
            }
        }
        log::trace!("attempting to serialize {real_name}");

        match real_value.as_ref() {
            Variant::Ref(referent) => {
                log::trace!("ref property");
                if referent.is_some() {
                    data_types::serialize_ref(writer, &real_name, state.map_or_set_ref(referent))?
                } else {
                    data_types::serialize_ref(writer, &real_name, "null")?
                }
            }
            Variant::SharedString(sstr) => {
                log::trace!(
                    "sstr property {:?} ({} bytes)",
                    sstr.hash(),
                    sstr.data().len()
                );

                // TODO account for potential collisions here
                // We only use the first 16 bytes of the hash for SSTR,
                // which means it's not impossible we could have a collision
                // one day.
                state.shared_strings.insert(sstr.hash(), sstr.clone());
                data_types::serialize_shared_string(writer, &real_name, sstr.hash())?
            }
            _ => {
                if data_types::is_known_type(&real_value) {
                    data_types::attempt_serialization(writer, &real_name, &real_value)?
                } else if state.options.unknown_type_err {
                    return Err(
                        ErrorKind::UnknownType(real_name.to_string(), real_value.ty()).err(),
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
    // Either gets a sequential integer string to replace a referent during
    // serialization or generates one.
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
}
