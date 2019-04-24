use rbx_dom_weak::{RbxInstanceProperties, RbxValue, RbxTree};

static TEST_FILE: &[u8] = include_bytes!("../test-files/number-value-int32.rbxmx");

fn new_test_tree() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: Default::default(),
    };

    RbxTree::new(root)
}

#[test]
fn f32_to_f64() {
    let _ = env_logger::try_init();

    let mut tree = new_test_tree();
    let root_id = tree.get_root_id();

    rbx_xml::decode(&mut tree, root_id, TEST_FILE).unwrap();

    let root_instance = tree.get_instance(root_id).unwrap();
    let value_id = root_instance.get_children_ids()[0];
    let value = tree.get_instance(value_id).unwrap();

    assert_eq!(value.name, "A NumberValue");
    assert_eq!(value.class_name, "NumberValue");
    assert_eq!(value.properties.get("Value"), Some(&RbxValue::Float64 { value: 308.0 }));
}