use crate::FontFallback;

/// A source distinction replaced while decoding an otherwise valid attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeDecodeDiagnostic {
    /// An unknown Font weight or style became a native default.
    FontFallback {
        /// The original attribute name, before insertion into the returned map.
        attribute: String,
        /// The zero-based serialized entry index, including repeated names.
        index: u32,
        /// The exact discriminator and the typed value substituted for it.
        fallback: FontFallback,
    },
}
