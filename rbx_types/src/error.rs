use thiserror::Error;

use crate::{AttributeError, Matrix3Error};

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

impl From<Matrix3Error> for Error {
    fn from(source: Matrix3Error) -> Self {
        Self {
            source: Box::new(source.into()),
        }
    }
}

#[derive(Debug, Error)]
enum InnerError {
    #[error(transparent)]
    AttributeError(#[from] AttributeError),

    #[error(transparent)]
    Matrix3Error(#[from] Matrix3Error),
}
