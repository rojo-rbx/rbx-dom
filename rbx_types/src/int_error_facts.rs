use std::num::{IntErrorKind, ParseIntError};

use serde::{ser::Error, Serialize, Serializer};

use crate::DiagnosticSerializationError;

/// The original integer parser's failure category, without reconstructing the error.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct IntErrorFacts {
    kind: &'static str,
}

/// Projects a parser error's kind, refusing kinds it cannot name exactly.
impl IntErrorFacts {
    /// Reads the original parser error's kind.
    ///
    /// Returns [`DiagnosticSerializationError::IntegerKind`] for a parser kind
    /// this projection does not name, rather than substituting another kind.
    pub fn new(error: &ParseIntError) -> Result<Self, DiagnosticSerializationError> {
        let kind = match error.kind() {
            IntErrorKind::Empty => "Empty",
            IntErrorKind::InvalidDigit => "InvalidDigit",
            IntErrorKind::PosOverflow => "PosOverflow",
            IntErrorKind::NegOverflow => "NegOverflow",
            IntErrorKind::Zero => "Zero",
            kind => return Err(DiagnosticSerializationError::IntegerKind { kind: *kind }),
        };
        Ok(Self { kind })
    }

    /// Returns the parser kind's name.
    pub fn kind(&self) -> &'static str {
        self.kind
    }

    /// Serializes an original parser error through these same available facts.
    pub fn serialize_error<S: Serializer>(
        error: &ParseIntError,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::new(error)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}
