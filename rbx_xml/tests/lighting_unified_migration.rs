//! End-to-end tests for the class-level inject mechanism in the XML format,
//! mirroring the binary tests so any drift between the two surfaces fast.

use rbx_dom_weak::types::{Attributes, Variant};
use rbx_dom_weak::{ustr, InstanceBuilder, WeakDom};

fn round_trip_lighting(lighting_builder: InstanceBuilder) -> rbx_dom_weak::UstrMap<Variant> {
    let dom = WeakDom::new(InstanceBuilder::new("DataModel").with_child(lighting_builder));
    let mut buf = Vec::new();
    let root_children = dom.root().children().to_vec();
    rbx_xml::to_writer_default(&mut buf, &dom, &root_children).expect("to_writer");

    let decoded = rbx_xml::from_reader_default(buf.as_slice()).expect("from_reader");
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

    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
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

    assert_eq!(attrs.get("UserKey"), Some(&Variant::Bool(true)));
    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
}

#[test]
fn instance_set_marker_overrides_default() {
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
    assert_eq!(
        attrs.get("RBX_LightingTechnologyUnifiedMigration"),
        Some(&Variant::Bool(true)),
    );
}
