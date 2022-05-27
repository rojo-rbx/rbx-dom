pub struct VecMap<T> {
    storage: Vec<Option<T>>,
}

impl<T> VecMap<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut storage = Vec::new();
        storage.resize_with(capacity, || None);

        Self { storage }
    }

    pub fn insert<K: Key>(&mut self, key: K, value: T) {
        let key = key.into_usize();

        self.grow_if_needed(key);
        self.storage[key] = Some(value);
    }

    pub fn get<K: Key>(&self, key: K) -> Option<&T> {
        let key = key.into_usize();

        self.storage.get(key).and_then(|entry| entry.as_ref())
    }

    pub fn get_mut<K: Key>(&mut self, key: K) -> Option<&mut T> {
        let key = key.into_usize();

        self.storage.get_mut(key).and_then(|entry| entry.as_mut())
    }

    pub fn remove<K: Key>(&mut self, key: K) -> Option<T> {
        let key = key.into_usize();

        self.storage.get_mut(key).and_then(|entry| entry.take())
    }

    fn grow_if_needed(&mut self, new_max: usize) {
        let new_cap = new_max + 1;
        if self.storage.len() < new_cap {
            self.storage.resize_with(new_cap, || None);
        }
    }
}

pub trait Key {
    fn into_usize(self) -> usize;
}

impl Key for usize {
    fn into_usize(self) -> usize {
        self
    }
}

impl Key for u32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl Key for i32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
