use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex, Weak},
};

use blake3::Hash;

lazy_static::lazy_static! {
    static ref STRING_CACHE: Arc<Mutex<HashMap<Hash, Weak<Vec<u8>>>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

/// A version of `BinaryString` used for data that's commonly repeated.
/// `rbx_types` automatically deduplicates data as it's loaded into
/// `SharedString` values.
#[derive(Debug, Clone)]
pub struct SharedString {
    data: Option<Arc<Vec<u8>>>,
    hash: Hash,
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
        &self.data.as_ref().unwrap()
    }

    #[inline]
    pub fn hash(&self) -> SharedStringHash {
        SharedStringHash(self.hash)
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
        if Arc::try_unwrap(self.data.take().unwrap()).is_ok() {
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
pub struct SharedStringHash(Hash);

impl SharedStringHash {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes().as_ref()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // Mock implementations of traits to get things compiling. We'll need to
    // decide how to actually serialize SharedString at some point.

    impl Serialize for SharedString {
        fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
            unimplemented!();
        }
    }

    impl<'de> Deserialize<'de> for SharedString {
        fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
            unimplemented!();
        }
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
