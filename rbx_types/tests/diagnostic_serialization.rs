#![cfg(feature = "serde")]

use behave::prelude::*;
use rbx_types::{Attributes, DiagnosticValue, UniqueId, Variant};
use serde_json::json;

behave! {
    "Keeping native values in diagnostic JSON" {
        "nested integers and IEEE patterns survive without changing ordinary value serde" {
            // @behavior native-diagnostic-exact-recursive-values
            // @boundary before=nested-native-values at=public-diagnostic-value-projection after=bits-and-full-decimal-domains
            // @red There is no exact outgoing DiagnosticValue entry.
            // @green One canonical serde traversal retains exceptional numbers recursively, while ordinary serde stays unchanged.
            let value = Variant::Attributes(Attributes::new()
                .with("float", f64::from_bits(0x7ff8_0000_0000_1234))
                .with("nested", Attributes::new().with("integer", 9_007_199_254_740_993i64)));
            let projected = DiagnosticValue::from_variant(&value)?;
            let encoded = serde_json::to_value(&projected)?;
            let text = serde_json::to_string(&encoded)?;
            expect!(text.contains("7ff8000000001234")).to_equal(true)?;
            expect!(text.contains("9007199254740993")).to_equal(true)?;
            let ordinary = serde_json::to_value(&value)?;
            expect!(&ordinary["Attributes"]["float"]).to_equal(&json!({"Float64":null}))?;
            for (bits, expected) in [(0u64, "0000000000000000"), (0x8000_0000_0000_0000, "8000000000000000"), (0x7ff0_0000_0000_0000, "7ff0000000000000")] {
                let value = DiagnosticValue::from_variant(&Variant::Float64(f64::from_bits(bits)))?;
                expect!(serde_json::to_value(value)?).to_equal(json!({"Map":[[{"String":"Float64"},{"F64":expected}]]}))?;
            }
        }

        "attribute diagnostics and value errors serialize the facts their decoder retained" {
            // @behavior native-attribute-diagnostic-outgoing-json
            // @boundary before=attribute-bytes-with-losses-and-failures at=public-attributes-and-value-entries after=exact-diagnostic-and-error-json
            // @red Attribute diagnostics and value errors have no outgoing Serialize implementation.
            // @green Discarded values keep their bits, envelopes keep raw and normalized bits, and errors keep their original kind and facts.
            let mut duplicate = vec![2, 0, 0, 0, 1, 0, 0, 0, b'A', 6];
            duplicate.extend(0x7ff8_0000_0000_1234u64.to_le_bytes());
            duplicate.extend([1, 0, 0, 0, b'A', 3, 1]);
            let mut envelope = vec![1, 0, 0, 0, 1, 0, 0, 0, b'C', 0x19];
            envelope.extend(1u32.to_le_bytes());
            envelope.extend(0x8000_0000u32.to_le_bytes());
            for component in [0.0f32, 0.25, 0.5, 0.75] {
                envelope.extend(component.to_le_bytes());
            }
            let mut diagnostics = Vec::new();
            for bytes in [duplicate, envelope] {
                diagnostics.push(serde_json::to_value(Attributes::from_reader_with_diagnostics(bytes.as_slice())?.diagnostics())?);
            }
            expect!(&diagnostics[0]).to_equal(&json!([{"DuplicateEntry": {"attribute": "A", "index": 0, "replacement_index": 1,
                "displaced_value": {"Map":[[{"String":"Float64"},{"F64":"7ff8000000001234"}]]}}}]))?;
            expect!(&diagnostics[1]).to_equal(&json!([{"ColorSequenceEnvelope": {"attribute": "C", "index": 0, "keypoint_index": 0,
                "envelope": {"bits": 0x8000_0000u32, "normalized_bits": 0}}}]))?;

            let mut brick_color = vec![1, 0, 0, 0, 1, 0, 0, 0, b'A', 0x0e];
            brick_color.extend(65_537u32.to_le_bytes());
            let errors = [
                (brick_color, json!({"Attribute": {"InvalidBrickColor": 65_537}})),
                (vec![1, 0, 0, 0, 1, 0, 0, 0, 255, 3, 1],
                    json!({"Attribute": {"KeyBadUnicode": {"bytes": [255], "cause": {"valid_up_to": "0", "error_len": 1}}}})),
                (vec![1, 0, 0, 0, 1, 0, 0, 0, b'A', 0x14, 0, 0],
                    json!({"Attribute": {"Io": {"kind": "UnexpectedEof", "origin": "Simple"}}})),
            ];
            for (bytes, expected) in errors {
                match Attributes::from_reader(bytes.as_slice()) {
                    Err(error) => expect!(serde_json::to_value(&error)?).to_equal(expected)?,
                    Ok(found) => return Err(format!("expected a refusal, decoded {found:?}").into()),
                }
            }
            match "zz000000000000000000000000000000".parse::<UniqueId>() {
                Err(error) => expect!(serde_json::to_value(&error)?)
                    .to_equal(json!({"UniqueId": {"FromStrParseError": {"err": {"kind": "InvalidDigit"}}}}))?,
                Ok(found) => return Err(format!("expected a refusal, parsed {found:?}").into()),
            }
        }
    }
}
