use std::{
    collections::HashMap,
    mem::size_of,
};

use rbx_dom_weak::{RbxTree, RbxInstanceProperties};

fn new_data_model() -> RbxTree {
    let root = RbxInstanceProperties {
        name: "DataModel".to_string(),
        class_name: "DataModel".to_string(),
        properties: HashMap::new(),
    };

    RbxTree::new(root)
}

#[test]
fn errors_are_small() {
    assert!(size_of::<rbx_xml::DecodeError>() <= 8);
    assert!(size_of::<rbx_xml::EncodeError>() <= 8);
}

#[test]
fn first_line_bad_xml() {
    let doc = "hi";

    let mut tree = new_data_model();
    let root_id = tree.get_root_id();

    let err = rbx_xml::decode_str(&mut tree, root_id, doc).unwrap_err();

    assert_eq!(err.line(), 1);
    assert_eq!(err.column(), 0);
}

#[test]
fn bad_version() {
    let doc = r#"<roblox version="3"></roblox>"#;

    let mut tree = new_data_model();
    let root_id = tree.get_root_id();

    let err = rbx_xml::decode_str(&mut tree, root_id, doc).unwrap_err();

    assert_eq!(err.line(), 1);
    assert_eq!(err.column(), 0);
}

#[test]
fn second_line_gunk() {
    let doc = r#"<roblox version="4">
        <asfk />
    </roblox>"#;

    let mut tree = new_data_model();
    let root_id = tree.get_root_id();

    let err = rbx_xml::decode_str(&mut tree, root_id, doc).unwrap_err();

    assert_eq!(err.line(), 2);
    assert_eq!(err.column(), 8);
}