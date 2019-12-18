use rbx_dom_test::InstanceBuilder;
use rbx_dom_weak::RbxValue;

use crate::{encode, text_deserializer::DecodedModel};

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
