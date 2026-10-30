//! `AssetContentMap` values are stored as a JSON array of entries:
//!
//! ```xml
//! <AssetContentMap name="VoxelGridAssetContentMap">[{"key":"L3;0,0,0;N4","data":"VlhGRw=="}]</AssetContentMap>
//! ```
//!
//! `data` is base64 and may contain line breaks. Roblox has no XML form for
//! a bundle asset ID, so maps that only refer to a bundle cannot be written.

use std::io::{Read, Write};

use rbx_dom_weak::types::AssetContentMap;
use serde_json::Value;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError, EncodeErrorKind},
    serializer_core::XmlEventWriter,
};

impl XmlType for AssetContentMap {
    const XML_TAG_NAME: &'static str = "AssetContentMap";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        if self.bundle_asset_id().is_some() {
            return Err(writer.error(EncodeErrorKind::UnsupportedAssetContentMapBundle));
        }

        let entries: Vec<Value> = self
            .iter()
            .map(|(key, data)| {
                serde_json::json!({
                    "key": key,
                    "data": base64::encode(data),
                })
            })
            .collect();

        writer.write_string(&Value::Array(entries).to_string())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let contents = reader.read_characters()?;

        // Studio writes `[]` for an empty map, but tolerate an empty element.
        if contents.trim().is_empty() {
            return Ok(AssetContentMap::new());
        }

        let invalid = |explain| reader.error(DecodeErrorKind::InvalidContent(explain));

        let root: Value = serde_json::from_str(&contents)
            .map_err(|_| invalid("AssetContentMap is not valid JSON"))?;
        let entries = root
            .as_array()
            .ok_or_else(|| invalid("AssetContentMap must be a JSON array"))?;

        let mut map = AssetContentMap::new();
        for entry in entries {
            let key = entry
                .get("key")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("AssetContentMap entry has no string 'key'"))?;
            let encoded = entry
                .get("data")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("AssetContentMap entry has no string 'data'"))?;

            let encoded: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
            let data = base64::decode(encoded).map_err(|err| reader.error(err))?;

            // Roblox rejects the whole value when a key repeats.
            if map.insert(key.to_owned(), data.into()).is_some() {
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
    fn bundle_is_unsupported() {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(1));

        let mut buffer = Vec::new();
        let mut writer = XmlEventWriter::from_output(&mut buffer);
        assert!(map.write_outer_xml("foo", &mut writer).is_err());
    }
}
