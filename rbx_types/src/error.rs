use thiserror::Error;

use crate::AttributeError;

/// Represents an error that occurred when using a fallible method.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<AttributeError> for Error {
    fn from(source: AttributeError) -> Self {
        Self {
            source: Box::new(source.into()),
        }
    }
}

#[derive(Debug, Error)]
enum InnerError {
    #[error(transparent)]
    AttributeError(#[from] AttributeError),
}
