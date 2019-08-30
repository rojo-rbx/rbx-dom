use rbx_dom_weak::RbxValue;

#[test]
fn binary_string() {
    let value = RbxValue::BinaryString {
        value: b"Hello, world!".to_vec(),
    };

    let encoded = serde_json::to_string(&value).expect("Couldn't encode value to JSON");

    assert_eq!(
        encoded,
        r#"{"Type":"BinaryString","Value":"SGVsbG8sIHdvcmxkIQ=="}"#
    );

    let decoded: RbxValue =
        serde_json::from_str(&encoded).expect("Couldn't decode value from JSON");

    assert_eq!(decoded, value);
}
