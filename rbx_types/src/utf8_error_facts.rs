use serde::{Serialize, Serializer};

/// Available facts from the actual UTF-8 decoder, without reconstructing its error.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct Utf8ErrorFacts {
    #[serde(serialize_with = "crate::diagnostic_serde::decimal")]
    #[cfg_attr(feature = "typescript", ts(as = "String"))]
    valid_up_to: usize,
    error_len: Option<usize>,
}

/// Borrows the standard decoder's offsets without re-decoding bytes.
impl Utf8ErrorFacts {
    /// Records the original offset and invalid width (or incomplete sequence).
    pub fn new(error: std::str::Utf8Error) -> Self {
        Self {
            valid_up_to: error.valid_up_to(),
            error_len: error.error_len(),
        }
    }

    /// Returns the original byte offset before the invalid or incomplete sequence.
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    /// Returns the actual invalid width, or None for an incomplete sequence.
    pub fn error_len(&self) -> Option<usize> {
        self.error_len
    }

    /// Serializes an original standard error through these same available facts.
    pub fn serialize_error<S: Serializer>(
        error: &std::str::Utf8Error,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::new(*error).serialize(serializer)
    }
}
