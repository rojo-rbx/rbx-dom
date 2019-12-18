use rbx_dom_test::InstanceBuilder;
use rbx_dom_weak::RbxValue;

use crate::{encode, text_deserializer::DecodedModel};

/// A basic test to make sure we can serialize the simplest instance: a Folder.
#[test]
fn just_folder() {
    let tree = InstanceBuilder::new("Folder").build();
    let mut buffer = Vec::new();

    encode(&tree, &[tree.get_root_id()], &mut buffer).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that a tree containing some instances with a value and others
/// without will correctly fall back to (some) default value.
#[test]
fn partially_present() {
    let tree = InstanceBuilder::new("Folder")
        .children(vec![
            // This instance's `Value` property should be preserved.
            InstanceBuilder::new("StringValue").property(
                "Value",
                RbxValue::String {
                    value: "Hello".to_string(),
                },
            ),
            // This instance's `Value` property should be the empty string.
            InstanceBuilder::new("StringValue"),
        ])
        .build();

    let root_ids = tree
        .get_instance(tree.get_root_id())
        .unwrap()
        .get_children_ids();

    let mut buffer = Vec::new();
    encode(&tree, root_ids, &mut buffer).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that unknown properties get serialized on instances.
#[test]
fn unknown_property() {
    let tree = InstanceBuilder::new("Folder")
        .property(
            "WILL_NEVER_EXIST",
            RbxValue::String {
                value: "Hi, mom!".to_owned(),
            },
        )
        .build();

    let mut buffer = Vec::new();
    encode(&tree, &[tree.get_root_id()], &mut buffer).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}

/// Ensures that serializing a tree with an unimplemented property type returns
/// an error instead of panicking.
///
/// This test will need to be updated once we implement the type used here.
#[test]
fn unimplemented_type_known_property() {
    let tree = InstanceBuilder::new("UIListLayout")
        .property("Padding", RbxValue::UDim { value: (1.0, -30) })
        .build();

    let mut buffer = Vec::new();
    let result = encode(&tree, &[tree.get_root_id()], &mut buffer);

    assert!(result.is_err());
}

/// Ensures that serializing a tree with an unimplemented property type AND an
/// unknown property descriptor returns an error instead of panicking.
///
/// Because rbx_binary has additional logic for falling back to values with no
/// known property descriptor, we should make sure that logic works.
///
/// This test will need to be updated once we implement the type used here.
#[test]
fn unimplemented_type_unknown_property() {
    let tree = InstanceBuilder::new("Folder")
        .property("WILL_NEVER_EXIST", RbxValue::UDim { value: (0.0, 50) })
        .build();

    let mut buffer = Vec::new();
    let result = encode(&tree, &[tree.get_root_id()], &mut buffer);

    assert!(result.is_err());
}
