use thiserror::Error;

use crate::{AttributeError, MaterialColorsError, Matrix3Error, SmoothGridError, UniqueIdError};

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

impl From<MaterialColorsError> for Error {
    fn from(source: MaterialColorsError) -> Self {
        Self {
            source: Box::new(source.into()),
        }
    }
}

impl From<SmoothGridError> for Error {
    fn from(source: SmoothGridError) -> Self {
        Self {
            source: Box::new(source.into()),
        }
    }
}

impl From<UniqueIdError> for Error {
    fn from(source: UniqueIdError) -> Self {
        Self {
            source: Box::new(source.into()),
        }
    }
}

#[derive(Debug, Error)]
enum InnerError {
    #[error(transparent)]
    Attribute(#[from] AttributeError),

    #[error(transparent)]
    Matrix3(#[from] Matrix3Error),

    #[error(transparent)]
    MaterialColors(#[from] MaterialColorsError),

    #[error(transparent)]
    SmoothGrid(#[from] SmoothGridError),

    #[error(transparent)]
    UniqueId(#[from] UniqueIdError),
}
