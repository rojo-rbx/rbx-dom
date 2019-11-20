use std::{collections::HashMap, fs, io::Cursor, path::Path};

use log::trace;

use rbx_dom_test::TreeViewer;
use rbx_dom_weak::{RbxInstanceProperties, RbxTree};

#[test]
fn just_folder() {
    test_round_trip("just-folder.rbxm");
}

#[test]
fn folder_with_script() {
    test_round_trip("folder-with-script.rbxm");
}

#[test]
fn hierarchy_with_refs() {
    test_round_trip("hierarchy-with-refs.rbxm");
}

/// Helper method to test both reading and writing using only rbx_binary.
fn test_round_trip(test_name: &str) {
    let _ = env_logger::try_init();

    let input_path = Path::new("test-files").join(test_name);
    let initial_contents = fs::read(&input_path).unwrap();

    let mut initial_tree = new_test_tree();

    let root_id = initial_tree.get_root_id();
    rbx_binary::decode(&mut initial_tree, root_id, initial_contents.as_slice()).unwrap();

    let initial_viewed = TreeViewer::new().view_children(&initial_tree);
    insta::assert_yaml_snapshot!(test_name, initial_viewed);

    let root_instance = initial_tree.get_instance(root_id).unwrap();
    let root_children = root_instance.get_children_ids();
    let mut roundtrip_contents = Vec::new();
    rbx_binary::encode(&initial_tree, root_children, &mut roundtrip_contents).unwrap();

    let mut roundtrip_tree = new_test_tree();

    let root_id = roundtrip_tree.get_root_id();
    rbx_binary::decode(&mut roundtrip_tree, root_id, roundtrip_contents.as_slice()).unwrap();

    let roundtrip_viewed = TreeViewer::new().view_children(&roundtrip_tree);
    insta::assert_yaml_snapshot!(test_name, roundtrip_viewed);
}

fn new_test_tree() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: HashMap::new(),
    };

    RbxTree::new(root)
}
