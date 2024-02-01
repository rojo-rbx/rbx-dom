//! Attributes reading and writing is based on the spec that Anaminus put
//! together:
//! https://github.com/RobloxAPI/rbxattr/blob/06116439a68931d9d591d11ffff77ff982c9947d/spec.md

mod error;
mod reader;
mod type_id;
mod writer;

use std::{
    borrow::Borrow,
    collections::{btree_map, BTreeMap},
    hash::Hash,
    io::{Read, Write},
    iter::FromIterator,
};

use crate::{Error, Variant};

use self::reader::read_attributes;
use self::writer::write_attributes;

pub(crate) use self::error::AttributeError;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Attributes {
    data: BTreeMap<String, Variant>,
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

    /// Inserts an attribute with the given key and value.
    /// Will overwrite the attribute that used to be there if one existed.
    pub fn with<K: Into<String>, V: Into<Variant>>(mut self, key: K, value: V) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Removes an attribute with the given key.
    /// Will return the value that was there if one existed.
    pub fn remove<K: Hash + Eq + Borrow<str>>(&mut self, key: K) -> Option<Variant> {
        self.data.remove(key.borrow())
    }

    /// Returns an iterator of borrowed attributes.
    #[inline]
    pub fn iter(&self) -> AttributesIter<'_> {
        AttributesIter {
            iter: self.data.iter(),
        }
    }

    /// Returns the number of attributes.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the struct contains no attributes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Extend<(String, Variant)> for Attributes {
    fn extend<T: IntoIterator<Item = (String, Variant)>>(&mut self, iter: T) {
        self.data.extend(iter)
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

impl<'a> IntoIterator for &'a Attributes {
    type IntoIter = AttributesIter<'a>;
    type Item = (&'a String, &'a Variant);

    fn into_iter(self) -> Self::IntoIter {
        AttributesIter {
            iter: self.data.iter(),
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
    iter: btree_map::IntoIter<String, Variant>,
}

impl Iterator for AttributesIntoIter {
    type Item = (String, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// A borrowed iterator over the entries of an `Attributes`.
/// This is created by [`Attributes::iter`].
pub struct AttributesIter<'a> {
    iter: btree_map::Iter<'a, String, Variant>,
}

impl<'a> Iterator for AttributesIter<'a> {
    type Item = (&'a String, &'a Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
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

        insta::assert_yaml_snapshot!(attributes);

        let mut new_attribute_bytes = Vec::<u8>::new();
        attributes
            .to_writer(&mut new_attribute_bytes)
            .expect("couldn't write attributes to buffer");

        let new_attributes = Attributes::from_reader(new_attribute_bytes.as_slice())
            .expect("couldn't deserialize crate produced binary");

        assert_eq!(attributes, new_attributes);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_encode_json() {
        use serde_json::{json, Value};

        fn assert_json(value: Attributes, expected: Value) {
            let encoded = serde_json::to_string(&value).unwrap();
            let decoded: Value = serde_json::from_str(&encoded).unwrap();

            assert_eq!(decoded, expected);
        }

        let empty = Attributes::new();
        assert_json(empty, json!({}));

        let number = Attributes::new().with("hello", 5.0f64);
        assert_json(
            number,
            json!({
                "hello": {
                    "Float64": 5.0
                }
            }),
        );
    }

    #[test]
    fn test_attribute_removal() {
        let mut attributes = Attributes::new();
        attributes.insert("key".to_owned(), Variant::Bool(true));
        assert_eq!(attributes.remove("key"), Some(Variant::Bool(true)));
    }
}
