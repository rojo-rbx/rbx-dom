use std::convert::TryFrom;

use behave::prelude::*;
use rbx_types::{
    AttributeDecodeDiagnostic, Attributes, Font, FontFallback, FontStyle, FontWeight, Variant,
};

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
            let positions: Vec<_> = report.diagnostics().iter().map(|diagnostic| match diagnostic {
                AttributeDecodeDiagnostic::FontFallback { attribute, index, .. } => (attribute.as_str(), *index),
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
