#![cfg(feature = "serde")]

use behave::prelude::*;
use rbx_binary::{DecodeDiagnostic, Deserializer};
use serde_json::json;

#[path = "support/native_model.rs"]
mod native_model;

use native_model::{strings, Model};

behave! {
    "Sending native decode evidence as JSON" {
        "a real decode retains ordered locations and exact discarded values in outgoing evidence" {
            // @behavior native-diagnostic-outgoing-json
            // @boundary before=native-bytes-with-losses at=public-audited-decoder-and-serde after=ordered-exact-native-evidence-json
            // @red DecodeDiagnostic has no outgoing Serialize implementation.
            // @green Original diagnostics serialize without a DOM clone, source-error replacement or numeric normalization.
            let mut model = Model::new("Folder")?;
            model.chunk([b'X', 255, b'Y', 0], &[0, 255, 7])?;
            model.property("Missing", &[])?;
            model.property("Name", &strings(&[b'A', 255], &[b'B', 0xe2, 0x82])?)?;
            let mut attributes = vec![2, 0, 0, 0, 1, 0, 0, 0, b'A', 6];
            attributes.extend(0x7ff8_0000_0000_1234u64.to_le_bytes());
            attributes.extend([1, 0, 0, 0, b'A', 3, 1]);
            model.property("AttributesSerialize", &strings(&attributes, &0u32.to_le_bytes())?)?;
            let bytes = model.finish()?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            let encoded = serde_json::to_value(report.diagnostics())?;
            expect!(encoded.as_array().map(Vec::len)).to_equal(Some(5))?;
            expect!(&encoded[0]).to_equal(&json!({"UnknownChunk": {"name": [88, 255, 89, 0], "data": [0, 255, 7]}}))?;
            expect!(&encoded[1]["PropertySkipped"]["property"]).to_equal(&json!({"class_name":"Folder", "property_name":"Missing", "type_id":7}))?;
            expect!(&encoded[1]["PropertySkipped"]["reason"]).to_equal(&json!("MissingType"))?;
            for (position, offset, length) in [(2, "1", Some(1)), (4, "1", None)] {
                expect!(&encoded[position]["InvalidUtf8"]["cause"]).to_equal(&json!({"valid_up_to":offset,"error_len":length}))?;
            }
            expect!(&encoded[3]["AttributeLoss"]["diagnostic"]["DuplicateEntry"]["displaced_value"])
                .to_equal(&json!({"Map":[[{"String":"Float64"},{"F64":"7ff8000000001234"}]]}))?;
            for (position, diagnostic) in report.diagnostics().iter().enumerate().skip(2) {
                let (name, instance) = match diagnostic {
                    DecodeDiagnostic::InvalidUtf8 { instance, .. } => ("InvalidUtf8", instance),
                    DecodeDiagnostic::AttributeLoss { instance, .. } => ("AttributeLoss", instance),
                    found => return Err(format!("unexpected per-instance evidence: {found:?}").into()),
                };
                expect!(&encoded[position][name]["instance"]).to_equal(&json!({"document_referent":instance.document_referent(), "dom_ref":instance.dom_ref()}))?;
                expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?;
            }
            #[cfg(feature = "typescript")]
            {
                let envelope = serde_json::to_value(rbx_binary::ColorSequenceEnvelope::from_bits(0x8000_0000))?;
                let declarations = declarations();
                let mut keys = Vec::new();
                object_keys(&encoded, &mut keys);
                object_keys(&envelope, &mut keys);
                let undeclared: Vec<&String> = keys.iter().filter(|key| !declarations.contains(key.as_str())).collect();
                expect!(undeclared.is_empty()).to_equal(true)?;
            }
        }
    }
}

/// Every object key the encoded evidence writes, at any depth.
#[cfg(feature = "typescript")]
fn object_keys(value: &serde_json::Value, keys: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, nested) in map {
                keys.push(key.clone());
                object_keys(nested, keys);
            }
        }
        serde_json::Value::Array(items) => items.iter().for_each(|item| object_keys(item, keys)),
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => {}
    }
}

/// The generated declarations of the outgoing diagnostic and every leaf it writes.
#[cfg(feature = "typescript")]
fn declarations() -> String {
    use rbx_binary::{
        ColorSequenceEnvelope, InstanceLocation, PropertyLocation, PropertySkipReason,
    };
    use rbx_dom_weak::types::{AttributeDecodeDiagnostic, DiagnosticValue, Utf8ErrorFacts};
    use ts_rs::{Config, TS};

    let config = Config::new();
    [
        DecodeDiagnostic::decl(&config),
        InstanceLocation::decl(&config),
        PropertyLocation::decl(&config),
        PropertySkipReason::decl(&config),
        AttributeDecodeDiagnostic::decl(&config),
        DiagnosticValue::decl(&config),
        Utf8ErrorFacts::decl(&config),
        ColorSequenceEnvelope::decl(&config),
    ]
    .join("\n")
}
