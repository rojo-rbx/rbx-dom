use crate::debug_deserializer::DecodedModel;

#[test]
fn folder_from_studio() {
    static FOLDER: &[u8] = include_bytes!("../../test-files/just-folder.rbxm");

    let decoded = DecodedModel::from_reader(FOLDER);
    insta::assert_yaml_snapshot!(decoded);
}
