use rbx_dom_test::InstanceBuilder;

use crate::{encode, text_deserializer::DecodedModel};

#[test]
fn just_folder() {
    let tree = InstanceBuilder::new("Folder").build();
    let mut buffer = Vec::new();

    encode(&tree, &[tree.get_root_id()], &mut buffer).expect("failed to encode model");

    let decoded = DecodedModel::from_reader(buffer.as_slice());
    insta::assert_yaml_snapshot!(decoded);
}
