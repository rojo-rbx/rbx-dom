use crate::{FontFallback, Variant};

/// A source distinction replaced while decoding an otherwise valid attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeDecodeDiagnostic {
    /// A color envelope's source bits differed from those emitted by the writer.
    ColorSequenceEnvelope {
        /// The original attribute name containing the color sequence.
        attribute: String,
        /// The zero-based serialized attribute entry index.
        index: u32,
        /// The zero-based keypoint index within that attribute's sequence.
        keypoint_index: u32,
        /// The exact original IEEE 754 single-precision envelope bits.
        found_bits: u32,
        /// The positive-zero bits emitted for this field by the attribute writer.
        normalized_bits: u32,
    },
    /// A later entry with the same name displaced an already decoded value.
    DuplicateEntry {
        /// The original attribute name shared by both entries.
        attribute: String,
        /// The serialized entry index of the displaced value.
        index: u32,
        /// The serialized entry index whose value replaced it.
        replacement_index: u32,
        /// The actual decoded value removed from the resulting attribute map.
        displaced_value: Variant,
    },
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
