use std::{
    io::Cursor,
    collections::HashMap,
};

use log::info;
use rbx_dom_weak::{RbxInstanceProperties, RbxTree};

static TEST_MODELS: &[&str] = &[
    include_str!("../test-files/baseplate.rbxlx"),
    include_str!("../test-files/decals.rbxmx"),
    include_str!("../test-files/effects.rbxmx"),
    include_str!("../test-files/gui.rbxmx"),
    include_str!("../test-files/inf-and-nan.rbxmx"),
    include_str!("../test-files/numbers.rbxmx"),
    include_str!("../test-files/part-referent.rbxmx"),
    include_str!("../test-files/parts.rbxmx"),
    include_str!("../test-files/terrain.rbxmx"),
];

fn new_test_tree() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: HashMap::new(),
    };

    RbxTree::new(root)
}

#[test]
fn round_trip() {
    let _ = env_logger::try_init();

    for (index, model_source) in TEST_MODELS.iter().enumerate() {
        let mut tree = new_test_tree();
        let root_id = tree.get_root_id();

        info!("Decoding #{}...", index);
        rbx_xml::decode_str(&mut tree, root_id, *model_source).unwrap();

        info!("Encoding #{}...", index);
        let mut buffer = Vec::new();
        rbx_xml::encode(&tree, &[root_id], Cursor::new(&mut buffer)).unwrap();

        info!("Re-Decoding #{}...", index);
        rbx_xml::decode(&mut tree, root_id, buffer.as_slice()).unwrap();
    }
}