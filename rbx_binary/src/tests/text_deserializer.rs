use crate::text_deserializer::DecodedModel;

static FOLDER: &[u8] = include_bytes!("../../test-files/from-studio/default-inserted-folder.rbxm");
static PART: &[u8] = include_bytes!("../../test-files/from-studio/default-inserted-part.rbxm");
static NESTED_FOLDERS: &[u8] = include_bytes!("../../test-files/from-studio/nested-folders.rbxm");

#[test]
fn default_inserted_folder() {
    let decoded = DecodedModel::from_reader(FOLDER);
    insta::assert_yaml_snapshot!(decoded);
}

#[test]
fn default_inserted_part() {
    let decoded = DecodedModel::from_reader(PART);
    insta::assert_yaml_snapshot!(decoded);
}

#[test]
fn nested_folders() {
    let decoded = DecodedModel::from_reader(NESTED_FOLDERS);
    insta::assert_yaml_snapshot!(decoded);
}
