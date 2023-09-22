use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    fmt,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, Weak},
};

use blake3::Hash as Blake3Hash;

lazy_static::lazy_static! {
    static ref STRING_CACHE: Arc<Mutex<HashMap<Blake3Hash, Weak<Vec<u8>>>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

/// A version of `BinaryString` used for data that's commonly repeated.
/// `rbx_types` automatically deduplicates data as it's loaded into
/// `SharedString` values.
#[derive(Debug, Clone)]
pub struct SharedString {
    data: Option<Arc<Vec<u8>>>,
    hash: Blake3Hash,
}

impl SharedString {
    /// Construct a SharedString from an owned buffer of data.
    pub fn new(data: Vec<u8>) -> SharedString {
        let hash = blake3::hash(&data);

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

        SharedString {
            data: Some(data),
            hash,
        }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        self.data.as_ref().unwrap()
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
        if Arc::into_inner(self.data.take().unwrap()).is_some() {
            let mut cache = match STRING_CACHE.lock() {
                Ok(v) => v,
                Err(_) => {
                    // If the lock is poisoned, we should just leave it
                    // alone so that we don't accidentally double-panic.
                    return;
                }
            };

            cache.remove(&self.hash);
        }
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
pub(crate) mod variant_serialization {
    use super::*;

    use serde::de::Error as _;
    use serde::ser::Error as _;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(_value: &SharedString, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(S::Error::custom(
            "SharedString cannot be serialized as part of a Variant",
        ))
    }

    pub fn deserialize<'de, D>(_deserializer: D) -> Result<SharedString, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(D::Error::custom(
            "SharedString cannot be deserialized as part of a Variant",
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_twice() {
        let handle_1 = SharedString::new(vec![5, 4, 3]);
        let handle_2 = SharedString::new(vec![5, 4, 3]);

        let data_1 = handle_1.data.as_ref().unwrap();
        let data_2 = handle_2.data.as_ref().unwrap();

        assert!(Arc::ptr_eq(data_1, data_2));
    }

    #[test]
    fn drop() {
        {
            let _x = SharedString::new(vec![2]);
        }

        {
            let _y = SharedString::new(vec![5, 6, 7, 1]);
        }
    }
}
