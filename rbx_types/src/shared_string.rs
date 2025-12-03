use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    fmt,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, Weak},
};

use blake3::Hash as Blake3Hash;

lazy_static::lazy_static! {
    static ref STRING_CACHE: Arc<Mutex<HashMap<Blake3Hash, Weak<[u8]>>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

/// A version of `BinaryString` used for data that's commonly repeated.
/// `rbx_types` automatically deduplicates data as it's loaded into
/// `SharedString` values.
#[derive(Debug, Clone)]
pub struct SharedString {
    data: Arc<[u8]>,
    hash: Blake3Hash,
}

impl SharedString {
    /// Construct a SharedString from an owned buffer of data.
    pub fn new(data: impl AsRef<[u8]>) -> SharedString {
        let data = data.as_ref();
        let hash = blake3::hash(data);

        let data = {
            let mut cache = STRING_CACHE.lock().unwrap();

            match cache.entry(hash) {
                Entry::Occupied(mut occupied) => match occupied.get().upgrade() {
                    Some(handle) => {
                        // An existing entry that we can reference
                        handle
                    }
                    None => {
                        // An existing entry that's starting to be evicted from
                        // the Drop of another SharedString instance.
                        //
                        // We can replace this handle with our copy of the data,
                        // but re-use this spot in the map.

                        let handle = Arc::from(data);
                        occupied.insert(Arc::downgrade(&handle));
                        handle
                    }
                },
                Entry::Vacant(vacant) => {
                    // This string didn't exist before, so we'll populate it.

                    let handle = Arc::from(data);
                    vacant.insert(Arc::downgrade(&handle));
                    handle
                }
            }
        };

        SharedString { data, hash }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    #[inline]
    pub fn hash(&self) -> SharedStringHash {
        SharedStringHash(self.hash)
    }
}

impl Hash for SharedString {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.hash.as_bytes());
    }
}

impl PartialEq for SharedString {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for SharedString {}

impl AsRef<[u8]> for SharedString {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl Drop for SharedString {
    fn drop(&mut self) {
        // If the reference we're about to drop is the very last reference to
        // the buffer, we'll be able to unwrap it and remove it from the
        // SharedString cache.

        // If we have the only strong reference, then
        // we can be sure this is the only copy.  The inner Arc
        // is never exposed so there will not be any downgraded
        // weak copies anywhere other than the string cache.
        // Therefore, it is not possible for another thread
        // to change the strong count in between this check
        // and `self.data` being dropped at the end of the scope.
        if Arc::strong_count(&self.data) != 1 {
            return;
        };

        let Ok(mut cache) = STRING_CACHE.lock() else {
            // If the lock is poisoned, we should just leave it
            // alone so that we don't accidentally double-panic.
            return;
        };

        cache.remove(&self.hash);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SharedStringHash(Blake3Hash);

impl SharedStringHash {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes().as_ref()
    }
}

impl Ord for SharedStringHash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialOrd for SharedStringHash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for SharedStringHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.to_hex().as_str())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod serde_impl {
    use super::*;

    use serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl Serialize for SharedString {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                let encoded = base64::encode(self.data());

                serializer.serialize_str(&encoded)
            } else {
                self.data().serialize(serializer)
            }
        }
    }

    struct SharedStringVisitor;

    impl Visitor<'_> for SharedStringVisitor {
        type Value = SharedString;

        fn expecting(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(out, "a SharedString value")
        }

        fn visit_str<E: Error>(self, str: &str) -> Result<Self::Value, E> {
            let buffer = base64::decode(str).map_err(E::custom)?;
            Ok(SharedString::new(buffer))
        }
    }

    impl<'de> Deserialize<'de> for SharedString {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(SharedStringVisitor)
            } else {
                // For compatibility reasons, we use `Vec<u8>`'s implementation
                // of deserialize.
                let buffer = <Vec<u8>>::deserialize(deserializer)?;
                Ok(SharedString::new(buffer))
            }
        }
    }
}

/// A type used by Roblox for certain networking and memory guarantees.
///
/// This type is functionally identical to a `SharedString` when serialized.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct NetAssetRef(SharedString);

impl NetAssetRef {
    /// Construct a `NetAssetRef` from an owned buffer of data.
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        Self(SharedString::new(data))
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        self.0.data()
    }

    pub fn hash(&self) -> NetAssetRefHash {
        NetAssetRefHash(self.0.hash)
    }
}

impl AsRef<[u8]> for NetAssetRef {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl AsRef<SharedString> for NetAssetRef {
    fn as_ref(&self) -> &SharedString {
        &self.0
    }
}

impl From<SharedString> for NetAssetRef {
    fn from(value: SharedString) -> Self {
        Self(value)
    }
}

impl From<NetAssetRef> for SharedString {
    fn from(value: NetAssetRef) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetAssetRefHash(Blake3Hash);

impl NetAssetRefHash {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes().as_ref()
    }
}

impl Ord for NetAssetRefHash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialOrd for NetAssetRefHash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for NetAssetRefHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.to_hex().as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_twice() {
        let handle_1 = SharedString::new(&[5, 4, 3]);
        let handle_2 = SharedString::new(&[5, 4, 3]);

        let data_1 = &handle_1.data;
        let data_2 = &handle_2.data;

        assert!(Arc::ptr_eq(data_1, data_2));
    }

    #[test]
    fn drop() {
        {
            let _x = SharedString::new(&[2]);
        }

        {
            let _y = SharedString::new(&[5, 6, 7, 1]);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_human() {
        let sstr = SharedString::new(b"a test string");
        let serialized = serde_json::to_string(&sstr).unwrap();

        assert_eq!(serialized, r#""YSB0ZXN0IHN0cmluZw==""#);

        let deserialized: SharedString = serde_json::from_str(&serialized).unwrap();

        assert_eq!(sstr, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_non_human() {
        use std::{io::Write, mem};

        let sstr = SharedString::new(b"a test string");
        let data = sstr.data();
        let serialized = bincode::serialize(&sstr).unwrap();

        // Write the length of the string as little-endian u64 followed by the
        // bytes of the string. This is analoglous to how bincode does.
        let mut expected = Vec::with_capacity(mem::size_of::<u64>() + data.len());
        expected
            .write_all(&(data.len() as u64).to_le_bytes())
            .unwrap();
        expected.write_all(data).unwrap();

        assert_eq!(serialized, expected);

        let deserialized: SharedString = bincode::deserialize(&serialized).unwrap();

        assert_eq!(sstr, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn netassetref_serde() {
        let sstr = SharedString::new(&[13, 37]);
        let net = NetAssetRef::new(&[13, 37]);

        let ser_sstr_1 = serde_json::to_string(&sstr).unwrap();
        let ser_net_1 = serde_json::to_string(&net).unwrap();

        assert_eq!(ser_sstr_1, ser_net_1);

        let de_net_1: NetAssetRef = serde_json::from_str(&ser_net_1).unwrap();

        assert_eq!(net, de_net_1);

        let ser_sstr_2 = bincode::serialize(&sstr).unwrap();
        let ser_net_2 = bincode::serialize(&net).unwrap();

        assert_eq!(ser_sstr_2, ser_net_2);

        let de_net_2: NetAssetRef = bincode::deserialize(&ser_net_2).unwrap();

        assert_eq!(net, de_net_2);
    }
}
