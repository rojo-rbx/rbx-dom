use std::io::{Cursor, Write};

use rbx_archive::{ContentEnvelope, Limits, Reader};
use rbx_dom_weak::{
    types::{BinaryString, Content, ContentType, SharedString, Variant},
    InstanceBuilder, WeakDom,
};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..32].to_owned()
}
fn binary(dom: &WeakDom) -> Vec<u8> {
    let mut scene = Vec::new();
    rbx_binary::Serializer::new()
        .compression_type(rbx_binary::CompressionType::None)
        .serialize(&mut scene, dom, dom.root().children())
        .unwrap();
    scene
}
fn dom(children: Vec<InstanceBuilder>) -> WeakDom {
    WeakDom::new(InstanceBuilder::new("DataModel").with_children(children))
}
fn envelope(payload: &[u8], metadata: &[u8]) -> SharedString {
    let mut bytes = 0i32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&(payload.len() as i32).to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(&(metadata.len() as i32).to_le_bytes());
    bytes.extend_from_slice(metadata);
    SharedString::new(bytes)
}
fn archive(entries: Vec<(String, Vec<u8>)>) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, data) in entries {
        writer
            .start_file(
                name,
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored),
            )
            .unwrap();
        writer.write_all(&data).unwrap();
    }
    writer.finish().unwrap().into_inner()
}
fn container(scene: &[u8], objects: &[&[u8]]) -> Vec<u8> {
    let root = digest(scene);
    let mut manifest = format!("{{\"kind\":\"header\",\"version\":1,\"digestAlgorithm\":\"blake3-128\"}}\n{{\"kind\":\"package\",\"type\":\"place\",\"place\":\"nested/scene.rbxl\"}}\n{{\"kind\":\"content\",\"path\":\"nested/scene.rbxl\",\"objectDigest\":\"{root}\"}}\n");
    let mut entries = vec![(format!("objects/{root}"), scene.to_vec())];
    for object in objects {
        let id = digest(object);
        manifest += &format!("{{\"kind\":\"content\",\"objectDigest\":\"{id}\"}}\n");
        entries.push((format!("objects/{id}"), object.to_vec()));
    }
    entries.push(("manifest.jsonl".into(), manifest.into_bytes()));
    archive(entries)
}
fn add_metadata(scene: &[u8], name: &str, metadata: &[u8], value_type: Option<u8>) -> Vec<u8> {
    let mut result = scene[..32].to_vec();
    let mut offset = 32;
    while offset < scene.len() {
        assert_eq!(&scene[offset + 4..offset + 8], &[0; 4]);
        let len = u32::from_le_bytes(scene[offset + 8..offset + 12].try_into().unwrap()) as usize;
        let mut body = scene[offset + 16..offset + 16 + len].to_vec();
        if &scene[offset..offset + 4] == b"PROP" {
            let name_len = u32::from_le_bytes(body[4..8].try_into().unwrap()) as usize;
            if &body[8..8 + name_len] == name.as_bytes() {
                if let Some(ty) = value_type {
                    body[8 + name_len] = ty;
                }
                body.splice(
                    8 + name_len..8 + name_len,
                    [vec![0x82], metadata.to_vec()].concat(),
                );
            }
        }
        result.extend_from_slice(&scene[offset..offset + 8]);
        result.extend_from_slice(&(body.len() as u32).to_le_bytes());
        result.extend_from_slice(&[0; 4]);
        result.extend_from_slice(&body);
        offset += 16 + len;
    }
    result
}
fn reference(bytes: &[u8], identity: Option<&str>) -> Vec<u8> {
    let mut result = vec![if identity.is_some() { 2 } else { 1 }];
    result.extend_from_slice(&blake3::hash(bytes).as_bytes()[..16]);
    result.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    if let Some(identity) = identity {
        result.extend_from_slice(&(identity.len() as u32).to_le_bytes());
        result.extend_from_slice(identity.as_bytes());
    }
    [(result.len() as i32).to_le_bytes().to_vec(), result].concat()
}
fn property<'a>(dom: &'a WeakDom, instance_name: &str, property: &str) -> &'a Variant {
    &dom.descendants()
        .find(|instance| instance.name == instance_name)
        .unwrap()
        .properties[&property.into()]
}

