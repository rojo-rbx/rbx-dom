use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;

use crate::reflection_types::{RbxEnum, RbxInstanceClass};

mod classes;
mod enums;
mod version;

pub use self::version::*;

lazy_static! {
    static ref CLASSES: HashMap<Cow<'static, str>, RbxInstanceClass> =
        self::classes::generate_classes();
    static ref ENUMS: HashMap<Cow<'static, str>, RbxEnum> = self::enums::generate_enums();
}

#[inline]
pub fn get_class_descriptor(name: &str) -> Option<&'static RbxInstanceClass> {
    CLASSES.get(name)
}

#[inline]
pub fn iter_class_descriptors() -> impl Iterator<Item = (&'static str, &'static RbxInstanceClass)> {
    CLASSES.iter().map(|(key, value)| (key.as_ref(), value))
}

#[inline]
pub fn get_enum_descriptor(name: &str) -> Option<&'static RbxEnum> {
    ENUMS.get(name)
}

#[inline]
pub fn iter_enum_descriptors() -> impl Iterator<Item = (&'static str, &'static RbxEnum)> {
    ENUMS.iter().map(|(key, value)| (key.as_ref(), value))
}