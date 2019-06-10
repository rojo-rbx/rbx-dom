//! An implementation of interned buffers keyed by MD5 hash, which is the
//! mechanism Roblox uses in the SharedString API.

use std::{
    collections::{hash_map, HashMap},
    sync::{Arc, Weak, RwLock},
};

use serde::{Serialize, Serializer, Deserialize, Deserializer};

lazy_static::lazy_static! {
    static ref CACHE: RwLock<HashMap<[u8; 16], Weak<Vec<u8>>>> = RwLock::new(HashMap::new());
}

/// A shared buffer of data that is automatically deduplicated so that only one
/// copy of a given buffer is in memory at a time.
///
/// SharedString is cheap to clone, since it's internally reference-counted.
///
/// This type was introduced by Roblox to help efficiently store data like CSG
/// operation, which was previously kept as BinaryValue objects in
/// CSGDictionaryService.
///
/// rbx_dom_weak's implementation of SharedString tries to be faithful to how
/// SharedString works in Roblox, which unfortunately means that it's keyed by
/// MD5 hash. In the event of a hash collision, one of the involved buffers will
/// be silently discarded. It's possible to diverge from this behavior, but
/// could potentially cause edge-case content compatibility issues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedString {
    // We use an Option<Arc<T>> instead of just an Arc<T> so that our Drop impl
    // can take ownership of the Arc.
    data: Option<Arc<Vec<u8>>>,
    hash: [u8; 16],
}

impl SharedString {
    /// Constructs a SharedString from a buffer of data.
    ///
    /// If the data is already present in memory as a SharedString, this method
    /// will return a reference to it, otherwise it'll be inserted.
    pub fn new(data: Vec<u8>) -> SharedString {
        let hash = {
            let mut context = md5::Context::new();
            context.consume(&data);
            context.compute().0
        };

        let data = {
            let data = Arc::new(data);
            let mut cache = CACHE.write().unwrap();

            match cache.entry(hash) {
                hash_map::Entry::Occupied(mut occupied) => {
                    match occupied.get().upgrade() {
                        Some(data) => {
                            // This is an existing entry that hasn't expired,
                            // the happy path!
                            data
                        }
                        None => {
                            // An entry is still present in the global cache,
                            // but there are no outstanding handles.
                            //
                            // This might not do quite what we want. The Arc
                            // might have been unwrapped in SharedString::drop
                            // but not yet removed from the cache. That
                            // worst-case should still be okay, we'll just end
                            // up with an extra copy of this data.
                            occupied.insert(Arc::downgrade(&data));
                            data
                        }
                    }
                }
                hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(Arc::downgrade(&data));
                    data
                }
            }
        };

        SharedString {
            hash,
            data: Some(data),
        }
    }

    /// Attempts to find an existing SharedString with the given MD5 hash.
    #[allow(unused)]
    fn get_from_md5_hash(hash: [u8; 16]) -> Option<SharedString> {
        let cache = CACHE.read().unwrap();

        cache.get(&hash).and_then(|data| {
            Some(SharedString {
                hash,
                data: Some(data.upgrade()?.clone()),
            })
        })
    }

    /// Returns the MD5 hash of the SharedString, which is used as unique
    /// identifier presently barring hash collisions.
    pub fn md5_hash(&self) -> [u8; 16] {
        self.hash
    }

    /// Returns an immutable reference to the underlying buffer from the
    /// SharedString.
    pub fn data(&self) -> &[u8] {
        self.data.as_ref().unwrap()
    }
}

/// SharedString serializes its hash when serialized through Serde. This is to
/// prevent accidentally creating lots of redundant copies of its data.
impl Serialize for SharedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&base64::encode(&self.hash))
        } else {
            serializer.serialize_bytes(&self.hash)
        }
    }
}

/// SharedString cannot be deserialized, since the serialization implementation
/// only includes its hash.
impl<'de> Deserialize<'de> for SharedString {
    fn deserialize<D>(_deserializer: D) -> Result<SharedString, D::Error>
    where
        D: Deserializer<'de>,
    {
        panic!("SharedString cannot be deserialized using Serde");
    }
}

impl Drop for SharedString {
    fn drop(&mut self) {
        // If the reference we're about to drop is the very last reference to
        // the buffer, we'll be able to unwrap it and remove it from the
        // SharedString cache.
        if Arc::try_unwrap(self.data.take().unwrap()).is_ok() {
            let mut cache = match CACHE.write() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_get() {
        let handle_1 = SharedString::new(vec![1, 2, 3]);
        let handle_2 = SharedString::get_from_md5_hash(handle_1.hash)
            .expect("Couldn't find SharedString that was just inserted");

        let data_1 = handle_1.data.as_ref().unwrap();
        let data_2 = handle_2.data.as_ref().unwrap();

        assert!(Arc::ptr_eq(data_1, data_2));
    }

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
        let hash = {
            SharedString::new(vec![4, 5, 6]).hash
        };

        assert_eq!(SharedString::get_from_md5_hash(hash), None);
    }

    #[test]
    fn serialization() {
        let value = SharedString::new(vec![9, 1, 3, 4]);

        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, r#""mdxCr5/mgQ6KwGBx5rzuGg==""#);
    }
}