#[test]
fn inline_dom_round_trip() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "print('archive')")
    ]);
    let mut encoded = Cursor::new(Vec::new());
    rbx_archive::to_writer(&mut encoded, &original, original.root().children()).unwrap();
    let decoded = rbx_archive::from_reader(Cursor::new(encoded.into_inner())).unwrap();
    assert_eq!(
        property(&decoded, "Script", "Source"),
        &Variant::String("print('archive')".into())
    );
}

#[test]
fn reads_embedded_source_with_manifest_last() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "")
    ]);
    let source = b"print('embedded')\n";
    let scene = add_metadata(&binary(&original), "Source", &reference(source, None), None);
    assert!(rbx_binary::from_reader(scene.as_slice()).is_err());
    let decoded = rbx_archive::from_reader(Cursor::new(container(&scene, &[source]))).unwrap();
    assert_eq!(
        property(&decoded, "Script", "Source"),
        &Variant::String(String::from_utf8(source.to_vec()).unwrap())
    );
}

#[test]
fn reads_binary_and_shared_strings() {
    for shared in [false, true] {
        let empty: Variant = if shared {
            SharedString::new(Vec::new()).into()
        } else {
            BinaryString::from(Vec::new()).into()
        };
        let original = dom(vec![
            InstanceBuilder::new("Folder").with_property("Bytes", empty)
        ]);
        let object = b"\xff\0\x80binary";
        let scene = add_metadata(&binary(&original), "Bytes", &reference(object, None), None);
        let decoded = rbx_archive::from_reader(Cursor::new(container(&scene, &[object]))).unwrap();
        let expected: Variant = if shared {
            SharedString::new(object.to_vec()).into()
        } else {
            BinaryString::from(object.to_vec()).into()
        };
        assert_eq!(property(&decoded, "Folder", "Bytes"), &expected);
    }
}

#[test]
fn opaque_round_trip_preserves_identity_and_envelope() {
    let bytes = envelope(b"hello", br#"{"INTERNAL_TYPE":"Text"}"#);
    let original = dom(vec![
        InstanceBuilder::new("TextDocument")
            .with_name("a")
            .with_property(
                "TextContent",
                Content::from_embedded("rbxruntime://A".into(), bytes.clone()),
            ),
        InstanceBuilder::new("TextDocument")
            .with_name("b")
            .with_property(
                "TextContent",
                Content::from_embedded("rbxruntime://A".into(), bytes.clone()),
            ),
        InstanceBuilder::new("TextDocument")
            .with_name("c")
            .with_property(
                "TextContent",
                Content::from_embedded("rbxruntime://B".into(), bytes.clone()),
            ),
        InstanceBuilder::new("TextDocument")
            .with_name("uri")
            .with_property("TextContent", Content::from_uri("rbxassetid://42")),
        InstanceBuilder::new("TextDocument")
            .with_name("none")
            .with_property("TextContent", Content::none()),
    ]);
    let mut encoded = Cursor::new(Vec::new());
    rbx_archive::to_writer(&mut encoded, &original, original.root().children()).unwrap();
    let mut reader = Reader::new(Cursor::new(encoded.into_inner()), Limits::default()).unwrap();
    assert_eq!(
        reader.manifest().content.len(),
        2,
        "identical envelopes should share one object"
    );
    let decoded = reader.deserialize().unwrap();
    for name in ["a", "b", "c", "uri", "none"] {
        assert_eq!(
            property(&decoded, name, "TextContent"),
            property(&original, name, "TextContent")
        );
    }
    let Variant::Content(content) = property(&decoded, "a", "TextContent") else {
        panic!()
    };
    let ContentType::Embedded(content) = content.value() else {
        panic!()
    };
    assert_eq!(
        ContentEnvelope::from_bytes(content.data.data())
            .unwrap()
            .payload,
        b"hello"
    );
    assert!(rbx_binary::to_writer(Vec::new(), &decoded, decoded.root().children()).is_err());
    assert!(rbx_xml::to_writer(
        Vec::new(),
        &decoded,
        decoded.root().children(),
        rbx_xml::EncodeOptions::new()
            .property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown)
    )
    .is_err());
}

