//! An implementation of interned buffers keyed by MD5 hash, which is the
//! mechanism Roblox uses in the SharedString API.

use std::{
    collections::HashMap,
    sync::{Arc, Weak, RwLock},
};

use serde::{Serialize, Serializer, Deserialize, Deserializer};

lazy_static::lazy_static! {
    static ref CACHE: RwLock<HashMap<[u8; 16], Weak<Vec<u8>>>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedString {
    data: Option<Arc<Vec<u8>>>,
    hash: [u8; 16],
}

impl SharedString {
    pub fn get_from_hash(hash: [u8; 16]) -> Option<SharedString> {
        let cache = CACHE.read().unwrap();

        cache.get(&hash).and_then(|data| {
            Some(SharedString {
                hash,
                data: Some(data.upgrade()?.clone()),
            })
        })
    }

    pub fn insert(data: Vec<u8>) -> SharedString {
        let hash = {
            let mut context = md5::Context::new();
            context.consume(&data);
            context.compute().0
        };
        let data = Arc::new(data);

        // Explicitly return previous data, since SharedString::drop will attempt to
        // take a write lock on the cache and we don't want to deadlock.
        let previous = {
            let mut cache = CACHE.write().unwrap();
            cache.insert(hash, Arc::downgrade(&data))
        };

        // Explicitly drop the previous data here, after the lock is released.
        drop(previous);

        SharedString {
            hash,
            data: Some(data),
        }
    }
}

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

impl<'de> Deserialize<'de> for SharedString {
    fn deserialize<D>(_deserializer: D) -> Result<SharedString, D::Error>
    where
        D: Deserializer<'de>,
    {
        panic!("ahh");
    }
}

impl Drop for SharedString {
    fn drop(&mut self) {
        match Arc::try_unwrap(self.data.take().unwrap()) {
            Ok(_) => {
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
            // There are other references to this buffer still, so the entry
            // should remain in the string cache.
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_get() {
        let handle = SharedString::insert(vec![1, 2, 3]);
        let second_handle = SharedString::get_from_hash(handle.hash);

        assert_eq!(Some(handle), second_handle);
    }

    #[test]
    fn drop() {
        let hash = {
            SharedString::insert(vec![4, 5, 6]).hash
        };

        assert_eq!(SharedString::get_from_hash(hash), None);
    }
}
