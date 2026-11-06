//! Binary encoding of the `AssetContentMap` type (type ID `0x23`).
//!
//! Unlike most types, values are not interleaved. Each instance's value is
//! written one after another:
//!
//! ```text
//! u8 flags
//! if flags & HAS_BUNDLE_ID:   string bundle_asset_id   (decimal digits)
//! if flags & HAS_BINARY_DATA: i64 count, then count * (string key, string data)
//! ```
//!
//! Strings are u32-length-prefixed and the count is little-endian. Roblox
//! writes exactly one of the two flags, but reads values with both set.

use std::io::{self, Read, Write};

use rbx_dom_weak::types::AssetContentMap;

use crate::{
    core::{RbxReadExt, RbxWriteExt},
    deserializer::InnerError,
};

const HAS_BUNDLE_ID: u8 = 1 << 0;
const HAS_BINARY_DATA: u8 = 1 << 1;

pub(crate) fn read_asset_content_map<R: Read>(
    reader: &mut R,
) -> Result<AssetContentMap, InnerError> {
    let flags = reader.read_u8()?;
    let mut map = AssetContentMap::new();

    if flags & HAS_BUNDLE_ID != 0 {
        let id_string = reader.read_string()?;
        let id: i64 = id_string
            .parse()
            .map_err(|_| InnerError::BadAssetContentMapBundleId(id_string))?;

        map.set_bundle_asset_id(Some(id));
    }

    if flags & HAS_BINARY_DATA != 0 {
        let mut count_bytes = [0; 8];
        reader.read_exact(&mut count_bytes)?;
        let count = i64::from_le_bytes(count_bytes);
        if count < 0 {
            return Err(InnerError::BadAssetContentMapEntryCount(count));
        }

        for _ in 0..count {
            let key = reader.read_string()?;
            let data = reader.read_binary_string()?;

            // Roblox keeps the first value when a key repeats.
            if map.get(&key).is_none() {
                map.insert(key, data.into());
            }
        }
    }

    Ok(map)
}

pub(crate) fn write_asset_content_map<W: Write>(
    writer: &mut W,
    map: &AssetContentMap,
) -> io::Result<()> {
    let bundle_asset_id = map.bundle_asset_id();

    // Entries are written whenever there are any, so that a value read with
    // both flags set round-trips. Empty maps without a bundle are written as
    // an explicit empty entry list, which is what Roblox does.
    let write_entries = !map.is_empty() || bundle_asset_id.is_none();

    let mut flags = 0;
    if bundle_asset_id.is_some() {
        flags |= HAS_BUNDLE_ID;
    }
    if write_entries {
        flags |= HAS_BINARY_DATA;
    }
    writer.write_u8(flags)?;

    if let Some(id) = bundle_asset_id {
        writer.write_string(&id.to_string())?;
    }

    if write_entries {
        writer.write_all(&(map.len() as i64).to_le_bytes())?;
        for (key, data) in map {
            writer.write_string(key)?;
            writer.write_binary_string(data.as_ref())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn round_trip(map: &AssetContentMap) -> Vec<u8> {
        let mut buffer = Vec::new();
        write_asset_content_map(&mut buffer, map).unwrap();

        let decoded = read_asset_content_map(&mut buffer.as_slice()).unwrap();
        assert_eq!(&decoded, map);

        buffer
    }

    /// The bytes Studio writes for an empty map.
    #[test]
    fn empty() {
        let buffer = round_trip(&AssetContentMap::new());
        assert_eq!(buffer, [0x02, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn entries() {
        let mut map = AssetContentMap::new();
        map.insert("L3;0,0,0;N4".to_owned(), b"VXFG\x00\xff".to_vec().into());
        map.insert("L7;0,0,0;N4".to_owned(), b"".to_vec().into());

        let buffer = round_trip(&map);

        let mut expected = vec![0x02, 2, 0, 0, 0, 0, 0, 0, 0];
        expected.extend_from_slice(&11u32.to_le_bytes());
        expected.extend_from_slice(b"L3;0,0,0;N4");
        expected.extend_from_slice(&6u32.to_le_bytes());
        expected.extend_from_slice(b"VXFG\x00\xff");
        expected.extend_from_slice(&11u32.to_le_bytes());
        expected.extend_from_slice(b"L7;0,0,0;N4");
        expected.extend_from_slice(&0u32.to_le_bytes());
        assert_eq!(buffer, expected);
    }

    #[test]
    fn bundle_only() {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(1234567890));

        let buffer = round_trip(&map);

        let mut expected = vec![0x01];
        expected.extend_from_slice(&10u32.to_le_bytes());
        expected.extend_from_slice(b"1234567890");
        assert_eq!(buffer, expected);
    }

    #[test]
    fn bundle_and_entries() {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(42));
        map.insert("key".to_owned(), b"data".to_vec().into());

        let buffer = round_trip(&map);
        assert_eq!(buffer[0], 0x03);
    }

    #[test]
    fn zero_bundle_id_is_none() {
        let mut buffer = vec![0x01];
        buffer.extend_from_slice(&1u32.to_le_bytes());
        buffer.extend_from_slice(b"0");

        let decoded = read_asset_content_map(&mut buffer.as_slice()).unwrap();
        assert_eq!(decoded, AssetContentMap::new());
    }

    #[test]
    fn duplicate_key_keeps_first() {
        let mut buffer = vec![0x02, 2, 0, 0, 0, 0, 0, 0, 0];
        for data in [b"first", b"other"] {
            buffer.extend_from_slice(&1u32.to_le_bytes());
            buffer.extend_from_slice(b"k");
            buffer.extend_from_slice(&5u32.to_le_bytes());
            buffer.extend_from_slice(data);
        }

        let decoded = read_asset_content_map(&mut buffer.as_slice()).unwrap();
        assert_eq!(decoded.len(), 1);
        let data: &[u8] = decoded.get("k").unwrap().as_ref();
        assert_eq!(data, b"first");
    }

    #[test]
    fn invalid_bundle_id() {
        let mut buffer = vec![0x01];
        buffer.extend_from_slice(&3u32.to_le_bytes());
        buffer.extend_from_slice(b"abc");

        assert!(matches!(
            read_asset_content_map(&mut buffer.as_slice()),
            Err(InnerError::BadAssetContentMapBundleId(_))
        ));
    }

    #[test]
    fn negative_count() {
        let mut buffer = vec![0x02];
        buffer.extend_from_slice(&(-1i64).to_le_bytes());

        assert!(matches!(
            read_asset_content_map(&mut buffer.as_slice()),
            Err(InnerError::BadAssetContentMapEntryCount(-1))
        ));
    }

    #[test]
    fn truncated() {
        let buffer = [0x02, 1, 0, 0, 0, 0, 0, 0, 0];

        assert!(matches!(
            read_asset_content_map(&mut buffer.as_slice()),
            Err(InnerError::Io { .. })
        ));
    }
}