#[test]
fn sparse_metadata_skips_inline_values() {
    let original = dom(vec![
        InstanceBuilder::new("Script")
            .with_name("inline")
            .with_property("Source", "kept"),
        InstanceBuilder::new("Script")
            .with_name("embedded")
            .with_property("Source", ""),
    ]);
    let source = b"external";
    let metadata = [(-1i32).to_le_bytes().to_vec(), reference(source, None)].concat();
    let scene = add_metadata(&binary(&original), "Source", &metadata, None);
    let decoded = rbx_archive::from_reader(Cursor::new(container(&scene, &[source]))).unwrap();
    assert_eq!(
        property(&decoded, "inline", "Source"),
        &Variant::String("kept".into())
    );
    assert_eq!(
        property(&decoded, "embedded", "Source"),
        &Variant::String("external".into())
    );
}

#[test]
fn malformed_references_are_errors() {
    let source = b"external";
    for (placeholder, metadata, object) in [
        ("occupied", reference(source, None), source.as_slice()),
        ("", (-2i32).to_le_bytes().to_vec(), source.as_slice()),
        ("", reference(b"other", None), source.as_slice()),
        (
            "",
            reference(source, Some("rbxruntime://A")),
            source.as_slice(),
        ),
    ] {
        let original = dom(vec![
            InstanceBuilder::new("Script").with_property("Source", placeholder)
        ]);
        let scene = add_metadata(&binary(&original), "Source", &metadata, None);
        assert!(rbx_archive::from_reader(Cursor::new(container(&scene, &[object]))).is_err());
    }
}

#[test]
fn limits_fail_before_object_restoration() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "")
    ]);
    let source = b"external";
    let scene = add_metadata(&binary(&original), "Source", &reference(source, None), None);
    let encoded = container(&scene, &[source]);
    let mut limits = Limits::default();
    limits.max_archive_bytes = encoded.len() as u64 - 1;
    assert!(Reader::new(Cursor::new(&encoded), limits).is_err());
    let mut limits = Limits::default();
    limits.max_entries = 2;
    assert!(Reader::new(Cursor::new(&encoded), limits).is_err());
    let mut limits = Limits::default();
    limits.max_restored_bytes = 3;
    assert!(Reader::new(Cursor::new(&encoded), limits)
        .unwrap()
        .deserialize()
        .is_err());
}

#[test]
fn copy_preserves_aliases_and_metadata_spelling() {
    let original = dom(vec![InstanceBuilder::new("Folder")]);
    let scene = binary(&original);
    let id = digest(&scene);
    let manifest = format!("{{\"kind\":\"header\",\"version\":1,\"digestAlgorithm\":\"blake3-128\"}}\n{{\"kind\":\"package\",\"type\":\"place\",\"place\":\"scene\"}}\n{{\"kind\":\"content\",\"path\":\"scene\",\"objectDigest\":\"{id}\",\"metadata\":{{\"value\":1e+02}}}}\n{{\"kind\":\"content\",\"path\":\"alias\",\"objectDigest\":\"{id}\"}}\n");
    let encoded = archive(vec![
        ("manifest.jsonl".into(), manifest.as_bytes().to_vec()),
        (format!("objects/{id}"), scene),
    ]);
    let mut reader = Reader::new(Cursor::new(encoded), Limits::default()).unwrap();
    let mut copied = Cursor::new(Vec::new());
    reader.copy_to(&mut copied).unwrap();
    let copy = Reader::new(Cursor::new(copied.into_inner()), Limits::default()).unwrap();
    assert_eq!(copy.manifest().as_bytes(), manifest.as_bytes());
}

#[test]
fn malformed_manifests_are_rejected() {
    let scene = binary(&dom(vec![]));
    let id = digest(&scene);
    let header = "{\"kind\":\"header\",\"version\":1,\"digestAlgorithm\":\"blake3-128\"}\n";
    let package = "{\"kind\":\"package\",\"type\":\"place\",\"place\":\"scene\"}\n";
    let content =
        format!("{{\"kind\":\"content\",\"path\":\"scene\",\"objectDigest\":\"{id}\"}}\n");
    for manifest in [
        format!("{header}{package}{content}{content}"),
        format!("{header}{package}\n{content}"),
        format!("{header}{package}{content}").replace("\"scene\"", "\"../scene\""),
        format!("{header}{package}{content}")
            .replace("\"version\":1", "\"version\":1,\"version\":1"),
        format!("{header}{package}{content}").replace("\"type\":\"place\"", "\"type\":\"model\""),
        format!("{header}{package}{content}").replace(
            &format!("\"{id}\""),
            &format!("\"{id}\",\"metadata\":{{\"x\":1,\"x\":2}}"),
        ),
    ] {
        let bytes = archive(vec![
            ("manifest.jsonl".into(), manifest.into_bytes()),
            (format!("objects/{id}"), scene.clone()),
        ]);
        assert!(Reader::new(Cursor::new(bytes), Limits::default()).is_err());
    }
}

