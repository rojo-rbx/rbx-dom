use rbx_dom_weak::{BrickColor, RbxValue, RbxValueConversion, RbxValueType};

#[test]
fn color3_to_color3uint8() {
    let black = RbxValue::Color3 {
        value: [0.0, 0.0, 0.0],
    };

    assert_eq!(
        black.try_convert_ref(RbxValueType::Color3uint8),
        RbxValueConversion::Converted(RbxValue::Color3uint8 { value: [0, 0, 0] }),
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
    let black = RbxValue::Color3uint8 { value: [0, 0, 0] };

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

#[test]
fn int_to_brickcolor() {
    let eggplant = RbxValue::Int32 { value: 316 };
    assert_eq!(
        eggplant.try_convert_ref(RbxValueType::BrickColor),
        RbxValueConversion::Converted(RbxValue::BrickColor {
            value: BrickColor::Eggplant,
        }),
    );

    let zero = RbxValue::Int32 { value: 0 };
    assert_eq!(
        zero.try_convert_ref(RbxValueType::BrickColor),
        RbxValueConversion::Failed,
    );

    let out_of_range = RbxValue::Int32 { value: 9999999 };
    assert_eq!(
        out_of_range.try_convert_ref(RbxValueType::BrickColor),
        RbxValueConversion::Failed,
    );
}
