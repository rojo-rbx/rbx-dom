use std::{borrow::Cow, collections::HashMap};

use crate::reflection_types::RbxClassDescriptor;

pub struct ReflectionDatabase {
    pub studio_version: [u32; 4],
    pub classes: HashMap<Cow<'static, str>, RbxClassDescriptor>,
}

impl ReflectionDatabase {
    pub fn new() -> Self {
        Self {
            studio_version: [0, 0, 0, 0],
            classes: HashMap::new(),
        }
    }
}
