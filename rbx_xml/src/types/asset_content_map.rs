//! `AssetContentMap` values are stored as a JSON array of entries:
//!
//! ```xml
//! <AssetContentMap name="VoxelGridAssetContentMap">[{"key":"L3;0,0,0;N4","data":"VlhGRw=="}]</AssetContentMap>
//! ```
//!
//! `data` is base64 and may contain line breaks. Roblox has no XML form for
//! a bundle asset ID, so maps that only refer to a bundle cannot be written.

use std::{
    borrow::Cow,
    io::{Read, Write},
};

use base64::display::Base64Display;
use rbx_dom_weak::types::AssetContentMap;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError, EncodeErrorKind},
    serializer_core::XmlEventWriter,
};

/// One element of the JSON array, as read from a file.
#[derive(Deserialize)]
struct EntryIn<'a> {
    #[serde(borrow)]
    key: Cow<'a, str>,
    #[serde(borrow)]
    data: Cow<'a, str>,
}

/// One element of the JSON array, as written. Fields are declared in the
/// order Studio writes them.
#[derive(Serialize)]
struct EntryOut<'a> {
    key: &'a str,
    #[serde(serialize_with = "serialize_base64")]
    data: &'a [u8],
}

fn serialize_base64<S: Serializer>(data: &&[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&Base64Display::with_config(data, base64::STANDARD))
}

struct EntriesOut<'a>(&'a AssetContentMap);

impl Serialize for EntriesOut<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.0.iter().map(|(key, data)| EntryOut {
            key,
            data: data.as_ref(),
        }))
    }
}

impl XmlType for AssetContentMap {
    const XML_TAG_NAME: &'static str = "AssetContentMap";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        if self.bundle_asset_id().is_some() {
            return Err(writer.error(EncodeErrorKind::UnsupportedAssetContentMapBundle));
        }

        let json = serde_json::to_string(&EntriesOut(self)).unwrap();
        writer.write_string(&json)?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let contents = reader.read_characters()?;

        // Studio writes `[]` for an empty map, but tolerate an empty element.
        if contents.trim().is_empty() {
            return Ok(AssetContentMap::new());
        }

        let invalid = |explain| reader.error(DecodeErrorKind::InvalidContent(explain));

        let entries: Vec<EntryIn> = serde_json::from_str(&contents).map_err(|_| {
            invalid("AssetContentMap must be a JSON array of objects with string 'key' and 'data' fields")
        })?;

        let mut map = AssetContentMap::new();
        for EntryIn { key, data } in entries {
            // Studio wraps long base64 values with line feeds.
            let data = if data.contains(char::is_whitespace) {
                let stripped: String = data.chars().filter(|c| !c.is_whitespace()).collect();
                base64::decode(stripped)
            } else {
                base64::decode(data.as_bytes())
            }
            .map_err(|err| reader.error(err))?;

            // Roblox rejects the whole value when a key repeats.
            if map.insert(key.into_owned(), data.into()).is_some() {
                return Err(invalid("AssetContentMap has a duplicate key"));
            }
        }

        Ok(map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rbx_dom_weak::types::BinaryString;

    use crate::test_util;

    fn sample() -> AssetContentMap {
        let mut map = AssetContentMap::new();
        map.insert("L3;0,0,0;N4".to_owned(), b"VXFG\x00\xff".to_vec().into());
        map.insert("L7;0,0,0;N4".to_owned(), BinaryString::new());
        map
    }

    #[test]
    fn round_trip_empty() {
        test_util::test_xml_round_trip(&AssetContentMap::new());
    }

    #[test]
    fn round_trip_entries() {
        test_util::test_xml_round_trip(&sample());
    }

    #[test]
    fn serialize_empty() {
        test_util::test_xml_serialize(
            r#"<AssetContentMap name="foo">[]</AssetContentMap>"#,
            &AssetContentMap::new(),
        );
    }

    #[test]
    fn deserialize_empty_element() {
        test_util::test_xml_deserialize(
            r#"<AssetContentMap name="foo"></AssetContentMap>"#,
            &AssetContentMap::new(),
        );
    }

    #[test]
    fn deserialize_line_wrapped_base64() {
        let mut expected = AssetContentMap::new();
        expected.insert("k".to_owned(), b"Hello, world!".to_vec().into());

        test_util::test_xml_deserialize(
            r#"<AssetContentMap name="foo">[{"key":"k","data":"SGVsbG8s\nIHdvcmxkIQ=="}]</AssetContentMap>"#,
            &expected,
        );
    }

    #[test]
    fn deserialize_ignores_order() {
        test_util::test_xml_deserialize(
            r#"<AssetContentMap name="foo">[{"key":"L7;0,0,0;N4","data":""},{"key":"L3;0,0,0;N4","data":"VlhGRwD/"}]</AssetContentMap>"#,
            &sample(),
        );
    }

    fn decode(source: &str) -> Result<AssetContentMap, DecodeError> {
        let mut reader = XmlEventReader::from_source(source.as_bytes());
        reader.next().unwrap().unwrap();
        AssetContentMap::read_outer_xml(&mut reader)
    }

    #[test]
    fn reject_duplicate_key() {
        let result = decode(
            r#"<AssetContentMap name="foo">[{"key":"k","data":""},{"key":"k","data":""}]</AssetContentMap>"#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn reject_non_array() {
        let result = decode(r#"<AssetContentMap name="foo">{}</AssetContentMap>"#);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_key_before_data() {
        let mut map = AssetContentMap::new();
        map.insert("k".to_owned(), b"Rojo is cool!".to_vec().into());

        test_util::test_xml_serialize(
            r#"<AssetContentMap name="foo">[{"key":"k","data":"Um9qbyBpcyBjb29sIQ=="}]</AssetContentMap>"#,
            &map,
        );
    }

    #[test]
    fn reject_missing_data() {
        let result = decode(r#"<AssetContentMap name="foo">[{"key":"k"}]</AssetContentMap>"#);
        assert!(result.is_err());
    }

    #[test]
    fn bundle_is_unsupported() {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(1));

        let mut buffer = Vec::new();
        let mut writer = XmlEventWriter::from_output(&mut buffer);
        assert!(map.write_outer_xml("foo", &mut writer).is_err());
    }
}
