use serde::{Serialize, Serializer};

use crate::Utf8ErrorFacts;

/// The exact retained buffer and original UTF-8 failure facts from a String decode.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct FromUtf8ErrorFacts<'a> {
    bytes: &'a [u8],
    cause: Utf8ErrorFacts,
}

/// Keeps the original buffer borrowed for the duration of outgoing serialization.
impl<'a> FromUtf8ErrorFacts<'a> {
    /// Projects the error's retained bytes and original standard UTF-8 cause.
    pub fn new(error: &'a std::string::FromUtf8Error) -> Self {
        Self {
            bytes: error.as_bytes(),
            cause: Utf8ErrorFacts::new(error.utf8_error()),
        }
    }

    /// Returns the exact buffer retained by the original error.
    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    /// Returns the original decoder's offset and invalid width.
    pub fn cause(&self) -> &Utf8ErrorFacts {
        &self.cause
    }

    /// Serializes a retained String decode error without copying its buffer.
    pub fn serialize_error<S: Serializer>(
        error: &std::string::FromUtf8Error,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        FromUtf8ErrorFacts::new(error).serialize(serializer)
    }
}
