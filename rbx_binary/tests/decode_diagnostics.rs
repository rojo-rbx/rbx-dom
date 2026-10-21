use std::convert::TryFrom;

use behave::prelude::*;
use rbx_binary::{
    BinaryStringFallbackCause, DecodeDiagnostic, DecodeReport, Deserializer, FontFallback,
    InstanceLocation, PropertyLocation, PropertySkipReason, ReferenceKind,
};
use rbx_dom_weak::types::VariantType;
use rbx_dom_weak::types::{
    AttributeDecodeDiagnostic, Attributes, Content, Font, FontStyle, FontWeight, Ref, Variant,
};
use rbx_reflection::{
    ClassDescriptor, DataType, MigrationError, PropertyDescriptor, PropertyKind,
    PropertySerialization, ReflectionDatabase,
};

#[path = "support/native_model.rs"]
mod native_model;

use native_model::{interleaved, referents, strings, Model};

behave! {
    "Reading binary models with retained loss evidence" {
        "Font substitutions inside successful Attributes keep the native instance identity" {
            // @behavior native-nested-attribute-font-evidence
            // @boundary before=successful-attributes-font-fallback at=public-native-report after=nested-source-evidence
            // @red The successful Attributes decode leaves the native report empty.
            // @green Both substitutions carry their attribute and native instance location.
            let mut attributes = 1u32.to_le_bytes().to_vec();
            attributes.extend(4u32.to_le_bytes());
            attributes.extend(b"Face");
            attributes.push(0x21);
            attributes.extend(123u16.to_le_bytes());
            attributes.push(2);
            let family = "rbxasset://fonts/families/SourceSansPro.json";
            attributes.extend(u32::try_from(family.len())?.to_le_bytes());
            attributes.extend(family.as_bytes());
            attributes.extend(0u32.to_le_bytes());
            let mut model = Model::new("Folder")?;
            model.property("AttributesSerialize", &strings(&attributes, &0u32.to_le_bytes())?)?;
            let bytes = model.finish()?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            expect!(report.diagnostics().len()).to_equal(2)?;
            let mut nested = Vec::new();
            for diagnostic in report.diagnostics() { match diagnostic {
                DecodeDiagnostic::AttributeLoss { property, instance, diagnostic } => {
                    expect!(property.property_name()).to_equal("AttributesSerialize")?;
                    expect!(property.class_name()).to_equal("Folder")?;
                    expect!(property.type_id()).to_equal(7)?;
                    expect!(instance.document_referent()).to_equal(10)?;
                    expect!(report.dom().root().children().first()).to_equal(Some(&instance.dom_ref()))?;
                    nested.push(diagnostic.clone());
                }
                found => expect!(format!("{found:?}")).to_equal("nested Attributes evidence".to_owned())?,
            } }
            expect!(nested).to_equal(vec![
                AttributeDecodeDiagnostic::FontFallback { attribute: "Face".to_owned(), index: 0, fallback: FontFallback::Weight { found: 123, replacement: FontWeight::Regular } },
                AttributeDecodeDiagnostic::FontFallback { attribute: "Face".to_owned(), index: 0, fallback: FontFallback::Style { found: 2, replacement: FontStyle::Normal } },
            ])?;
            let legacy = rbx_binary::from_reader(bytes.as_slice())?;
            for dom in [report.dom(), &legacy] {
                let expected = [Attributes::new().with("Face", Font::default()), Attributes::new()];
                for (referent, attributes) in dom.root().children().iter().zip(expected) {
                    expect!(dom.get_by_ref(*referent).and_then(|node| node.properties.get(&"Attributes".into())))
                        .to_equal(Some(&Variant::Attributes(attributes)))?;
                }
            }
        }

        "unknown font weights and styles retain their raw discriminators instead of silently becoming defaults" {
            // @behavior native-font-default-substitution-evidence
            // @boundary before=unknown-font-weight-and-style at=public-native-decoder after=typed-source-discriminators-and-preserved-defaults
            // @red Invalid weight123/style2 decoded with an empty diagnostic slice.
            // @green Both substitutions retain their source identity and typed replacement.
            let mut model = Model::new("Folder")?;
            let mut fonts = vec![0x20];
            let family = "rbxasset://fonts/families/SourceSansPro.json";
            for (weight, style) in [(123u16, 2u8), (400, 0)] {
                fonts.extend(u32::try_from(family.len())?.to_le_bytes());
                fonts.extend(family.as_bytes());
                fonts.extend(weight.to_le_bytes());
                fonts.push(style);
                fonts.extend(0u32.to_le_bytes());
            }
            model.property("Typeface", &fonts)?;
            let bytes = model.finish()?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            expect!(report.diagnostics().len()).to_equal(2)?;
            match report.diagnostics() {
                [DecodeDiagnostic::FontFallback { property, instance, fallback: FontFallback::Weight { found: 123, replacement: FontWeight::Regular } },
                 DecodeDiagnostic::FontFallback { property: style_property, instance: style_instance, fallback: FontFallback::Style { found: 2, replacement: FontStyle::Normal } }] => {
                    expect!(property).to_equal(style_property)?;
                    expect!(property.property_name()).to_equal("Typeface")?;
                    expect!(property.class_name()).to_equal("Folder")?;
                    expect!(property.type_id()).to_equal(7)?;
                    expect!(instance).to_equal(style_instance)?;
                    expect!(instance.document_referent()).to_equal(10)?;
                    expect!(report.dom().root().children().first()).to_equal(Some(&instance.dom_ref()))?;
                }
                found => expect!(format!("{found:?}")).to_equal("weight and style source evidence".to_owned())?,
            }
            let expected = Variant::Font(Font::regular(family));
            let legacy = rbx_binary::from_reader(bytes.as_slice())?;
            for dom in [report.dom(), &legacy] {
                for referent in dom.root().children() {
                    expect!(dom.get_by_ref(*referent).and_then(|node| node.properties.get(&"Typeface".into())))
                        .to_equal(Some(&expected))?;
                }
            }
        }

        "valid native string values do not manufacture a loss" {
            // @behavior native-decode-no-spurious-loss
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut model = Model::new("StringValue")?;
            model.property("Name", &strings(b"same", b"same")?)?;
            model.property("Value", &strings(b"first", b"second")?)?;
            let bytes = model.finish()?;
            let decoded: Result<DecodeReport, rbx_binary::DecodeError> =
                Deserializer::new().deserialize_with_diagnostics(bytes.as_slice());
            let report = decoded?;
            expect!(report.diagnostics().len()).to_equal(0)?;
            let nodes: Vec<_> = report.dom().root().children().iter()
                .filter_map(|id| report.dom().get_by_ref(*id)).collect();
            expect!(nodes.len()).to_equal(2)?;
            expect!(nodes.iter().map(|node| node.name.as_str()).collect::<Vec<_>>()).to_equal(vec!["same", "same"])?;
            expect!(nodes.iter().map(|node| node.properties.get(&"Value".into())).collect::<Vec<_>>())
                .to_equal(vec![Some(&Variant::String("first".into())), Some(&Variant::String("second".into()))])?;
        }

        "an unknown chunk and every skipped property retain their original locations and bytes" {
            // @behavior native-decode-skipped-chunks-properties
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut model = Model::new("Folder")?;
            model.chunk([0xff, 0, 1, 2], &[9, 8, 7])?;
            model.property("Future", &[])?;
            model.property("First", &[0xff, 4, 5])?;
            model.property("Second", &[0xff, 6, 7])?;
            let bytes = model.finish()?;
            let legacy = rbx_binary::from_reader(bytes.as_slice())?;
            expect!(legacy.root().children().len()).to_equal(2)?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            expect!(report.dom().root().children().len()).to_equal(2)?;
            expect!(report.diagnostics().len()).to_equal(4)?;
            match report.diagnostics() {
                [DecodeDiagnostic::UnknownChunk { name, data },
                 DecodeDiagnostic::PropertySkipped { property: missing, instances, reason: PropertySkipReason::MissingType, data: empty },
                 DecodeDiagnostic::PropertySkipped { property: first_property, reason: PropertySkipReason::UnknownType(0xff), data: first, .. },
                 DecodeDiagnostic::PropertySkipped { property: second_property, reason: PropertySkipReason::UnknownType(0xff), data: second, .. }] => {
                    expect!(*name).to_equal([0xff, 0, 1, 2])?;
                    expect!(data.as_slice()).to_equal(&[9, 8, 7][..])?;
                    let missing: &PropertyLocation = missing;
                    expect!(missing.class_name()).to_equal("Folder")?;
                    expect!(missing.property_name()).to_equal("Future")?;
                    expect!(missing.type_id()).to_equal(7)?;
                    expect!(instances.iter().map(|instance| instance.document_referent()).collect::<Vec<_>>()).to_equal(vec![10, 20])?;
                    for instance in instances { expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?; }
                    expect!(empty.len()).to_equal(0)?;
                    expect!(first.as_slice()).to_equal(&[4, 5][..])?;
                    expect!(second.as_slice()).to_equal(&[6, 7][..])?;
                    expect!(first_property.property_name()).to_equal("First")?;
                    expect!(second_property.property_name()).to_equal("Second")?;
                }
                found => expect!(format!("{found:?}")).to_equal("all four source losses".to_owned())?,
            }
            let (dom, diagnostics): (rbx_dom_weak::WeakDom, Vec<DecodeDiagnostic>) = report.into_parts();
            expect!(dom.root().children().len()).to_equal(2)?;
            expect!(diagnostics.len()).to_equal(4)?;
        }

        "invalid names and string values retain each document referent and the unreplaced bytes" {
            // @behavior native-decode-utf8-source-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut model = Model::new("StringValue")?;
            model.property("Name", &strings(b"\xff", b"\xfe")?)?;
            model.property("Value", &strings(b"\xf0\x80", b"valid")?)?;
            let bytes = model.finish()?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            expect!(report.diagnostics().len()).to_equal(3)?;
            let mut found = Vec::new();
            for diagnostic in report.diagnostics() {
                match diagnostic {
                    DecodeDiagnostic::InvalidUtf8 { property, instance, bytes, cause } => {
                        let instance: &InstanceLocation = instance;
                        found.push((property.property_name(), instance.document_referent(), bytes.as_slice()));
                        expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?;
                        expect!(cause.valid_up_to()).to_equal(0)?;
                    }
                    found => expect!(format!("{found:?}")).to_equal("a UTF-8 replacement".to_owned())?,
                }
            }
            expect!(found).to_equal(vec![("Name", 10, &b"\xff"[..]), ("Value", 10, &b"\xf0\x80"[..]), ("Name", 20, &b"\xfe"[..])])?;
            let legacy = rbx_binary::from_reader(bytes.as_slice())?;
            let dom = report.into_dom();
            for (audited, permissive) in dom.root().children().iter().zip(legacy.root().children()) {
                expect!(dom.get_by_ref(*audited).map(|node| (&node.name, &node.properties)))
                    .to_equal(legacy.get_by_ref(*permissive).map(|node| (&node.name, &node.properties)))?;
            }
        }

        "reflection skips retain serialized property identity and the bytes they skipped" {
            // @behavior native-decode-reflection-skip-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut database = ReflectionDatabase::new();
            let mut class = ClassDescriptor::new("Folder");
            let mut descriptor = PropertyDescriptor::new("Canonical", DataType::Value(VariantType::Bool));
            descriptor.kind = PropertyKind::Canonical { serialization: PropertySerialization::DoesNotSerialize };
            class.properties.insert("Canonical", descriptor);
            let mut alias = PropertyDescriptor::new("Saved", DataType::Value(VariantType::Bool));
            alias.kind = PropertyKind::Alias { alias_for: "Canonical" };
            class.properties.insert("Saved", alias);
            database.classes.insert("Folder", class);
            let mut model = Model::new("Folder")?;
            model.property("Saved", &[2, 0, 1])?;
            let bytes = model.finish()?;
            let report = Deserializer::new().reflection_database(&database).deserialize_with_diagnostics(bytes.as_slice())?;
            match report.diagnostics() {
                [DecodeDiagnostic::PropertySkipped { property, instances, reason: PropertySkipReason::DoesNotSerialize { canonical_property, binary_type }, data }] => {
                    expect!(property.property_name()).to_equal("Saved")?;
                    expect!(canonical_property.as_str()).to_equal("Canonical")?;
                    expect!(*binary_type).to_equal(2)?;
                    expect!(instances.iter().map(|instance| instance.document_referent()).collect::<Vec<_>>()).to_equal(vec![10, 20])?;
                    for instance in instances { expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?; }
                    expect!(data.as_slice()).to_equal(&[0, 1][..])?;
                }
                found => expect!(format!("{found:?}")).to_equal("one intentional property skip".to_owned())?,
            }
        }

        "failed property migrations retain their typed cause for every instance" {
            // @behavior native-decode-migration-failure-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut model = Model::new("TextLabel")?;
            let mut values = vec![0x12];
            // Enum values are interleaved without the signed transform.
            values.extend([0, 0, 0, 0, 3, 3, 0xe7, 0xe7]);
            model.property("Font", &values)?;
            let bytes = model.finish()?;
            let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
            expect!(report.diagnostics().len()).to_equal(2)?;
            for (diagnostic, expected_referent) in report.diagnostics().iter().zip([10, 20]) {
                match diagnostic {
                    DecodeDiagnostic::MigrationFailed { property, instance, canonical_property, cause: MigrationError::InvalidValueForMigration { actual, .. } } => {
                        expect!(property.property_name()).to_equal("Font")?;
                        expect!(canonical_property.as_str()).to_equal("Font")?;
                        expect!(instance.document_referent()).to_equal(expected_referent)?;
                        expect!(actual.clone()).to_equal(Variant::Enum(rbx_dom_weak::types::Enum::from_u32(999)))?;
                        expect!(report.dom().get_by_ref(instance.dom_ref()).map(|node| node.class.as_str())).to_equal(Some("TextLabel"))?;
                    }
                    found => expect!(format!("{found:?}")).to_equal("a typed invalid font migration".to_owned())?,
                }
            }
        }

        "semantic binary fallbacks keep the source buffer and the native decode error" {
            // @behavior native-decode-semantic-binary-fallback-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            for (class, property) in [("Folder", "AttributesSerialize"), ("Terrain", "MaterialColors")] {
                let mut model = Model::new(class)?;
                model.property(property, &strings(&[0xff], &[0xfe])?)?;
                let bytes = model.finish()?;
                let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
                expect!(report.diagnostics().len()).to_equal(2)?;
                for (diagnostic, (expected_referent, expected_bytes)) in report.diagnostics().iter().zip([(10, &[0xff][..]), (20, &[0xfe][..])]) {
                    match diagnostic {
                        DecodeDiagnostic::BinaryStringFallback { property: location, instance, bytes, cause } => {
                            let cause: &BinaryStringFallbackCause = cause;
                            expect!(location.property_name()).to_equal(property)?;
                            expect!(instance.document_referent()).to_equal(expected_referent)?;
                            expect!(bytes.as_slice()).to_equal(expected_bytes)?;
                            expect!(std::error::Error::source(cause).is_some()).to_equal(true)?;
                            expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?;
                        }
                        found => expect!(format!("{found:?}")).to_equal("a semantic binary fallback".to_owned())?,
                    }
                }
            }
        }

        "ignored content external data and instances absent from the parent table are visible" {
            // @behavior native-decode-external-content-and-unparented-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            let mut model = Model::new("Folder")?;
            let mut content = vec![0x22];
            content.extend(interleaved(&[0, 0]));
            content.extend(0u32.to_le_bytes());
            content.extend(0u32.to_le_bytes());
            content.extend(1u32.to_le_bytes());
            let external_data = referents(&[88]);
            content.extend(&external_data);
            model.property("Target", &content)?;
            // Neither INST node is in a PRNT entry; END still succeeds in the
            // legacy decoder, which returns an empty DataModel.
            model.chunk(*b"END\0", b"</roblox>")?;
            let report = Deserializer::new().deserialize_with_diagnostics(model.bytes.as_slice())?;
            expect!(report.dom().root().children().len()).to_equal(0)?;
            match report.diagnostics() {
                [DecodeDiagnostic::ExternalContentReferencesIgnored { property, count, data, .. },
                 DecodeDiagnostic::UnparentedInstance { instance: first, class_name: first_class },
                 DecodeDiagnostic::UnparentedInstance { instance: second, class_name: second_class }] => {
                    expect!(property.property_name()).to_equal("Target")?;
                    expect!(*count).to_equal(1)?;
                    expect!(data).to_equal(&external_data)?;
                    expect!(first.document_referent()).to_equal(10)?;
                    expect!(second.document_referent()).to_equal(20)?;
                    expect!(report.dom().get_by_ref(first.dom_ref()).is_none()).to_equal(true)?;
                    expect!(report.dom().get_by_ref(second.dom_ref()).is_none()).to_equal(true)?;
                    expect!(first_class.as_str()).to_equal("Folder")?;
                    expect!(second_class.as_str()).to_equal("Folder")?;
                }
                found => expect!(format!("{found:?}")).to_equal("external data and both unparented instances".to_owned())?,
            }
        }

        "missing targets stay distinct from null in both direct and content object references" {
            // @behavior native-decode-nonnull-unresolved-reference-evidence
            // @boundary before=native-source-bytes at=public-deserializer after=dom-with-structured-source-losses
            // @red The diagnostics entry and its exported evidence types are absent.
            // @green The same native pass retains the specific source facts below.
            for kind in [ReferenceKind::Ref, ReferenceKind::ContentObject] {
                for targets in [[-1, 999], [20, 999]] {
                    let mut model = Model::new("Folder")?;
                    let mut property = match kind {
                        ReferenceKind::Ref => vec![0x13],
                        ReferenceKind::ContentObject => {
                            let mut data = vec![0x22];
                            data.extend(interleaved(&[2, 2]));
                            data.extend(0u32.to_le_bytes());
                            data.extend(2u32.to_le_bytes());
                            data
                        }
                    };
                    property.extend(referents(&targets));
                    if kind == ReferenceKind::ContentObject {
                        property.extend(0u32.to_le_bytes());
                    }
                    model.property("Target", &property)?;
                    let bytes = model.finish()?;
                    let report = Deserializer::new().deserialize_with_diagnostics(bytes.as_slice())?;
                    match report.diagnostics() {
                        [DecodeDiagnostic::UnresolvedReference { property, instance, target, kind: found_kind }] => {
                            expect!(property.property_name()).to_equal("Target")?;
                            expect!(instance.document_referent()).to_equal(20)?;
                            expect!(*target).to_equal(999)?;
                            expect!(*found_kind).to_equal(kind)?;
                            expect!(report.dom().get_by_ref(instance.dom_ref()).is_some()).to_equal(true)?;
                        }
                        found => expect!(format!("{found:?}")).to_equal("one non-null missing target".to_owned())?,
                    }
                    let nodes: Vec<_> = report.dom().root().children().iter()
                        .filter_map(|id| report.dom().get_by_ref(*id)).collect();
                    let expected_null = match kind {
                        ReferenceKind::Ref => Variant::Ref(Ref::none()),
                        ReferenceKind::ContentObject => Variant::Content(Content::none()),
                    };
                    expect!(nodes.get(1).and_then(|node| node.properties.get(&"Target".into())))
                        .to_equal(Some(&expected_null))?;
                    let expected_first = match (targets[0], kind) {
                        (-1, _) => expected_null,
                        (_, ReferenceKind::Ref) => Variant::Ref(report.dom().root().children()[1]),
                        (_, ReferenceKind::ContentObject) => Variant::Content(Content::from_referent(report.dom().root().children()[1])),
                    };
                    expect!(nodes.first().and_then(|node| node.properties.get(&"Target".into())))
                        .to_equal(Some(&expected_first))?;
                }
            }
        }
    }
}
