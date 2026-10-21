use std::convert::TryFrom;

use behave::prelude::*;
use rbx_types::{
    AttributeDecodeDiagnostic, Attributes, BrickColor, Color3, ColorSequence,
    ColorSequenceKeypoint, Font, FontFallback, FontStyle, FontWeight, Variant,
};

fn color_sequence_envelopes(envelopes: [u32; 5]) -> Vec<u8> {
    let mut bytes = vec![2, 0, 0, 0, 1, 0, 0, 0, b'A', 3, 1, 1, 0, 0, 0, b'C', 0x19];
    bytes.extend(5u32.to_le_bytes());
    for (bits, time) in envelopes
        .iter()
        .copied()
        .zip([0.0f32, 0.25, 0.5, 0.75, 1.0])
    {
        bytes.extend(bits.to_le_bytes());
        bytes.extend(time.to_le_bytes());
        for color in [0.25f32, 0.5, 0.75] {
            bytes.extend(color.to_le_bytes());
        }
    }
    bytes
}

fn unknown_font() -> Result<Vec<u8>, std::num::TryFromIntError> {
    let mut bytes = 1u32.to_le_bytes().to_vec();
    bytes.extend(4u32.to_le_bytes());
    bytes.extend(b"Face");
    bytes.push(0x21);
    bytes.extend(123u16.to_le_bytes());
    bytes.push(2);
    let family = "rbxasset://fonts/families/SourceSansPro.json";
    bytes.extend(u32::try_from(family.len())?.to_le_bytes());
    bytes.extend(family.as_bytes());
    bytes.extend(0u32.to_le_bytes());
    Ok(bytes)
}

behave! {
    "Reading serialized color envelopes" {
        "nonzero envelope bits remain evidence without changing runtime keypoints" {
            // @behavior attribute-color-sequence-envelope-evidence
            // @boundary before=negative-zero-finite-and-nonfinite-envelopes at=public-attributes-report after=exact-omitted-bits-and-writer-normalization
            // @red Noncanonical envelope bits decode successfully with no diagnostics and rewrite as positive zero.
            // @green Audited evidence retains each original IEEE pattern and source location; canonical zero adds no loss.
            let patterns = [0, 0x8000_0000, 0x3f80_0000, 0x7f80_0000, 0x7fc1_2345];
            let expected = Attributes::new().with("A", true).with("C", ColorSequence {
                keypoints: [0.0f32, 0.25, 0.5, 0.75, 1.0].iter().copied()
                    .map(|time| ColorSequenceKeypoint::new(time, Color3::new(0.25, 0.5, 0.75))).collect(),
            });
            for envelopes in [[0; 5], patterns] {
                let bytes = color_sequence_envelopes(envelopes);
                let legacy = Attributes::from_reader(bytes.as_slice())?;
                let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
                expect!(&legacy).to_equal(&expected)?;
                expect!(report.attributes()).to_equal(&legacy)?;
                let expected_count = envelopes.iter().filter(|bits| **bits != 0).count();
                expect!(report.diagnostics().len()).to_equal(expected_count)?;
                let mut found = Vec::new();
                for diagnostic in report.diagnostics() {
                    match diagnostic {
                        AttributeDecodeDiagnostic::ColorSequenceEnvelope { attribute, index, keypoint_index, found_bits, normalized_bits } => {
                            found.push((attribute.as_str(), *index, *keypoint_index, *found_bits, *normalized_bits));
                        }
                        found => expect!(format!("{found:?}")).to_equal("a color envelope source distinction".to_owned())?,
                    }
                }
                let expected_evidence: Vec<_> = envelopes.iter().copied().zip(0u32..)
                    .filter(|(bits, _)| *bits != 0)
                    .map(|(bits, keypoint)| ("C", 1u32, keypoint, bits, 0u32)).collect();
                expect!(found).to_equal(expected_evidence)?;
                let mut written = Vec::new();
                report.attributes().to_writer(&mut written)?;
                expect!(written).to_equal(color_sequence_envelopes([0; 5]))?;
            }
        }
    }
}

behave! {
    "Reading repeated attribute entries" {
        "each displaced healthy value keeps its original source entry" {
            // @behavior attribute-duplicate-discard-evidence
            // @boundary before=three-healthy-entries-with-one-name at=public-attributes-report after=actual-displaced-values-and-indices
            // @red Last-value decoding succeeds with no diagnostics for the two displaced values.
            // @green The audited report retains both displaced values without changing the legacy winner.
            let mut bytes = 3u32.to_le_bytes().to_vec();
            for value in [0, 1, 0] {
                bytes.extend([1, 0, 0, 0, b'A', 3, value]);
            }
            let legacy = Attributes::from_reader(bytes.as_slice())?;
            let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
            expect!(report.attributes()).to_equal(&legacy)?;
            expect!(report.attributes().get("A")).to_equal(Some(&Variant::Bool(false)))?;
            expect!(report.diagnostics()).to_equal(&[
                AttributeDecodeDiagnostic::DuplicateEntry { attribute: "A".to_owned(), index: 0, replacement_index: 1, displaced_value: Variant::Bool(false) },
                AttributeDecodeDiagnostic::DuplicateEntry { attribute: "A".to_owned(), index: 1, replacement_index: 2, displaced_value: Variant::Bool(true) },
            ][..])?;
        }
    }
}

