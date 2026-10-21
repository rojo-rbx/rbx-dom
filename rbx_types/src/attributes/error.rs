use std::string::FromUtf8Error;

use thiserror::Error;

use crate::VariantType;

/// The original failure encountered while reading or writing attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// The list length could not be read.
    #[error("missing attribute list length")]
    InvalidLength,

    /// A key's length or bytes could not be read.
    #[error("missing attribute key name")]
    NoKey,

    /// A key retained bytes that do not form UTF-8.
    #[error("attribute key contained invalid UTF-8")]
    KeyBadUnicode(#[source] FromUtf8Error),

    /// The value's type byte could not be read.
    #[error("missing attribute value type")]
    NoValueType,

    /// The original type byte has no known attribute representation.
    #[error("invalid value type: {0}")]
    InvalidValueType(u8),

    /// The value type has no supported attribute reader or writer.
    #[error("{0:?} values are not supported in attributes")]
    UnsupportedVariantType(VariantType),

    /// The full source number does not identify a representable known color.
    #[error("invalid BrickColor value: {0}")]
    InvalidBrickColor(u32),

    /// The original reader or writer returned an I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// A textual value retained bytes that do not form UTF-8.
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    /// A nested native value decoder returned its own structured error.
    #[error(transparent)]
    BadAttributeValue(#[from] crate::Error),

    /// The named field could not be read completely.
    #[error("couldn't read bytes to deserialize {0}")]
    ReadType(&'static str),

    /// The named Font field retained invalid UTF-8 bytes.
    #[error("font contained invalid UTF-8 in {field}")]
    FontBadUnicode {
        /// The original bytes and UTF-8 decoder error.
        #[source]
        source: FromUtf8Error,

        /// The field that contained those bytes.
        field: &'static str,
    },
}
