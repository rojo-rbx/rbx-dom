mod conversions;
mod core;
mod data_types;
mod error;
mod writer;

use std::io::Write;

use rbx_dom_weak::{types::Ref, WeakDom};
use rbx_reflection::ReflectionDatabase;

pub use error::EncodeError;
pub use writer::XmlWriter;

pub fn encode_internal<W: Write>(
    output: &mut W,
    dom: &WeakDom,
    refs: &[Ref],
    options: EncodeOptions,
) -> Result<(), EncodeError> {
    core::serialize_refs(output, dom, refs, options)
}

/// Describes the strategy that rbx_xml should use when serializing properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EncodePropertyBehavior {
    /// Ignores properties that aren't known by rbx_xml.
    ///
    /// This is the default.
    IgnoreUnknown,

    /// Write unrecognized properties.
    ///
    /// With this option set, properties that are newer than rbx_xml's
    /// reflection database will show up. It may be problematic to depend on
    /// these properties, since rbx_xml may start supporting them with
    /// non-reflection specific names at a future date.
    WriteUnknown,

    /// Returns an error if any properties are found that aren't known by
    /// rbx_xml.
    ErrorOnUnknown,

    /// Completely turns off rbx_xml's reflection database. Property names and
    /// types will appear exactly as they are in the tree.
    ///
    /// This setting is useful for debugging the model format. It leaves the
    /// user to deal with oddities like how `Part.FormFactor` is actually
    /// serialized as `Part.formFactorRaw`.
    NoReflection,
}

/// Options available for serializing an XML-format model or place.
#[derive(Debug, Clone)]
pub struct EncodeOptions<'db> {
    database: Option<&'db ReflectionDatabase<'db>>,
    unknown_type_err: bool,
    property_behavior: EncodePropertyBehavior,
}

impl<'db> EncodeOptions<'db> {
    /// Constructs a `EncodeOptions` with all values set to their defaults.
    #[inline]
    pub fn new() -> Self {
        EncodeOptions {
            database: None,
            unknown_type_err: false,
            property_behavior: EncodePropertyBehavior::IgnoreUnknown,
        }
    }

    /// Determines how rbx_xml will serialize properties, especially unknown
    /// ones.
    #[inline]
    pub fn property_behavior(self, property_behavior: EncodePropertyBehavior) -> Self {
        EncodeOptions {
            database: self.database,
            unknown_type_err: self.unknown_type_err,
            property_behavior,
        }
    }

    /// Sets what database to use for serializing
    pub fn database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        EncodeOptions {
            database: Some(database),
            unknown_type_err: self.unknown_type_err,
            property_behavior: self.property_behavior,
        }
    }

    /// A utility function to determine whether or not we should reference the
    /// reflection database at all.
    pub(crate) fn use_reflection(&self) -> bool {
        self.property_behavior != EncodePropertyBehavior::NoReflection
    }

    /// A utility function to determine whether or not we should ignore unknown
    /// properties.
    pub(crate) fn ignore_unknown(&self) -> bool {
        matches!(
            self.property_behavior,
            EncodePropertyBehavior::IgnoreUnknown | EncodePropertyBehavior::NoReflection
        )
    }

    /// A utility function to determine whether or not we should error when
    /// an unknown type is encountered
    pub(crate) fn error_on_unknown_type(&self) -> bool {
        self.unknown_type_err
    }
}