#[test]
fn object_digest_mismatch_and_extra_members_are_rejected() {
    let scene = binary(&dom(vec![]));
    let encoded = container(&scene, &[]);
    let mut reader = Reader::new(Cursor::new(encoded), Limits::default()).unwrap();
    let manifest = reader.manifest().as_bytes().to_vec();
    let digest = reader.manifest().scene_digest().to_owned();
    assert!(Reader::new(
        Cursor::new(archive(vec![
            ("manifest.jsonl".into(), manifest.clone()),
            (format!("objects/{digest}"), b"wrong".to_vec())
        ])),
        Limits::default()
    )
    .is_err());
    let scene = reader.read_object(&digest).unwrap();
    assert!(Reader::new(
        Cursor::new(archive(vec![
            ("manifest.jsonl".into(), manifest),
            (format!("objects/{digest}"), scene),
            ("extra".into(), vec![])
        ])),
        Limits::default()
    )
    .is_err());
}

#[test]
fn opaque_envelope_validation() {
    let valid = envelope(b"", br#"{"INTERNAL_TYPE":"Text"}"#);
    assert!(ContentEnvelope::from_bytes(valid.data()).is_ok());
    for invalid in [
        envelope(b"", b""),
        envelope(b"x", br#"{"x":1}"#),
        envelope(b"x", br#"{"x":"a","x":"b"}"#),
        SharedString::new(vec![0; 4]),
        SharedString::new([valid.data(), b"x"].concat()),
    ] {
        assert!(ContentEnvelope::from_bytes(invalid.data()).is_err());
    }
}

#[test]
fn conflicting_opaque_identity_is_rejected_by_writer() {
    let original = dom(vec![
        InstanceBuilder::new("TextDocument").with_property(
            "TextContent",
            Content::from_embedded("rbxruntime://same".into(), envelope(b"a", b"")),
        ),
        InstanceBuilder::new("TextDocument").with_property(
            "TextContent",
            Content::from_embedded("rbxruntime://same".into(), envelope(b"b", b"")),
        ),
    ]);
    assert!(rbx_archive::to_writer(
        Cursor::new(Vec::new()),
        &original,
        original.root().children()
    )
    .is_err());
}

fn rewrite_equal_length(bytes: &mut [u8], old: &[u8], new: &[u8]) {
    assert_eq!(old.len(), new.len());
    let mut index = 0;
    while index + old.len() <= bytes.len() {
        if &bytes[index..index + old.len()] == old {
            bytes[index..index + old.len()].copy_from_slice(new);
            index += old.len();
        } else {
            index += 1;
        }
    }
}

#[test]
fn duplicate_physical_names_are_rejected() {
    let scene = binary(&dom(vec![]));
    let a = b"first";
    let b = b"second";
    let mut bytes = container(&scene, &[a, b]);
    rewrite_equal_length(
        &mut bytes,
        format!("objects/{}", digest(b)).as_bytes(),
        format!("objects/{}", digest(a)).as_bytes(),
    );
    assert!(matches!(
        Reader::new(Cursor::new(bytes), Limits::default()),
        Err(rbx_archive::Error::Invalid("duplicate ZIP member"))
    ));
}

#[test]
fn unsupported_zip_features_are_rejected() {
    let scene = binary(&dom(vec![]));
    for encrypted in [false, true] {
        let mut bytes = container(&scene, &[]);
        let mut index = 0;
        while index + 46 < bytes.len() {
            let fields = match &bytes[index..index + 4] {
                b"PK\x03\x04" => Some((index + 6, index + 8)),
                b"PK\x01\x02" => Some((index + 8, index + 10)),
                _ => None,
            };
            if let Some((flags, method)) = fields {
                if encrypted {
                    bytes[flags] |= 1;
                } else {
                    bytes[method] = 8;
                }
            }
            index += 1;
        }
        assert!(Reader::new(Cursor::new(bytes), Limits::default()).is_err());
    }
}

#[test]
fn zip64_local_entries_are_supported() {
    let scene = binary(&dom(vec![]));
    let encoded = container(&scene, &[]);
    let reader = Reader::new(Cursor::new(encoded), Limits::default()).unwrap();
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .large_file(true);
    zip.start_file("manifest.jsonl", options).unwrap();
    zip.write_all(reader.manifest().as_bytes()).unwrap();
    zip.start_file(format!("objects/{}", digest(&scene)), options)
        .unwrap();
    zip.write_all(&scene).unwrap();
    rbx_archive::from_reader(Cursor::new(zip.finish().unwrap().into_inner())).unwrap();
}

#[test]
fn string_and_shared_string_wire_encodings_restore_source() {
    let original = dom(vec![InstanceBuilder::new("Script")
        .with_property("Source", "")
        .with_property("EmptyShared", SharedString::new(Vec::new()))]);
    let source = b"print('shared source')";
    for ty in [1, 28] {
        let scene = add_metadata(
            &binary(&original),
            "Source",
            &reference(source, None),
            Some(ty),
        );
        let decoded = rbx_archive::from_reader(Cursor::new(container(&scene, &[source]))).unwrap();
        assert_eq!(
            property(&decoded, "Script", "Source"),
            &Variant::String(String::from_utf8(source.to_vec()).unwrap())
        );
    }
}

#[test]
fn malformed_opaque_properties_are_rejected() {
    let valid = envelope(b"hello", b"{}");
    let invalid = envelope(b"hello", br#"{"type":1}"#);
    for (content, identity, bytes) in [
        (Content::none(), "http://invalid", valid.data()),
        (Content::from_uri(""), "rbxruntime://A", valid.data()),
        (Content::none(), "rbxruntime://A", invalid.data()),
    ] {
        let original = dom(vec![
            InstanceBuilder::new("TextDocument").with_property("TextContent", content)
        ]);
        let scene = add_metadata(
            &binary(&original),
            "TextContent",
            &reference(bytes, Some(identity)),
            None,
        );
        assert!(rbx_archive::from_reader(Cursor::new(container(&scene, &[bytes]))).is_err());
    }
}

#[test]
fn reader_rejects_conflicting_opaque_identity() {
    let a = envelope(b"a", b"");
    let b = envelope(b"b", b"");
    let original = dom(vec![
        InstanceBuilder::new("TextDocument").with_property("TextContent", Content::none()),
        InstanceBuilder::new("TextDocument").with_property("TextContent", Content::none()),
    ]);
    let metadata = [
        reference(a.data(), Some("rbxruntime://same")),
        reference(b.data(), Some("rbxruntime://same")),
    ]
    .concat();
    let scene = add_metadata(&binary(&original), "TextContent", &metadata, None);
    assert!(
        rbx_archive::from_reader(Cursor::new(container(&scene, &[a.data(), b.data()]))).is_err()
    );
}

#[test]
fn manifest_text_validation() {
    let scene = binary(&dom(vec![]));
    let encoded = container(&scene, &[]);
    let reader = Reader::new(Cursor::new(encoded), Limits::default()).unwrap();
    let valid = String::from_utf8(reader.manifest().as_bytes().to_vec()).unwrap();
    for manifest in [
        format!("\u{feff}{valid}"),
        format!("{valid}\0"),
        valid.replace("\"path\":\"nested/scene.rbxl\"", "\"path\":null"),
        valid.replace(
            "\"path\":\"nested/scene.rbxl\"",
            "\"path\":\"nested/scene.rbxl\",\"metadata\":null",
        ),
        valid.replace(
            "\"path\":\"nested/scene.rbxl\"",
            "\"path\":\"nested/scene.rbxl\",\"metadata\":{\"a\":1,\"\\u0061\":2}",
        ),
        valid.replace(
            "\"path\":\"nested/scene.rbxl\"",
            &format!(
                "\"path\":\"nested/scene.rbxl\",\"metadata\":{{\"deep\":{}0{}}}",
                "[".repeat(16),
                "]".repeat(16)
            ),
        ),
    ] {
        assert!(Reader::new(
            Cursor::new(archive(vec![
                ("manifest.jsonl".into(), manifest.into_bytes()),
                (format!("objects/{}", digest(&scene)), scene.clone())
            ])),
            Limits::default()
        )
        .is_err());
    }
}

#[test]
fn writer_limits_are_enforced() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "source")
    ]);
    for limit in ["scene", "archive", "manifest", "entries", "total"] {
        let mut limits = Limits::default();
        match limit {
            "scene" => limits.max_object_bytes = 1,
            "archive" => limits.max_archive_bytes = 1,
            "manifest" => limits.max_manifest_bytes = 1,
            "entries" => limits.max_entries = 1,
            "total" => limits.max_total_object_bytes = 1,
            _ => unreachable!(),
        }
        assert!(rbx_archive::Writer::new(limits)
            .serialize(
                Cursor::new(Vec::new()),
                &original,
                original.root().children()
            )
            .is_err());
    }
}

