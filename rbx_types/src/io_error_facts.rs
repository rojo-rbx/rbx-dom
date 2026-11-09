use std::io::ErrorKind;

use serde::{ser::Error, Serialize, Serializer};

use crate::DiagnosticSerializationError;

/// Available facts from an original I/O error, without reconstructing it.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct IoErrorFacts {
    kind: &'static str,
    origin: IoErrorOrigin,
}

/// Where the original I/O error came from, as far as the standard library exposes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum IoErrorOrigin {
    /// An operating-system error with its raw code.
    Os {
        /// The original raw OS error code.
        code: i32,
    },
    /// A caller-supplied inner error whose type and content stay opaque.
    Custom,
    /// A standard-library error with no OS code and no inner error.
    Simple,
}

/// Projects an I/O error's kind and origin, refusing kinds it cannot name exactly.
impl IoErrorFacts {
    /// Reads the original error's kind and origin.
    ///
    /// Returns [`DiagnosticSerializationError::IoKind`] for a standard kind
    /// this projection does not name, rather than substituting another kind.
    pub fn new(error: &std::io::Error) -> Result<Self, DiagnosticSerializationError> {
        let origin = match (error.raw_os_error(), error.get_ref()) {
            (Some(code), _) => IoErrorOrigin::Os { code },
            (None, Some(_)) => IoErrorOrigin::Custom,
            (None, None) => IoErrorOrigin::Simple,
        };
        Ok(Self {
            kind: kind_name(error.kind())?,
            origin,
        })
    }

    /// Returns the standard kind's name.
    pub fn kind(&self) -> &'static str {
        self.kind
    }

    /// Returns where the original error came from.
    pub fn origin(&self) -> IoErrorOrigin {
        self.origin
    }

    /// Serializes an original I/O error through these same available facts.
    pub fn serialize_error<S: Serializer>(
        error: &std::io::Error,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        Self::new(error)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}

fn kind_name(kind: ErrorKind) -> Result<&'static str, DiagnosticSerializationError> {
    Ok(match kind {
        ErrorKind::NotFound => "NotFound",
        ErrorKind::PermissionDenied => "PermissionDenied",
        ErrorKind::ConnectionRefused => "ConnectionRefused",
        ErrorKind::ConnectionReset => "ConnectionReset",
        ErrorKind::ConnectionAborted => "ConnectionAborted",
        ErrorKind::NotConnected => "NotConnected",
        ErrorKind::AddrInUse => "AddrInUse",
        ErrorKind::AddrNotAvailable => "AddrNotAvailable",
        ErrorKind::BrokenPipe => "BrokenPipe",
        ErrorKind::AlreadyExists => "AlreadyExists",
        ErrorKind::WouldBlock => "WouldBlock",
        ErrorKind::InvalidInput => "InvalidInput",
        ErrorKind::InvalidData => "InvalidData",
        ErrorKind::TimedOut => "TimedOut",
        ErrorKind::WriteZero => "WriteZero",
        ErrorKind::Interrupted => "Interrupted",
        ErrorKind::Unsupported => "Unsupported",
        ErrorKind::UnexpectedEof => "UnexpectedEof",
        ErrorKind::OutOfMemory => "OutOfMemory",
        ErrorKind::Other => "Other",
        kind => return Err(DiagnosticSerializationError::IoKind { kind }),
    })
}
