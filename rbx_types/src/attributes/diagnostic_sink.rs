use std::collections::BTreeMap;

use crate::{ColorSequenceEnvelope, FontFallback, Variant};

use super::color_sequence_envelope_source::ColorSequenceEnvelopeSource;
use super::{AttributeDecodeDiagnostic, Attributes, AttributesDecodeReport};

/// Selects source retention without adding diagnostic state to legacy values.
pub(super) trait AttributeReadSink {
    /// Keeps source indices only in the audited map representation.
    type Stored;
    /// Prevents a discarding entry from returning an empty audited report.
    type Output;

    /// Transfers a decoded value into the selected map representation.
    fn store(index: u32, value: Variant) -> Self::Stored;
    /// Copies a source name only when its substitution will be retained.
    fn font_fallback(&mut self, index: u32, attribute: &str, fallback: FontFallback);
    /// Retains omitted envelope bits only when the audited entry requested them.
    fn color_sequence_envelope(&mut self, source: ColorSequenceEnvelopeSource<'_>);
    /// Takes ownership of the actual removed value without cloning it.
    fn displaced(&mut self, attribute: &str, replacement_index: u32, value: Self::Stored);
    /// Returns the values and only the evidence requested by this entry.
    fn finish(self, values: BTreeMap<String, Self::Stored>) -> Self::Output;
}

/// Keeps an original entry index alongside its currently surviving value.
pub(super) struct IndexedAttribute {
    index: u32,
    value: Variant,
}

#[derive(Default)]
pub(super) struct CollectAttributeDiagnostics {
    diagnostics: Vec<AttributeDecodeDiagnostic>,
}

impl AttributeReadSink for CollectAttributeDiagnostics {
    type Stored = IndexedAttribute;
    type Output = AttributesDecodeReport;

    fn store(index: u32, value: Variant) -> IndexedAttribute {
        IndexedAttribute { index, value }
    }

    fn font_fallback(&mut self, index: u32, attribute: &str, fallback: FontFallback) {
        self.diagnostics
            .push(AttributeDecodeDiagnostic::FontFallback {
                attribute: attribute.to_owned(),
                index,
                fallback,
            });
    }

    fn displaced(&mut self, attribute: &str, replacement_index: u32, value: IndexedAttribute) {
        self.diagnostics
            .push(AttributeDecodeDiagnostic::DuplicateEntry {
                attribute: attribute.to_owned(),
                index: value.index,
                replacement_index,
                displaced_value: value.value,
            });
    }

    fn color_sequence_envelope(&mut self, source: ColorSequenceEnvelopeSource<'_>) {
        self.diagnostics
            .push(AttributeDecodeDiagnostic::ColorSequenceEnvelope {
                attribute: source.attribute.to_owned(),
                index: source.index,
                keypoint_index: source.keypoint_index,
                envelope: ColorSequenceEnvelope::from_bits(source.found_bits),
            });
    }

    fn finish(self, values: BTreeMap<String, IndexedAttribute>) -> AttributesDecodeReport {
        let data = values
            .into_iter()
            .map(|(key, stored)| (key, stored.value))
            .collect();
        AttributesDecodeReport::new(Attributes { data }, self.diagnostics)
    }
}

/// Uses the original map value representation and drops displaced values directly.
pub(super) struct DiscardAttributeDiagnostics;

impl AttributeReadSink for DiscardAttributeDiagnostics {
    type Stored = Variant;
    type Output = Attributes;

    fn store(_index: u32, value: Variant) -> Variant {
        value
    }

    fn font_fallback(&mut self, _index: u32, _attribute: &str, _fallback: FontFallback) {}

    fn color_sequence_envelope(&mut self, _source: ColorSequenceEnvelopeSource<'_>) {}

    fn displaced(&mut self, _attribute: &str, _replacement_index: u32, _value: Variant) {}

    fn finish(self, data: BTreeMap<String, Variant>) -> Attributes {
        Attributes { data }
    }
}
