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

/// Retrieves reflection information for all known classes indexed by name.
#[inline]
pub fn get_classes() -> &'static HashMap<Cow<'static, str>, RbxInstanceClass> {
    &CLASSES
}

/// Retrieves reflection information for all known enum values indexed by name.
#[inline]
pub fn get_enums() -> &'static HashMap<Cow<'static, str>, RbxEnum> {
    &ENUMS
}
