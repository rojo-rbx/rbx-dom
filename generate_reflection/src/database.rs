use std::{
    borrow::Cow,
    collections::HashMap,
};

use crate::{
    api_dump::Dump,
    reflection_types::RbxInstanceClass,
};

pub struct ReflectionDatabase {
    pub dump: Dump,
    pub studio_version: [u32; 4],
    pub classes: HashMap<Cow<'static, str>, RbxInstanceClass>,
}