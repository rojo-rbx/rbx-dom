#![cfg(feature = "serde")]

use behave::prelude::*;
use rbx_types::{Attributes, DiagnosticValue, Variant};
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
    }
}
