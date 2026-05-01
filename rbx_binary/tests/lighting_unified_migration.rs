//! End-to-end tests for the class-level inject mechanism applied to
//! `Lighting.Attributes`. These verify that:
//!
//! - For a Lighting instance with no Attributes set in memory, the
//!   serializer force-emits Attributes filled from Studio's captured default.
//! - For a Lighting instance with user-set Attributes, the user's keys are
//!   preserved and the captured default's keys are merged in (user wins on
//!   collision).
//! - For non-inject classes (e.g. Folder), nothing is force-emitted.
//!
//! Specific marker values come from the captured Studio default (see
//! `Lighting.DefaultProperties.Attributes` in the regenerated database), so
//! the tests assert presence/precedence rather than hardcoded values that
//! could drift against future Studio releases. The single hardcoded value
//! (`RBX_LightingTechnologyUnifiedMigration: Bool(true)`) is the actual
//! marker that suppresses Roblox's unified-lighting migration on file load —
//! that one is invariant across Studio versions.

use rbx_dom_weak::types::{Attributes, Variant};
use rbx_dom_weak::{ustr, InstanceBuilder, WeakDom};

fn round_trip_lighting(lighting_builder: InstanceBuilder) -> rbx_dom_weak::UstrMap<Variant> {
    let dom = WeakDom::new(InstanceBuilder::new("DataModel").with_child(lighting_builder));
    let mut buf = Vec::new();
    let root_children = dom.root().children().to_vec();
    rbx_binary::to_writer(&mut buf, &dom, &root_children).expect("to_writer");

    let decoded = rbx_binary::from_reader(buf.as_slice()).expect("from_reader");
    let lighting = decoded
        .descendants()
        .find(|i| i.class == ustr("Lighting"))
        .expect("Lighting in round-tripped dom");
    lighting.properties.clone()
}

fn extract_attributes(props: &rbx_dom_weak::UstrMap<Variant>) -> &Attributes {
    match props.get(&ustr("Attributes")) {
        Some(Variant::Attributes(a)) => a,
        Some(other) => panic!("Attributes property is not Variant::Attributes: {:?}", other),
        None => panic!("Lighting is missing Attributes property after round-trip"),
    }
}

#[test]
fn lighting_with_no_attributes_synthesizes_markers() {
    let props = round_trip_lighting(InstanceBuilder::new("Lighting"));
    let attrs = extract_attributes(&props);

    // The unified-lighting marker is what actually suppresses Studio's reset
    // of LightingStyle to Soft on file load.
    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
    // The companion marker should also be present (value sourced from Studio).
    assert!(
        matches!(
            attrs.get("RBX_OriginalTechnologyOnFileLoad"),
            Some(Variant::Int32(_))
        ),
        "RBX_OriginalTechnologyOnFileLoad should be present and an Int32",
    );
}

#[test]
fn lighting_with_existing_attributes_preserves_user_keys() {
    let mut existing = Attributes::new();
    existing.insert("UserKey".to_string(), Variant::Bool(true));
    let props = round_trip_lighting(
        InstanceBuilder::new("Lighting").with_property("Attributes", existing),
    );
    let attrs = extract_attributes(&props);

    assert_eq!(
        attrs.get("UserKey"),
        Some(&Variant::Bool(true)),
        "user-set key should round-trip",
    );
    // Studio markers should be merged in alongside the user key.
    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
}

#[test]
fn instance_set_marker_overrides_default() {
    // Pick a marker value that's distinct from Studio's default so we can
    // observe precedence. RBX_OriginalTechnologyOnFileLoad is Int32; Studio
    // writes a small value (currently 2) — using 99 ensures we'd catch a
    // wrong-direction merge regardless of Studio's exact default.
    let mut existing = Attributes::new();
    existing.insert(
        "RBX_OriginalTechnologyOnFileLoad".to_string(),
        Variant::Int32(99),
    );
    let props = round_trip_lighting(
        InstanceBuilder::new("Lighting").with_property("Attributes", existing),
    );
    let attrs = extract_attributes(&props);

    assert_eq!(
        attrs.get("RBX_OriginalTechnologyOnFileLoad"),
        Some(&Variant::Int32(99)),
        "instance-set value should win against the default during inject merge",
    );
    // The other marker should still be filled in.
    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
}

#[test]
fn non_inject_class_unaffected() {
    let dom = WeakDom::new(
        InstanceBuilder::new("DataModel").with_child(InstanceBuilder::new("Folder")),
    );
    let mut buf = Vec::new();
    let root_children = dom.root().children().to_vec();
    rbx_binary::to_writer(&mut buf, &dom, &root_children).expect("to_writer");

    let decoded = rbx_binary::from_reader(buf.as_slice()).expect("from_reader");
    let folder = decoded
        .descendants()
        .find(|i| i.class == ustr("Folder"))
        .expect("Folder in dom");
    assert!(
        folder.properties.get(&ustr("Attributes")).is_none(),
        "Folder should not have synthesized Attributes (no Inject for that class)",
    );
}
