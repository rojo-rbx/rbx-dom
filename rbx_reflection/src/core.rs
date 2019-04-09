use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;

use crate::{
    reflection_database::{generate_classes, generate_enums},
    types::{RbxEnum, RbxInstanceClass},
};

lazy_static! {
    static ref CLASSES: HashMap<Cow<'static, str>, RbxInstanceClass> = generate_classes();
    static ref ENUMS: HashMap<Cow<'static, str>, RbxEnum> = generate_enums();
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
