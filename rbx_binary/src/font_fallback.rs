use rbx_dom_weak::types::{FontStyle, FontWeight};

/// Identifies an unknown native Font discriminator and the default substituted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFallback {
    /// An unknown two-byte weight was replaced with the native default weight.
    Weight {
        /// The exact weight discriminator read from the native property.
        found: u16,
        /// The weight inserted into the resulting Font value.
        replacement: FontWeight,
    },
    /// An unknown one-byte style was replaced with the native default style.
    Style {
        /// The exact style discriminator read from the native property.
        found: u8,
        /// The style inserted into the resulting Font value.
        replacement: FontStyle,
    },
}
