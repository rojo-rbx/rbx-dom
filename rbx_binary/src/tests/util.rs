use std::{fs, path::Path};

use rbx_dom_weak::DomViewer;

use crate::{
    deserializer::{decode, DecodePropertyBehavior},
    encode,
    text_deserializer::DecodedModel,
};

/// Run a basic gauntlet of tests to verify that the serializer and deserializer
/// can handle this model correctly.
pub fn run_model_base_suite(model_path: impl AsRef<Path>) {
    let model_path = model_path.as_ref();

    // The useful name of the test is the folder containing it.
    let model_stem = model_path
        .parent()
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let contents = fs::read(model_path).unwrap();

    // Write out a text version of the test file. This helps when debugging what
    // the actual test file is and also guards us against the test file
    // changing.
    let text_decoded = DecodedModel::from_reader(contents.as_slice());
    insta::assert_yaml_snapshot!(format!("{}__input", model_stem), text_decoded);

    // Decode the test file and snapshot a stable version of the resulting tree.
    // This should properly test the deserializer.
    let decoded = decode(contents.as_slice(), DecodePropertyBehavior::Default).unwrap();
    let decoded_viewed = DomViewer::new().view_children(&decoded);
    insta::assert_yaml_snapshot!(format!("{}__decoded", model_stem), decoded_viewed);

    // Re-encode the model that we decoded. We can't snapshot this directly...
    let decoded_root = decoded.root();
    let top_level_ids = decoded_root.children();
    let mut encoded = Vec::new();
    encode(&decoded, top_level_ids, &mut encoded).unwrap();

    // ...but we can snapshot the text representation of what we encoded! In an
    // ideal world, this would be very similar or the same as the text
    // representation of the original test file. In practice, we'll differ
    // slightly in chunk ordering, compression, etc.
    let text_roundtrip = DecodedModel::from_reader(encoded.as_slice());
    insta::assert_yaml_snapshot!(format!("{}__encoded", model_stem), text_roundtrip);

    // As a sanity check, make sure we can decode the re-encoded version of the
    // file.
    //
    // We don't make any assertions about the result right now, as our format
    // support is still lacking. In the future, we should assert that this is
    // the same as the original decoding of the test file.
    decode(encoded.as_slice(), DecodePropertyBehavior::Default).unwrap();
}
