use std::{
    io::Cursor,
    collections::HashMap,
};

use log::trace;

use rbx_tree::{RbxInstanceProperties, RbxTree};

static MODEL_GUI: &str = include_str!("../test-files/gui.rbxmx");
static MODEL_PARTS: &str = include_str!("../test-files/parts.rbxmx");
static MODEL_TERRAIN: &str = include_str!("../test-files/terrain.rbxmx");

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

    for model_source in &[MODEL_TERRAIN] {
        let mut tree = new_test_tree();
        let root_id = tree.get_root_id();

        trace!("Decode:");
        rbx_xml::decode_str(&mut tree, root_id, *model_source).unwrap();

        trace!("Encode:");
        let mut buffer = Vec::new();
        rbx_xml::encode(&tree, &[root_id], Cursor::new(&mut buffer)).unwrap();

        trace!("Decode:");
        rbx_xml::decode(&mut tree, root_id, buffer.as_slice()).unwrap();
    }
}