#[test]
fn unreferenced_objects_are_validated_without_restoring_envelopes() {
    let scene = binary(&dom(vec![InstanceBuilder::new("Folder")]));
    let arbitrary = b"not an envelope";
    let bytes = container(&scene, &[arbitrary]);
    let mut limits = Limits::default();
    limits.max_restored_bytes = 0;
    let mut reader = Reader::new(Cursor::new(bytes), limits).unwrap();
    reader.deserialize().unwrap();
    assert_eq!(reader.read_object(&digest(arbitrary)).unwrap(), arbitrary);
}

#[test]
fn empty_embedded_metadata_preserves_inline_binary_behavior() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "inline")
    ]);
    let scene = add_metadata(&binary(&original), "Source", &0i32.to_le_bytes(), None);
    let decoded = rbx_binary::from_reader(scene.as_slice()).unwrap();
    assert_eq!(
        property(&decoded, "Script", "Source"),
        &Variant::String("inline".into())
    );
}

#[test]
fn stored_zip_data_descriptors_are_supported_and_checked() {
    let scene = binary(&dom(vec![]));
    let reader = Reader::new(Cursor::new(container(&scene, &[])), Limits::default()).unwrap();
    for zip64 in [false, true] {
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .large_file(zip64);
        let mut zip = ZipWriter::new_stream(Vec::new());
        zip.start_file("manifest.jsonl", options).unwrap();
        zip.write_all(reader.manifest().as_bytes()).unwrap();
        zip.start_file(format!("objects/{}", digest(&scene)), options)
            .unwrap();
        zip.write_all(&scene).unwrap();
        let mut bytes = zip.finish().unwrap().into_inner();
        Reader::new(Cursor::new(&bytes), Limits::default()).unwrap();
        let descriptor = bytes
            .windows(4)
            .position(|bytes| bytes == b"PK\x07\x08")
            .unwrap();
        bytes[descriptor + 4] ^= 1;
        assert!(Reader::new(Cursor::new(bytes), Limits::default()).is_err());
    }
}

