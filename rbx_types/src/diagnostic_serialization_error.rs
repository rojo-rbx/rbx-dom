/// A projection cannot report exact facts for an unsupported source representation.
#[derive(Debug, thiserror::Error)]
pub enum DiagnosticSerializationError {
    /// The canonical value serializer refused the value before JSON conversion.
    #[error("canonical value serialization failed: {source}")]
    Value {
        /// The original serializer error.
        #[source]
        source: serde_value::SerializerError,
    },
    /// A future or unstable I/O category has no established outgoing representation.
    #[error("unsupported I/O error kind: {kind:?}")]
    IoKind {
        /// The actual standard-library category, without an arbitrary replacement.
        kind: std::io::ErrorKind,
    },
    /// A future integer parser category has no established outgoing representation.
    #[error("unsupported integer parser error kind: {kind:?}")]
    IntegerKind {
        /// The actual parser category, without an arbitrary replacement.
        kind: std::num::IntErrorKind,
    },
}
