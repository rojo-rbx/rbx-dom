use std::io::Cursor;

use rbx_dom_weak::{RbxInstanceProperties, RbxValue, RbxTree};

static TEST_FILE: &[u8] = include_bytes!("../test-files/part-referent.rbxmx");

fn new_test_tree() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: Default::default(),
    };

    RbxTree::new(root)
}

fn assert_referents_sound(tree: &RbxTree) {
    let root_id = tree.get_root_id();

    let root_instance = tree.get_instance(root_id).unwrap();

    let model_id = root_instance.get_children_ids()[0];
    let model_instance = tree.get_instance(model_id).unwrap();

    let part_id = model_instance.get_children_ids()[0];

    let primary_part = model_instance.properties.get("PrimaryPart");
    let expected_value = RbxValue::Ref {
        value: Some(part_id),
    };

    assert_eq!(primary_part, Some(&expected_value));
}

#[test]
fn referents_work() {
    let _ = env_logger::try_init();

    let mut first_tree = new_test_tree();
    let root_id = first_tree.get_root_id();

    rbx_xml::decode(&mut first_tree, root_id, TEST_FILE).unwrap();

    assert_referents_sound(&first_tree);

    let root_instance = first_tree.get_instance(root_id).unwrap();
    let model_id = root_instance.get_children_ids()[0];

    let mut buffer = Vec::new();
    rbx_xml::encode(&first_tree, &[model_id], Cursor::new(&mut buffer)).unwrap();

    let mut second_tree = new_test_tree();
    let new_root_id = second_tree.get_root_id();

    rbx_xml::decode(&mut second_tree, new_root_id, buffer.as_slice()).unwrap();

    assert_referents_sound(&second_tree);
}