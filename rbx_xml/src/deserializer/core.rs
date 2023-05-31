use std::io::BufRead;

use quick_xml::{events::Event, Reader};

use rbx_dom_weak::WeakDom;
use rbx_reflection::ReflectionDatabase;

use super::{error::DecodeError, reader::XmlReader};

pub(crate) fn decode_internal<R: BufRead>(
    reader: XmlReader<R>,
    config: DecodeConfig,
) -> Result<WeakDom, DecodeError> {
    unimplemented!()
}

/// A struct configuring the behavior of the deserializer.
/// By default, this uses no database. To add one, use `set_database`.
pub struct DecodeConfig<'db> {
    /// What database if any to use for decoding properties and classes.
    pub(crate) database: Option<ReflectionDatabase<'db>>,
    /// When `true`, class names be checked against the database and
    /// an error will be raised when an unknown class is encountered.
    pub(crate) strict_class_names: bool,
    /// When `true`, property types will be checked against the database and
    /// an error will be raised when a type mismatch is encountered.
    pub(crate) strict_data_types: bool,
    /// When `true`, property names will be checked against the database and
    /// an error will be raised when unknown properties are encountered.
    pub(crate) strict_property_names: bool,
    /// When `true`, any new property data types will be skipped.
    /// Otherwise, an error will be raised when a new data type is encountered.
    pub(crate) ignore_new_types: bool,
}

impl<'db> Default for DecodeConfig<'db> {
    fn default() -> Self {
        Self {
            database: None,
            strict_class_names: false,
            strict_data_types: false,
            strict_property_names: false,
            // This is why we manually implement `Default`!
            ignore_new_types: true,
        }
    }
}

impl<'db> DecodeConfig<'db> {
    /// Creates a new `DecodeConfig` with the default options. This means
    /// no database is used and unknown data types are skipped during
    /// deserialization.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `DecodeConfig` with the given database. By default,
    /// class names, property names, and property types are checked.
    /// Additionally, new data types are ignored.
    pub fn with_database(database: ReflectionDatabase<'db>) -> Self {
        Self {
            database: Some(database),
            strict_class_names: true,
            strict_data_types: true,
            strict_property_names: true,
            ignore_new_types: true,
        }
    }

    /// Sets the deserializer to use the given `ReflectionDatabase`.
    pub fn database(mut self, database: ReflectionDatabase<'db>) -> Self {
        self.database = Some(database);
        self
    }

    /// Sets whether class names are checked against the database. If `true`,
    /// an error will be raised during deserialization if an unknown class
    /// is encountered.
    pub fn strict_class_names(mut self, ignore: bool) -> Self {
        self.strict_class_names = ignore;
        self
    }

    /// Sets whether property data types are checked against the database.
    /// If `true`, an error will be raised during deserialization if a
    /// property's type does not match in the database.
    pub fn strict_data_types(mut self, ignore: bool) -> Self {
        self.strict_data_types = ignore;
        self
    }

    /// Sets whether property names are checked against the database.
    /// If `true`, an error will be raised during deserialization if a
    /// property's type does not match in the database.
    pub fn strict_property_names(mut self, ignore: bool) -> Self {
        self.strict_property_names = ignore;
        self
    }

    /// Sets whether unknown property data types are ignored during
    /// deserialization. If `true`, any property of an unknown type will be
    /// skipped.
    pub fn ignore_new_types(mut self, ignore: bool) -> Self {
        self.ignore_new_types = ignore;
        self
    }
}