#[test]
fn missing_physical_entries_are_rejected() {
    let scene = binary(&dom(vec![]));
    let mut bytes = container(&scene, &[]);
    let central = bytes
        .windows(4)
        .position(|bytes| bytes == b"PK\x01\x02")
        .unwrap();
    let first_name_len =
        u16::from_le_bytes(bytes[central + 28..central + 30].try_into().unwrap()) as usize;
    let extra_len =
        u16::from_le_bytes(bytes[central + 30..central + 32].try_into().unwrap()) as usize;
    let comment_len =
        u16::from_le_bytes(bytes[central + 32..central + 34].try_into().unwrap()) as usize;
    let first_entry_len = 46 + first_name_len + extra_len + comment_len;
    bytes.drain(central..central + first_entry_len);
    let eocd = bytes.len() - 22;
    bytes[eocd + 8..eocd + 10].copy_from_slice(&1u16.to_le_bytes());
    bytes[eocd + 10..eocd + 12].copy_from_slice(&1u16.to_le_bytes());
    let old_size = u32::from_le_bytes(bytes[eocd + 12..eocd + 16].try_into().unwrap());
    bytes[eocd + 12..eocd + 16].copy_from_slice(&(old_size - first_entry_len as u32).to_le_bytes());
    assert!(Reader::new(Cursor::new(bytes), Limits::default()).is_err());
}

