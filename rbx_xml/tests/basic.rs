//! Temporary tests while re-bootstrapping rbx_xml

use rbx_dom_weak::types::{Tags, Variant};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

#[test]
fn with_bool() {
    let _ = env_logger::try_init();

    let document = r#"
        <roblox version="4">
            <Item class="BoolValue" referent="hello">
                <Properties>
                    <bool name="Value">true</bool>
                </Properties>
            </Item>
        </roblox>
    "#;

    let tree = rbx_xml::from_str_default(document).unwrap();

    let root = tree.root();
    let child = tree.get_by_ref(root.children()[0]).unwrap();

    assert_eq!(child.name, "BoolValue");
    assert_eq!(child.class, "BoolValue");
    assert_eq!(child.properties.get("Value"), Some(&Variant::Bool(true)));
}

#[test]
fn empty_tags() {
    let _ = env_logger::try_init();

    let part = InstanceBuilder::new("Part").with_property("Tags", Tags::new());
    let dom = WeakDom::new(part);

    let mut encoded = Vec::new();
    rbx_xml::to_writer_default(&mut encoded, &dom, &[dom.root_ref()]).unwrap();
    insta::assert_snapshot!(std::str::from_utf8(&encoded).unwrap());
}
