use rbx_dom_weak::{RbxInstanceProperties, RbxValue, RbxTree};

static TEST_FILE: &[u8] = include_bytes!("../test-files/fire.rbxmx");

fn new_test_tree() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: Default::default(),
    };

    RbxTree::new(root)
}

#[test]
fn fire_weird_properties() {
    let _ = env_logger::try_init();

    let mut tree = new_test_tree();
    let root_id = tree.get_root_id();

    rbx_xml::decode(&mut tree, root_id, TEST_FILE).unwrap();

    let root_instance = tree.get_instance(root_id).unwrap();
    let fire_id = root_instance.get_children_ids()[0];
    let fire = tree.get_instance(fire_id).unwrap();

    assert_eq!(fire.name, "Fire");
    assert_eq!(fire.class_name, "Fire");
    assert_eq!(fire.properties.get("size_xml"), None);
    assert_eq!(fire.properties.get("heat_xml"), None);
    assert_eq!(fire.properties.get("Size"), Some(&RbxValue::Float32 { value: 2.0 }));
    assert_eq!(fire.properties.get("Heat"), Some(&RbxValue::Float32 { value: 3.0 }));
}