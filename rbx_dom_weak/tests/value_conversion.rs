use rbx_dom_weak::{RbxValue, RbxValueType, RbxValueConversion};

#[test]
fn color3_to_color3uint8() {
    let black = RbxValue::Color3 {
        value: [0.0, 0.0, 0.0],
    };

    assert_eq!(
        black.try_convert_ref(RbxValueType::Color3uint8),
        RbxValueConversion::Converted(RbxValue::Color3uint8 {
            value: [0, 0, 0],
        }),
    );

    let white = RbxValue::Color3 {
        value: [1.0, 1.0, 1.0],
    };

    assert_eq!(
        white.try_convert_ref(RbxValueType::Color3uint8),
        RbxValueConversion::Converted(RbxValue::Color3uint8 {
            value: [255, 255, 255],
        }),
    );
}

#[test]
fn color3uint8_to_color3() {
    let black = RbxValue::Color3uint8 {
        value: [0, 0, 0],
    };

    assert_eq!(
        black.try_convert_ref(RbxValueType::Color3),
        RbxValueConversion::Converted(RbxValue::Color3 {
            value: [0.0, 0.0, 0.0],
        }),
    );

    let white = RbxValue::Color3uint8 {
        value: [255, 255, 255],
    };

    assert_eq!(
        white.try_convert_ref(RbxValueType::Color3),
        RbxValueConversion::Converted(RbxValue::Color3 {
            value: [1.0, 1.0, 1.0],
        }),
    );
}