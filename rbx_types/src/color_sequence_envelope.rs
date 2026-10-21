/// Retains serialized envelope bits that runtime color keypoints do not expose.
///
/// Every IEEE 754 pattern is representable, including positive zero. Decoders
/// decide whether the source differs from the writer's normalization before
/// constructing diagnostic evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorSequenceEnvelope {
    bits: u32,
}

/// Shares the exact envelope representation and normalization across formats.
impl ColorSequenceEnvelope {
    /// The positive-zero bits emitted by writers for runtime color keypoints.
    pub const NORMALIZED_BITS: u32 = 0;

    /// Preserves a source pattern without floating-point conversion or arithmetic.
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Returns the exact original pattern, including zero signs and NaN payloads.
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Returns the pattern a writer emits for the omitted runtime envelope field.
    pub const fn normalized_bits(&self) -> u32 {
        Self::NORMALIZED_BITS
    }
}
