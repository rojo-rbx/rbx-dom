use std::{fs, path::Path};

use rbx_dom_test::{InstanceBuilder, TreeViewer};

use rbx_binary::{decode, encode};

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

    let mut initial_tree = InstanceBuilder::new("Folder").build();

    let root_id = initial_tree.get_root_id();
    decode(&mut initial_tree, root_id, initial_contents.as_slice()).unwrap();

    let initial_viewed = TreeViewer::new().view_children(&initial_tree);
    insta::assert_yaml_snapshot!(test_name, initial_viewed);

    let root_instance = initial_tree.get_instance(root_id).unwrap();
    let root_children = root_instance.get_children_ids();
    let mut roundtrip_contents = Vec::new();
    encode(&initial_tree, root_children, &mut roundtrip_contents).unwrap();

    let mut roundtrip_tree = InstanceBuilder::new("Folder").build();

    let root_id = roundtrip_tree.get_root_id();
    decode(&mut roundtrip_tree, root_id, roundtrip_contents.as_slice()).unwrap();

    let roundtrip_viewed = TreeViewer::new().view_children(&roundtrip_tree);
    insta::assert_yaml_snapshot!(test_name, roundtrip_viewed);
}