behave! {
    "Reading attribute colors" {
        "known colors keep their full declared numbers without diagnostic noise" {
            // @behavior attribute-brick-color-known-value-control
            // @boundary before=known-color-numbers at=public-attributes-readers after=unchanged-known-colors
            // @red An incorrect range repair could reject valid colors or alter their numbers.
            // @green White1 and MediumStoneGrey194 still decode without diagnostics and roundtrip unchanged.
            for (raw, expected) in [(1u32, BrickColor::White), (194, BrickColor::MediumStoneGrey)] {
                let mut bytes = vec![1, 0, 0, 0, 1, 0, 0, 0, b'A', 0x0e];
                bytes.extend(raw.to_le_bytes());
                let legacy = Attributes::from_reader(bytes.as_slice())?;
                let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
                expect!(report.attributes()).to_equal(&legacy)?;
                expect!(report.attributes().get("A")).to_equal(Some(&Variant::BrickColor(expected)))?;
                expect!(report.diagnostics().len()).to_equal(0)?;
                let mut rewritten = Vec::new();
                report.attributes().to_writer(&mut rewritten)?;
                expect!(rewritten).to_equal(bytes)?;
            }
        }

    }
}

behave! {
    "Reading attributes with evidence of substituted Font values" {
        "an unknown weight and style keep their original attribute and discriminators" {
            // @behavior attribute-font-default-substitution-evidence
            // @boundary before=unknown-attribute-font at=public-attributes-reader after=typed-substitution-evidence
            // @red Successful decoding silently rewrites weight123/style2 as400/0.
            // @green Audited decoding retains both source distinctions and native defaults.
            let bytes = unknown_font()?;
            let legacy = Attributes::from_reader(bytes.as_slice())?;
            let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
            expect!(report.attributes()).to_equal(&legacy)?;
            expect!(report.attributes().get("Face")).to_equal(Some(&Variant::Font(Font::default())))?;
            expect!(report.diagnostics()).to_equal(&[
                AttributeDecodeDiagnostic::FontFallback { attribute: "Face".to_owned(), index: 0, fallback: FontFallback::Weight { found: 123, replacement: FontWeight::Regular } },
                AttributeDecodeDiagnostic::FontFallback { attribute: "Face".to_owned(), index: 0, fallback: FontFallback::Style { found: 2, replacement: FontStyle::Normal } },
            ][..])?;
            let (attributes, diagnostics) = report.into_parts();
            expect!(attributes).to_equal(legacy)?;
            expect!(diagnostics.len()).to_equal(2)?;
        }

        "repeated attribute names keep distinct entry indices even when values overwrite" {
            // @behavior attribute-font-repeated-key-evidence
            // @boundary before=repeated-font-attribute-names at=public-attributes-report after=all-source-entry-indices
            // @red The audited Attributes entry does not exist before this change.
            // @green Both occurrences retain their individual serialized entry positions.
            let mut bytes = unknown_font()?;
            bytes[..4].copy_from_slice(&2u32.to_le_bytes());
            bytes.extend_from_within(4..);
            let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
            let positions: Vec<_> = report.diagnostics().iter().filter_map(|diagnostic| match diagnostic {
                AttributeDecodeDiagnostic::FontFallback { attribute, index, .. } => Some((attribute.as_str(), *index)),
                AttributeDecodeDiagnostic::DuplicateEntry { .. } | AttributeDecodeDiagnostic::ColorSequenceEnvelope { .. } => None,
            }).collect();
            expect!(positions).to_equal(vec![("Face", 0), ("Face", 0), ("Face", 1), ("Face", 1)])?;
            expect!(report.attributes().iter().count()).to_equal(1)?;
        }

        "a healthy Font keeps its weight and style without manufacturing a loss" {
            // @behavior attribute-font-no-spurious-loss
            // @boundary before=healthy-font-attribute at=public-attributes-report after=unchanged-value-without-diagnostics
            // @red The audited Attributes entry does not exist before this change.
            // @green Known nondefault Font fields survive and produce no evidence.
            let expected = Attributes::new().with("Face", Font { weight: FontWeight::Bold, style: FontStyle::Italic, ..Font::default() });
            let mut bytes = Vec::new();
            expected.to_writer(&mut bytes)?;
            let report = Attributes::from_reader_with_diagnostics(bytes.as_slice())?;
            expect!(report.diagnostics().len()).to_equal(0)?;
            expect!(report.into_attributes()).to_equal(expected)?;
        }
    }
}
