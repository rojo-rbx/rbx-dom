use std::{
    io::Cursor,
    collections::HashMap,
};

use log::info;

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

    for (index, model_source) in [MODEL_TERRAIN, MODEL_GUI, MODEL_PARTS].iter().enumerate() {
        let mut tree = new_test_tree();
        let root_id = tree.get_root_id();

        info!("Decode #{}:", index);
        rbx_xml::decode_str(&mut tree, root_id, *model_source).unwrap();

        info!("Encode #{}:", index);
        let mut buffer = Vec::new();
        rbx_xml::encode(&tree, &[root_id], Cursor::new(&mut buffer)).unwrap();

        info!("Re-Decode #{}:", index);
        rbx_xml::decode(&mut tree, root_id, buffer.as_slice()).unwrap();
    }
}