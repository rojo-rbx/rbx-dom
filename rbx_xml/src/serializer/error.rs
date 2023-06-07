use rbx_dom_weak::types::{Ref, VariantType};
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct EncodeError(Box<ErrorKind>);

#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    /// A property with a type that's not known to the serializer was passed.
    #[error("could not serialize property of type '{0:?}'")]
    UnknownType(VariantType),
    #[error("type {0:?} cannot be serialized without a state")]
    TypeNeedsState(VariantType),

    #[error("Instance {0} was not in the Dom")]
    InstNotInDom(Ref),

    /// A general serializer error happened while serializing a file.
    #[error("Could not serialize: {0}")]
    XmlSerializing(#[from] quick_xml::Error),
    /// An IO error occured while serializing a file.
    #[error("IO error was encountered: {0}")]
    Io(#[from] std::io::Error),
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
