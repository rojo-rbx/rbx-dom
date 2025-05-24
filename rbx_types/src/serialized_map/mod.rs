//! SerializedMap reading and writing is based on the spec that Anaminus put
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

use self::reader::read_serialized_map;
use self::writer::write_serialized_map;

pub(crate) use self::error::SerializedMapError;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct SerializedMap {
    data: BTreeMap<String, Variant>,
}

impl SerializedMap {
    /// Creates an empty `SerializedMap` struct
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads from a serialized map string, and produces a new `SerializedMap` from it.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        Ok(SerializedMap {
            data: read_serialized_map(reader)?,
        })
    }

    /// Writes the items as a serialized string to the writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        write_serialized_map(&self.data, &mut writer).map_err(Into::into)
    }

    /// Get the item with the following key.
    pub fn get<K: Borrow<str>>(&self, key: K) -> Option<&Variant> {
        self.data.get(key.borrow())
    }

    /// Inserts an item with the given key and value.
    /// Will return the item that used to be there if one existed.
    pub fn insert(&mut self, key: String, value: Variant) -> Option<Variant> {
        self.data.insert(key, value)
    }

    /// Inserts an item with the given key and value.
    /// Will overwrite the item that used to be there if one existed.
    pub fn with<K: Into<String>, V: Into<Variant>>(mut self, key: K, value: V) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Removes an item with the given key.
    /// Will return the value that was there if one existed.
    pub fn remove<K: Hash + Eq + Borrow<str>>(&mut self, key: K) -> Option<Variant> {
        self.data.remove(key.borrow())
    }

    /// Removes all items.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Returns an iterator of borrowed items.
    #[inline]
    pub fn iter(&self) -> SerializedMapIter<'_> {
        SerializedMapIter {
            iter: self.data.iter(),
        }
    }

    /// Removes all elements from the `SerializedMap`, returning them as an
    /// iterator. If the iterator is dropped before being fully consumed,
    /// it drops the remaining removed elements.
    #[inline]
    pub fn drain(&mut self) -> SerializedMapDrain<'_> {
        SerializedMapDrain { inner: self }
    }

    /// Returns the number of items.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the struct contains no items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Extend<(String, Variant)> for SerializedMap {
    fn extend<T: IntoIterator<Item = (String, Variant)>>(&mut self, iter: T) {
        self.data.extend(iter)
    }
}

impl IntoIterator for SerializedMap {
    type IntoIter = SerializedMapIterIntoIter;
    type Item = (String, Variant);

    fn into_iter(self) -> Self::IntoIter {
        SerializedMapIterIntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a SerializedMap {
    type IntoIter = SerializedMapIter<'a>;
    type Item = (&'a String, &'a Variant);

    fn into_iter(self) -> Self::IntoIter {
        SerializedMapIter {
            iter: self.data.iter(),
        }
    }
}

impl FromIterator<(String, Variant)> for SerializedMap {
    fn from_iter<T: IntoIterator<Item = (String, Variant)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

/// An owning iterator over the entries of an `SerializedMap`.
/// This is created by [`SerializedMap::into_iter`].
pub struct SerializedMapIterIntoIter {
    iter: btree_map::IntoIter<String, Variant>,
}

impl Iterator for SerializedMapIterIntoIter {
    type Item = (String, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// A borrowed iterator over the entries of an `SerializedMap`.
/// This is created by [`SerializedMap::iter`].
pub struct SerializedMapIter<'a> {
    iter: btree_map::Iter<'a, String, Variant>,
}

impl<'a> Iterator for SerializedMapIter<'a> {
    type Item = (&'a String, &'a Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// A draining iterator for `SerializedMap`.
/// This is created by [`SerializedMap::drain`].
///
/// If dropped before fully used, all remaining values will be dropped.
pub struct SerializedMapDrain<'a> {
    inner: &'a mut SerializedMap,
}

impl Iterator for SerializedMapDrain<'_> {
    type Item = (String, Variant);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.data.pop_first()
    }
}

impl Drop for SerializedMapDrain<'_> {
    fn drop(&mut self) {
        self.inner.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is taken from rbx-test-files/models/items/xml.rbxmx, but with
    // the NaN and Infinity removed. This is pasted raw as to not create a
    // circular dependency in test
    // (rbx_types -> rbx_xml/rbx_binary -> rbx_types)
    const SERIALIZED_MAP_BASE64: &str = "\
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
    fn test_round_trip_serialized_map() {
        let serialized_map_value =
            base64::decode(SERIALIZED_MAP_BASE64).expect("bad base64 for items");

        let items = SerializedMap::from_reader(&serialized_map_value[..])
            .expect("couldn't deserialize items");

        insta::assert_yaml_snapshot!(items);

        let mut new_item_bytes = Vec::<u8>::new();
        items
            .to_writer(&mut new_item_bytes)
            .expect("couldn't write items to buffer");

        let new_serialized_map = SerializedMap::from_reader(new_item_bytes.as_slice())
            .expect("couldn't deserialize crate produced binary");

        assert_eq!(items, new_serialized_map);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_encode_json() {
        use serde_json::{json, Value};

        fn assert_json(value: SerializedMap, expected: Value) {
            let encoded = serde_json::to_string(&value).unwrap();
            let decoded: Value = serde_json::from_str(&encoded).unwrap();

            assert_eq!(decoded, expected);
        }

        let empty = SerializedMap::new();
        assert_json(empty, json!({}));

        let number = SerializedMap::new().with("hello", 5.0f64);
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
    fn test_item_removal() {
        let mut items = SerializedMap::new();
        items.insert("key".to_owned(), Variant::Bool(true));
        assert_eq!(items.remove("key"), Some(Variant::Bool(true)));
    }

    #[test]
    fn item_drain() {
        let mut items = SerializedMap::new();
        items.extend([
            ("string".into(), "value".into()),
            ("float64".into(), 10.0_f64.into()),
            ("bool".into(), true.into()),
        ]);
        assert!(!items.is_empty());

        let mut map = BTreeMap::new();
        for (key, value) in items.drain() {
            map.insert(key, value);
        }

        assert_eq!(map.get("string"), Some(&Variant::String("value".into())));
        assert_eq!(map.get("float64"), Some(&Variant::Float64(10.0)));
        assert_eq!(map.get("bool"), Some(&Variant::Bool(true)));
        assert!(items.is_empty());
    }
}
