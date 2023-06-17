mod conversions;
mod core;
pub(crate) mod data_types;
mod error;
mod reader;

use std::io::Read;

use rbx_dom_weak::WeakDom;

pub use error::DecodeError;
use rbx_reflection::ReflectionDatabase;
pub use reader::XmlReader;

pub(crate) fn decode_internal<R: Read>(
    reader: R,
    config: DecodeOptions,
) -> Result<WeakDom, DecodeError> {
    core::deserialize_file(XmlReader::from_reader(reader), config)
}

/// Describes the strategy that rbx_xml should use when deserializing
/// properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DecodePropertyBehavior {
    /// Ignores properties that aren't known by rbx_xml.
    ///
    /// The default and safest option. With this set, properties that are newer
    /// than the reflection database rbx_xml uses won't show up when
    /// deserializing files.
    #[default]
    IgnoreUnknown,

    /// Read properties that aren't known by rbx_xml.
    ///
    /// With this option set, properties that are newer than rbx_xml's
    /// reflection database will show up. It may be problematic to depend on
    /// these properties, since rbx_xml may start supporting them with
    /// non-reflection specific names at a future date.
    ReadUnknown,

    /// Returns an error if any properties are found that aren't known by
    /// rbx_xml.
    ErrorOnUnknown,

    /// Completely turns off rbx_xml's reflection database. Property names and
    /// types will appear exactly as they are in XML.
    ///
    /// This setting is useful for debugging the model format. It leaves the
    /// user to deal with oddities like how `Part.FormFactor` is actually
    /// serialized as `Part.formFactorRaw`.
    NoReflection,
}

/// Options available for deserializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct DecodeOptions<'db> {
    database: Option<&'db ReflectionDatabase<'db>>,
    unknown_type_err: bool,
    property_behavior: DecodePropertyBehavior,
}

impl<'db> DecodeOptions<'db> {
    /// Constructs a `DecodeOptions` with all values set to their defaults.
    #[inline]
    pub fn new() -> Self {
        DecodeOptions {
            database: None,
            unknown_type_err: false,
            property_behavior: DecodePropertyBehavior::IgnoreUnknown,
        }
    }

    /// Determines how rbx_xml will deserialize properties, especially unknown
    /// ones.
    #[inline]
    pub fn property_behavior(self, property_behavior: DecodePropertyBehavior) -> Self {
        DecodeOptions {
            database: self.database,
            unknown_type_err: self.unknown_type_err,
            property_behavior,
        }
    }

    /// Sets what database to use for deserializing
    pub fn database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        DecodeOptions {
            database: Some(database),
            unknown_type_err: self.unknown_type_err,
            property_behavior: self.property_behavior,
        }
    }

    /// A utility function to determine whether or not we should reference the
    /// reflection database at all.
    pub(crate) fn use_reflection(&self) -> bool {
        self.database.is_some() && self.property_behavior != DecodePropertyBehavior::NoReflection
    }

    /// A utility function to determine whether or not we should ignore unknown
    /// properties.
    pub(crate) fn ignore_unknown(&self) -> bool {
        matches!(
            self.property_behavior,
            DecodePropertyBehavior::IgnoreUnknown | DecodePropertyBehavior::NoReflection
        )
    }

    /// A utility function to determine whether or not we should error when
    /// an unknown type is encountered
    pub(crate) fn error_on_unknown_type(&self) -> bool {
        self.unknown_type_err
    }
}
