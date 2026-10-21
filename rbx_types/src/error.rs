use thiserror::Error;

use crate::{AttributeError, MaterialColorsError, Matrix3Error, UniqueIdError};

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
    UniqueId(#[from] UniqueIdError),
}

#[cfg(test)]
mod tests {
    use behave::prelude::*;

    use super::{AttributeError, Error, InnerError};
    use crate::{Attributes, AttributesDecodeReport};

    behave! {
        "Reading invalid attribute colors" {
            "a color number is validated before any narrowing and its error retains the original number" {
                // @behavior attribute-brick-color-full-width-refusal
                // @boundary before=full-width-color-number at=public-attributes-entries after=typed-original-number-error
                // @red Number65537 currently succeeds as White after modulo65536 narrowing.
                // @green Both public entries reject65537 and retain its full number; low-invalid0 remains refused.
                for raw in [65_537u32, 0] {
                    let mut bytes = vec![1, 0, 0, 0, 1, 0, 0, 0, b'A', 0x0e];
                    bytes.extend(raw.to_le_bytes());
                    let entries = [
                        Attributes::from_reader(bytes.as_slice()),
                        Attributes::from_reader_with_diagnostics(bytes.as_slice()).map(AttributesDecodeReport::into_attributes),
                    ];
                    for result in entries {
                        match result {
                            Err(Error { source }) => match *source {
                                InnerError::Attribute(AttributeError::InvalidBrickColor(found)) => expect!(found).to_equal(raw)?,
                                found => expect!(format!("{found:?}")).to_equal("InvalidBrickColor".to_owned())?,
                            },
                            Ok(found) => expect!(format!("{found:?}")).to_equal("a full-width color refusal".to_owned())?,
                        }
                    }
                }
            }
        }
    }
}
