use std::io::Cursor;

use rbx_dom_weak::{RbxInstanceProperties, RbxValue, RbxTree};

static TEST_FILE: &[u8] = include_bytes!("../test-files/part-referent.rbxmx");

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

    let first_tree = rbx_xml::from_reader(TEST_FILE).unwrap();
    let root_id = first_tree.get_root_id();

    assert_referents_sound(&first_tree);

    let root_instance = first_tree.get_instance(root_id).unwrap();
    let model_id = root_instance.get_children_ids()[0];

    let mut buffer = Vec::new();
    rbx_xml::to_writer(&first_tree, &[model_id], Cursor::new(&mut buffer)).unwrap();

    let second_tree = rbx_xml::from_reader(buffer.as_slice()).unwrap();
    let new_root_id = second_tree.get_root_id();

    assert_referents_sound(&second_tree);
}