use rbx_dom_weak::types::Error;

/// Retains the native semantic decoder's error while the bytes remain available.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum BinaryStringFallbackCause {
    /// Attributes could not be interpreted and remained a BinaryString.
    #[error("Attributes decoding failed")]
    Attributes(#[source] Error),
    /// MaterialColors could not be interpreted and remained a BinaryString.
    #[error("MaterialColors decoding failed")]
    MaterialColors(#[source] Error),
}