#[test]
fn binary_configurations_remain_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<rbx_binary::Deserializer<'static>>();
    assert_send_sync::<rbx_binary::Serializer<'static>>();
}

#[test]
fn resolver_is_scoped_to_one_decode_and_inline_values_do_not_invoke_it() {
    use std::{cell::Cell, io};
    struct Resolver {
        calls: Cell<usize>,
    }
    impl rbx_binary::EmbeddedContentResolver for Resolver {
        fn resolve(&self, _: &rbx_binary::EmbeddedReference) -> io::Result<SharedString> {
            self.calls.set(self.calls.get() + 1);
            Ok(SharedString::new(b"source".to_vec()))
        }
    }
    let decoder = rbx_binary::Deserializer::new();
    let resolver = Resolver {
        calls: Cell::new(0),
    };
    let inline = binary(&dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "")
    ]));
    decoder
        .deserialize_with_resolver(inline.as_slice(), &resolver)
        .unwrap();
    assert_eq!(resolver.calls.get(), 0);
    let embedded = add_metadata(&inline, "Source", &reference(b"source", None), None);
    let restored = decoder
        .deserialize_with_resolver(embedded.as_slice(), &resolver)
        .unwrap();
    assert_eq!(
        property(&restored, "Script", "Source"),
        &Variant::String("source".into())
    );
    assert_eq!(resolver.calls.get(), 1);
    assert!(decoder.deserialize(embedded.as_slice()).is_err());
    assert_eq!(resolver.calls.get(), 1);
}

#[test]
fn existing_asset_content_map_values_survive_archive_round_trip() {
    let mut map = rbx_dom_weak::types::AssetContentMap::new();
    map.set_bundle_asset_id(Some(123));
    map.insert("fragment".into(), vec![0, 255, 1].into());
    let original = dom(vec![
        InstanceBuilder::new("Terrain").with_property("VoxelGridAssetContentMap", map.clone())
    ]);
    let mut encoded = Cursor::new(Vec::new());
    rbx_archive::to_writer(&mut encoded, &original, original.root().children()).unwrap();
    let decoded = rbx_archive::from_reader(Cursor::new(encoded.into_inner())).unwrap();
    assert_eq!(
        property(&decoded, "Terrain", "VoxelGridAssetContentMap"),
        &Variant::AssetContentMap(map)
    );
}

#[test]
fn mixed_opaque_and_object_content_preserves_referents() {
    let target = InstanceBuilder::new("Folder").with_name("target");
    let target_ref = target.referent();
    let original = dom(vec![
        target,
        InstanceBuilder::new("TextDocument")
            .with_name("object")
            .with_property("TextContent", Content::from_referent(target_ref)),
        InstanceBuilder::new("TextDocument")
            .with_name("opaque")
            .with_property(
                "TextContent",
                Content::from_embedded("rbxruntime://A".into(), envelope(b"text", b"")),
            ),
    ]);
    let mut encoded = Cursor::new(Vec::new());
    rbx_archive::to_writer(&mut encoded, &original, original.root().children()).unwrap();
    let decoded = rbx_archive::from_reader(Cursor::new(encoded.into_inner())).unwrap();
    let Variant::Content(content) = property(&decoded, "object", "TextContent") else {
        panic!()
    };
    assert_eq!(
        decoded
            .get_by_ref(content.as_object().unwrap())
            .unwrap()
            .name,
        "target"
    );
}

#[test]
fn duplicate_property_columns_are_rejected() {
    let original = dom(vec![
        InstanceBuilder::new("Script").with_property("Source", "")
    ]);
    let scene = binary(&original);
    let mut offset = 32;
    let mut duplicate = None;
    let mut end = None;
    while offset < scene.len() {
        let len = u32::from_le_bytes(scene[offset + 8..offset + 12].try_into().unwrap()) as usize;
        if &scene[offset..offset + 4] == b"PROP" {
            duplicate = Some(scene[offset..offset + 16 + len].to_vec());
        }
        if &scene[offset..offset + 4] == b"END\0" {
            end = Some(offset);
        }
        offset += 16 + len;
    }
    let mut scene = scene;
    scene.splice(end.unwrap()..end.unwrap(), duplicate.unwrap());
    assert!(rbx_archive::from_reader(Cursor::new(container(&scene, &[]))).is_err());
}
