use std::{
    io::Cursor,
    collections::HashMap,
};

use rbx_tree::{RbxInstance, RbxTree};

static MODEL_A: &[u8] = include_bytes!("../test-files/model-a.rbxm");
static MODEL_B: &[u8] = include_bytes!("../test-files/model-b.rbxm");
static MODEL_C: &[u8] = include_bytes!("../test-files/model-c.rbxm");

fn new_test_tree() -> RbxTree {
    let root = RbxInstance {
        name: "Folder".to_string(),
        class_name: "Folder".to_string(),
        properties: HashMap::new(),
    };

    RbxTree::new(root)
}

#[test]
fn round_trip() {
    for model_source in &[MODEL_A, MODEL_B, MODEL_C] {
        let mut tree = new_test_tree();
        let root_id = tree.get_root_id();

        println!("Decode:");
        rbx_binary::decode(&mut tree, root_id, *model_source).unwrap();

        println!("Encode:");
        let mut buffer = Vec::new();
        rbx_binary::encode(&tree, &[root_id], Cursor::new(&mut buffer)).unwrap();

        println!("Decode:");
        rbx_binary::decode(&mut tree, root_id, Cursor::new(&buffer)).unwrap();
    }
}