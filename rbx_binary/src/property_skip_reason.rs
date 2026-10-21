/// Explains why an entire serialized property was omitted from the DOM.
#[derive(Debug, PartialEq, Eq)]
pub enum PropertySkipReason {
    /// The PROP ended immediately after its property name.
    MissingType,
    /// The decoder does not recognize this raw binary type byte.
    UnknownType(u8),
    /// Reflection explicitly marks the canonical property as nonserializable.
    DoesNotSerialize {
        /// The canonical property whose reflection policy caused the skip.
        canonical_property: String,
        /// The native type byte whose payload was skipped.
        binary_type: u8,
    },
    /// A recognized binary type has no default DOM representation.
    ///
    /// All currently recognized binary types have default representations; this
    /// variant retains evidence if that mapping gains an unsupported type.
    UnsupportedType {
        /// The recognized native binary type byte.
        binary_type: u8,
    },
}
