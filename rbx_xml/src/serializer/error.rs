use rbx_dom_weak::types::{Ref, VariantType};
use thiserror::Error;

// The redirection used here is so that the size `EncodeError` occupies on
// the stack is kept small. This is important because the type is included
// as part of the return type of almost every function in the serializer.

#[derive(Debug, Error)]
#[error(transparent)]
/// An error that may be raised when serializing a file.
pub struct EncodeError(Box<ErrorKind>);

#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    /// A property with a type that's not known to the serializer was passed.
    #[error("could not serialize property {0} of type '{1:?}'")]
    UnknownType(String, VariantType),
    #[error("type {0:?} cannot be serialized without a state")]
    TypeNeedsState(VariantType),
    #[error("cannot be strict with {0} without a database")]
    StrictWithoutDatabase(&'static str),

    /// A `Ref` was not located inside of the provided `WeakDom`
    #[error("Ref {0} was not in the Dom")]
    RefNotInDom(Ref),
    /// A class was not known to the reflection database
    #[error("unknown class name '{0}'")]
    UnknownClass(String),
    /// A property was not known by name to the reflection database
    #[error("unknown property '{0}.{1}'")]
    UnknownProperty(String, String),

    /// A conversion between two data types wasn't possible. This occurs when
    /// a property's type does not match the type it is meant to be
    /// and it cannot be converted to the correct one. As such, this will only
    /// be raised when a database is used.
    #[error(
        "property {class}.{name} could not be converted from {from:?} to {to:?} because: {error}"
    )]
    ConversionFail {
        class: String,
        name: String,
        from: VariantType,
        to: VariantType,
        error: super::conversions::ConversionError,
    },

    /// An error with `quick_xml` happened while serializing a file.
    #[error("could not serialize: {0}")]
    XmlSerializing(#[from] quick_xml::Error),
    /// An IO error occured while serializing a file.
    #[error("IO error was encountered: {0}")]
    Io(#[from] std::io::Error),
    #[error("cannot serialize type because {0}")]
    RbxType(#[from] rbx_dom_weak::types::Error),
}

impl ErrorKind {
    /// A convenience function for turning an `ErrorKind` into a `EncodeError`.
    pub fn err(self) -> EncodeError {
        EncodeError(Box::from(self))
    }
}

impl EncodeError {
    /// Returns a new `EncodeError` from the given `ErrorKind`.
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self(Box::from(kind))
    }
}

impl<T> From<T> for EncodeError
where
    ErrorKind: From<T>,
{
    fn from(value: T) -> Self {
        Self(Box::from(ErrorKind::from(value)))
    }
}
