use crate::debug_deserializer::DecodedModel;

static FOLDER: &[u8] = include_bytes!("../../test-files/from-studio/default-inserted-folder.rbxm");
static PART: &[u8] = include_bytes!("../../test-files/from-studio/default-inserted-part.rbxm");

#[test]
fn folder_from_studio() {
    let decoded = DecodedModel::from_reader(FOLDER);
    insta::assert_yaml_snapshot!(decoded);
}

#[test]
fn part_from_studio() {
    let decoded = DecodedModel::from_reader(PART);
    insta::assert_yaml_snapshot!(decoded);
}
