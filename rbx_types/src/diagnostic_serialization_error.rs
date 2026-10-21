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
}
