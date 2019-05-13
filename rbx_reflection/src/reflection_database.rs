use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;

use crate::reflection_types::{RbxClassDescriptor, RbxEnumDescriptor};

mod classes;
mod enums;
mod version;

pub use self::version::*;

lazy_static! {
    static ref CLASSES: HashMap<Cow<'static, str>, RbxClassDescriptor> =
        self::classes::generate_classes();
    static ref ENUMS: HashMap<Cow<'static, str>, RbxEnumDescriptor> = self::enums::generate_enums();
}

/// Returns the class descriptor with the given name from the reflection
/// database, if it exists.
#[inline]
pub fn get_class_descriptor(name: &str) -> Option<&'static RbxClassDescriptor> {
    CLASSES.get(name)
}

/// Returns an iterator over all class descriptors in the reflection database.
#[inline]
pub fn iter_class_descriptors() -> impl Iterator<Item = (&'static str, &'static RbxClassDescriptor)>
{
    CLASSES.iter().map(|(key, value)| (key.as_ref(), value))
}

/// Returns the enum descriptor with the given name from the reflection
/// database, if it exists.
#[inline]
pub fn get_enum_descriptor(name: &str) -> Option<&'static RbxEnumDescriptor> {
    ENUMS.get(name)
}

/// Returns an itertaor over all enum descriptors in the reflection database.
#[inline]
pub fn iter_enum_descriptors() -> impl Iterator<Item = (&'static str, &'static RbxEnumDescriptor)> {
    ENUMS.iter().map(|(key, value)| (key.as_ref(), value))
}
