//! An implementation of interned buffers keyed by MD5 hash, which is the
//! mechanism Roblox uses in the SharedString API.

use std::{
    collections::HashMap,
    sync::{Arc, Weak, RwLock},
};

use serde::{Serialize, Deserialize};

lazy_static::lazy_static! {
    static ref CACHE: RwLock<HashMap<[u8; 16], Weak<Vec<u8>>>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handle {
    key: [u8; 16],

    #[serde(skip)]
    value: Option<Arc<Vec<u8>>>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        match Arc::try_unwrap(self.value.take().unwrap()) {
            Ok(_) => {
                let mut cache = match CACHE.write() {
                    Ok(v) => v,
                    Err(_) => {
                        // If the lock is poisoned, we should just leave it
                        // alone so that we don't accidentally double-panic.
                        return;
                    }
                };

                cache.remove(&self.key);
            }
            // There are other references to this buffer still
            Err(_) => {}
        }
    }
}

fn get(key: [u8; 16]) -> Option<Handle> {
    let cache = CACHE.read().unwrap();

    cache.get(&key).and_then(|value| {
        Some(Handle {
            key,
            value: Some(value.upgrade()?.clone()),
        })
    })
}

fn insert(value: Vec<u8>) -> Handle {
    let key = {
        let mut context = md5::Context::new();
        context.consume(&value);
        context.compute().0
    };
    let value = Arc::new(value);

    // Explicitly return previous value, since Handle::drop will attempt to take
    // a write lock on the cache and we don't want to deadlock.
    let previous = {
        let mut cache = CACHE.write().unwrap();
        cache.insert(key, Arc::downgrade(&value))
    };

    // Explicitly drop the previous value here, after the lock is released.
    drop(previous);

    Handle {
        key,
        value: Some(value),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_get() {
        let handle = insert(vec![1, 2, 3]);
        let second_handle = get(handle.key);

        assert_eq!(Some(handle), second_handle);
    }

    #[test]
    fn drop() {
        let key = {
            insert(vec![4, 5, 6]).key
        };

        assert_eq!(get(key), None);
    }
}
