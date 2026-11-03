use std::collections::{btree_map, BTreeMap};

use crate::BinaryString;

/// A map of named binary blobs, exposed in Roblox as the `AssetContentMap`
/// type.
///
/// Roblox uses this type to hold content that it may upload separately from
/// the place or model it belongs to. For example, `Terrain.VoxelGridAssetContentMap`
/// holds the voxel data of places that use Expanded Terrain; in those places,
/// `Terrain.SmoothGrid` only contains a small header, so dropping this value
/// deletes the terrain.
///
/// A map has two independent parts:
///
/// - **Entries**, which hold the content inline. Keys are unique and their
///   order is not significant to Roblox. The values are opaque to rbx-dom.
/// - An optional **bundle asset ID**, which refers to an uploaded asset that
///   holds the entries instead. Roblox downloads it when the value is loaded.
///   rbx-dom does not download it, and only preserves the reference.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct AssetContentMap {
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "deserialize_bundle_asset_id")
    )]
    bundle_asset_id: Option<i64>,
    entries: BTreeMap<String, BinaryString>,
}

/// Keeps the invariant that a bundle asset ID is never `Some(0)`, matching
/// [`AssetContentMap::set_bundle_asset_id`].
#[cfg(feature = "serde")]
fn deserialize_bundle_asset_id<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<i64>, D::Error> {
    let id: Option<i64> = serde::Deserialize::deserialize(deserializer)?;
    Ok(id.filter(|&id| id != 0))
}

impl AssetContentMap {
    /// Constructs an empty `AssetContentMap` with no bundle asset ID.
    #[inline]
    pub const fn new() -> Self {
        Self {
            bundle_asset_id: None,
            entries: BTreeMap::new(),
        }
    }

    /// Returns the ID of the uploaded asset that holds this map's content, if
    /// there is one.
    #[inline]
    pub fn bundle_asset_id(&self) -> Option<i64> {
        self.bundle_asset_id
    }

    /// Sets or clears the ID of the uploaded asset that holds this map's
    /// content. Roblox uses 0 to mean "no bundle", so `Some(0)` is stored as
    /// `None`.
    #[inline]
    pub fn set_bundle_asset_id(&mut self, bundle_asset_id: Option<i64>) {
        self.bundle_asset_id = bundle_asset_id.filter(|&id| id != 0);
    }

    /// Inserts an entry, returning the previous value for `key` if there was
    /// one.
    #[inline]
    pub fn insert(&mut self, key: String, value: BinaryString) -> Option<BinaryString> {
        self.entries.insert(key, value)
    }

    /// Returns the value stored for `key`, if any.
    #[inline]
    pub fn get(&self, key: &str) -> Option<&BinaryString> {
        self.entries.get(key)
    }

    /// Removes the entry for `key`, returning its value if there was one.
    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<BinaryString> {
        self.entries.remove(key)
    }

    /// Returns the number of inline entries. This does not include content
    /// that is only reachable through the bundle asset ID.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if there are no inline entries. A map with no entries
    /// may still refer to content through its bundle asset ID.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over the inline entries, sorted by key.
    #[inline]
    pub fn iter(&self) -> AssetContentMapIter<'_> {
        AssetContentMapIter {
            iter: self.entries.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a AssetContentMap {
    type Item = (&'a String, &'a BinaryString);
    type IntoIter = AssetContentMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A borrowed iterator over the inline entries of an `AssetContentMap`,
/// sorted by key. This is created by [`AssetContentMap::iter`].
pub struct AssetContentMapIter<'a> {
    iter: btree_map::Iter<'a, String, BinaryString>,
}

impl<'a> Iterator for AssetContentMapIter<'a> {
    type Item = (&'a String, &'a BinaryString);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_bundle_id_is_none() {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(0));
        assert_eq!(map.bundle_asset_id(), None);
    }

    #[test]
    fn insert_replaces() {
        let mut map = AssetContentMap::new();
        assert!(map.is_empty());

        assert_eq!(map.insert("a".to_owned(), b"1".to_vec().into()), None);
        assert_eq!(
            map.insert("a".to_owned(), b"2".to_vec().into()),
            Some(b"1".to_vec().into())
        );

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("a"), Some(&BinaryString::from(b"2".to_vec())));
    }

    #[test]
    fn iter_is_sorted() {
        let mut map = AssetContentMap::new();
        map.insert("b".to_owned(), BinaryString::new());
        map.insert("a".to_owned(), BinaryString::new());

        let keys: Vec<&str> = map.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(keys, ["a", "b"]);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    fn sample() -> AssetContentMap {
        let mut map = AssetContentMap::new();
        map.set_bundle_asset_id(Some(1234));
        map.insert("L3;0,0,0;N4".to_owned(), b"VXFG\x00\x01".to_vec().into());
        map
    }

    #[test]
    fn human() {
        let map = sample();

        let ser = serde_json::to_string(&map).unwrap();
        let de: AssetContentMap = serde_json::from_str(&ser).unwrap();

        assert_eq!(de, map);
    }

    #[test]
    fn non_human() {
        let map = sample();

        let ser = rmp_serde::to_vec(&map).unwrap();
        let de: AssetContentMap = rmp_serde::from_slice(&ser).unwrap();

        assert_eq!(de, map);
    }

    #[test]
    fn zero_bundle_id_is_none() {
        let de: AssetContentMap =
            serde_json::from_str(r#"{"bundleAssetId":0,"entries":{}}"#).unwrap();

        assert_eq!(de, AssetContentMap::new());
    }

    #[test]
    fn missing_bundle_id_is_none() {
        let de: AssetContentMap = serde_json::from_str(r#"{"entries":{}}"#).unwrap();

        assert_eq!(de, AssetContentMap::new());
    }
}
