//! Attributes reading and writing is based on the spec that Anaminus put
//! together:
//! https://github.com/RobloxAPI/rbxattr/blob/06116439a68931d9d591d11ffff77ff982c9947d/spec.md

mod error;
mod reader;
mod type_id;
mod writer;

use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    io::{Read, Write},
    iter::FromIterator,
};

use crate::{Error, Variant};

use self::reader::read_attributes;
use self::writer::write_attributes;

pub(crate) use self::error::AttributeError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Attributes {
    data: HashMap<String, Variant>,
}

impl Attributes {
    /// Creates an empty `Attributes` struct
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads from a serialized attributes string, and produces a new `Attributes` from it.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        Ok(Attributes {
            data: read_attributes(reader)?,
        })
    }

    /// Writes the attributes as a serialized string to the writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        write_attributes(&self.data, &mut writer).map_err(Into::into)
    }

    /// Get the attribute with the following key.
    pub fn get<K: Borrow<str>>(&self, key: K) -> Option<&Variant> {
        self.data.get(key.borrow())
    }

    /// Inserts an attribute with the given key and value.
    /// Will return the attribute that used to be there if one existed.
    pub fn insert(&mut self, key: String, value: Variant) -> Option<Variant> {
        self.data.insert(key, value)
    }

    /// Removes an attribute with the given key.
    /// Will return the value that was there if one existed.
    pub fn remove<K: Hash + Eq + Borrow<str>>(&mut self, key: K) -> Option<Variant> {
        self.data.remove(key.borrow())
    }

    /// Returns an iterator of borrowed attributes.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Variant)> {
        self.data.iter()
    }
}

impl IntoIterator for Attributes {
    type IntoIter = AttributesIntoIter;
    type Item = (String, Variant);

    fn into_iter(self) -> Self::IntoIter {
        AttributesIntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl FromIterator<(String, Variant)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (String, Variant)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

/// An owning iterator over the entries of an `Attributes`.
/// This is created by [`Attributes::into_iter`].
pub struct AttributesIntoIter {
    iter: std::collections::hash_map::IntoIter<String, Variant>,
}

impl Iterator for AttributesIntoIter {
    type Item = (String, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    // This is taken from rbx-test-files/models/attributes/xml.rbxmx, but with
    // the NaN and Infinity removed. This is pasted raw as to not create a
    // circular dependency in test
    // (rbx_types -> rbx_xml/rbx_binary -> rbx_types)
    const ATTRIBUTES_BASE64: &str = "\
        DQAAAAcAAABCb29sZWFuAwEKAAAAQnJpY2tDb2xvcg7sAwAABgAAAENvbG9yMw+joiI/AAAA\
        AAAAgD8NAAAAQ29sb3JTZXF1ZW5jZRkDAAAAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAAAA\
        PwAAAAAAAIA/AAAAAAAAAAAAAIA/AAAAAAAAAAAAAIA/BgAAAE51bWJlcgYAAAAAgBzIQAsA\
        AABOdW1iZXJSYW5nZRsAAKBAAAAgQQ4AAABOdW1iZXJTZXF1ZW5jZRcDAAAAAAAAAAAAAAAA\
        AIA/AAAAAAAAAD8AAAAAAAAAAAAAgD8AAIA/BAAAAFJlY3QcAACAPwAAAEAAAEBAAACAQAYA\
        AABTdHJpbmcCDQAAAEhlbGxvLCB3b3JsZCEEAAAAVURpbQkAAAA/ZAAAAAUAAABVRGltMgoA\
        AAA/CgAAADMzMz8eAAAABwAAAFZlY3RvcjIQAAAgQQAASEIHAAAAVmVjdG9yMxEAAIA/AAAA\
        QAAAQEA=";

    #[test]
    #[cfg(feature = "serde")]
    fn test_round_trip_attributes() {
        let attributes_value =
            base64::decode(ATTRIBUTES_BASE64).expect("bad base64 for attributes");

        let attributes = Attributes::from_reader(&attributes_value[..])
            .expect("couldn't deserialize attributes");

        let attributes_stable_order: BTreeMap<_, _> = attributes.clone().into_iter().collect();
        insta::assert_yaml_snapshot!(attributes_stable_order);

        let mut new_attribute_bytes = Vec::<u8>::new();
        attributes
            .to_writer(&mut new_attribute_bytes)
            .expect("couldn't write attributes to buffer");

        let new_attributes = Attributes::from_reader(new_attribute_bytes.as_slice())
            .expect("couldn't deserialize crate produced binary");

        let new_attributes_stable_order: BTreeMap<_, _> = new_attributes.into_iter().collect();

        assert_eq!(attributes_stable_order, new_attributes_stable_order);
    }

    #[test]
    fn test_attribute_removal() {
        let mut attributes = Attributes::new();
        attributes.insert("key".to_owned(), Variant::Bool(true));
        assert_eq!(attributes.remove("key"), Some(Variant::Bool(true)));
    }
}
