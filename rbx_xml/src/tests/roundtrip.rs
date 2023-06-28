//! Simply roundtrip tests on data types to ensure if nothing else we
//! are compatible with ourselves.

use crate::deserializer::data_types as deserializers;
use crate::serializer::data_types as serializers;
use rbx_dom_weak::types::{
    Axes, BinaryString, CFrame, Color3, Color3uint8, ColorSequence, ColorSequenceKeypoint, Content,
    CustomPhysicalProperties, Enum, Faces, Font, FontStyle, FontWeight, Matrix3, NumberRange,
    NumberSequence, NumberSequenceKeypoint, PhysicalProperties, Ray, Rect, UDim, UDim2, UniqueId,
    Vector2, Vector3, Vector3int16,
};

use crate::roundtrip_test;

#[test]
fn string_roundtrip() {
    roundtrip_test!(
        serializers::string_serializer,
        deserializers::string_deserializer,
        "Hello, world! Look at this unicode: 你好 مرحبًا 🥔🤳"
    );
}

#[test]
fn axes_roundtrip() {
    roundtrip_test!(
        serializers::axes_serializer,
        deserializers::axes_deserializer,
        Axes::all()
    );
    roundtrip_test!(
        serializers::axes_serializer,
        deserializers::axes_deserializer,
        Axes::empty()
    );
}

#[test]
fn binary_string() {
    roundtrip_test!(
        serializers::binary_string_serializer,
        deserializers::binary_string_deserializer,
        BinaryString::from("Hello, world! Look at this unicode: 你好 مرحبًا 🥔🤳".as_bytes())
    );
}

#[test]
fn bool() {
    roundtrip_test!(
        serializers::bool_serializer,
        deserializers::bool_deserializer,
        true
    );
    roundtrip_test!(
        serializers::bool_serializer,
        deserializers::bool_deserializer,
        false
    );
}

#[test]
fn cframe() {
    roundtrip_test!(
        serializers::cframe_serializer,
        deserializers::cframe_deserializer,
        CFrame::new(
            Vector3::new(10.0, 20.0, 30.0),
            Matrix3 {
                x: Vector3::new(-10.0, -20.0, -30.0),
                y: Vector3::new(0.15625, -0.15625, 0.0),
                z: Vector3::new(f32::INFINITY, f32::NEG_INFINITY, 1337.0)
            }
        )
    );
}

#[test]
fn color_sequence() {
    roundtrip_test!(
        serializers::color_sequence_serializer,
        deserializers::color_sequence_deserializer,
        ColorSequence {
            keypoints: vec![
                ColorSequenceKeypoint::new(0.0, Color3::new(1.0, -1.0, 0.5)),
                ColorSequenceKeypoint::new(1.0, Color3::new(f32::INFINITY, f32::NEG_INFINITY, 0.0))
            ]
        }
    );
}

#[test]
fn color3() {
    roundtrip_test!(
        serializers::color3_serializer,
        deserializers::color3_deserializer,
        Color3::new(-0.5, 1.0, f32::NEG_INFINITY)
    );
}

#[test]
fn color3uint8() {
    roundtrip_test!(
        serializers::color3uint8_serializer,
        deserializers::color3uint8_deserializer,
        Color3uint8::new(10, 20, 30)
    );
}

#[test]
fn content() {
    roundtrip_test!(
        serializers::content_serializer,
        deserializers::content_deserializer,
        Content::from("Hello, world! Look at this unicode: 你好 مرحبًا 🥔🤳")
    );
    roundtrip_test!(
        serializers::content_serializer,
        deserializers::content_deserializer,
        Content::new()
    );
}

#[test]
fn enumeration() {
    roundtrip_test!(
        serializers::enum_serializer,
        deserializers::enum_deserializer,
        Enum::from_u32(1337)
    );
}

#[test]
fn faces() {
    roundtrip_test!(
        serializers::faces_serializer,
        deserializers::faces_deserializer,
        Faces::all()
    );
    roundtrip_test!(
        serializers::faces_serializer,
        deserializers::faces_deserializer,
        Faces::empty()
    );
}

#[test]
fn font() {
    roundtrip_test!(
        serializers::font_serializer,
        deserializers::font_deserializer,
        Font {
            family: "rbxasset://fonts/families/SourceSansPro.json".into(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            cached_face_id: Some("rbxasset://fonts/SourceSansPro-Regular.ttf".into())
        }
    );
    roundtrip_test!(
        serializers::font_serializer,
        deserializers::font_deserializer,
        Font {
            family: "rbxasset://fonts/families/SourceSansPro.json".into(),
            weight: FontWeight::Heavy,
            style: FontStyle::Italic,
            cached_face_id: None,
        }
    );
    roundtrip_test!(
        serializers::font_serializer,
        deserializers::font_deserializer,
        Font::default()
    );
}

#[test]
fn number_range() {
    roundtrip_test!(
        serializers::number_range_serializer,
        deserializers::number_range_deserializer,
        NumberRange::new(-0.15625, 1.5)
    );
    roundtrip_test!(
        serializers::number_range_serializer,
        deserializers::number_range_deserializer,
        NumberRange::new(f32::NEG_INFINITY, f32::INFINITY)
    );
}

#[test]
fn number_sequence() {
    roundtrip_test!(
        serializers::number_sequence_serializer,
        deserializers::number_sequence_deserializer,
        NumberSequence {
            keypoints: vec![
                NumberSequenceKeypoint::new(0.0, -1.0, f32::INFINITY),
                NumberSequenceKeypoint::new(1.0, f32::NEG_INFINITY, 1337.0),
            ]
        }
    )
}

#[test]
fn float32() {
    roundtrip_test!(
        serializers::f32_serializer,
        deserializers::f32_deserializer,
        f32::INFINITY
    );
    roundtrip_test!(
        serializers::f32_serializer,
        deserializers::f32_deserializer,
        f32::NEG_INFINITY
    );
    roundtrip_test!(
        serializers::f32_serializer,
        deserializers::f32_deserializer,
        -1.0
    );
    roundtrip_test!(
        serializers::f32_serializer,
        deserializers::f32_deserializer,
        0.5
    );
    roundtrip_test!(
        serializers::f32_serializer,
        deserializers::f32_deserializer,
        0.0
    );
}

#[test]
fn float64() {
    roundtrip_test!(
        serializers::f64_serializer,
        deserializers::f64_deserializer,
        f64::INFINITY
    );
    roundtrip_test!(
        serializers::f64_serializer,
        deserializers::f64_deserializer,
        f64::NEG_INFINITY
    );
    roundtrip_test!(
        serializers::f64_serializer,
        deserializers::f64_deserializer,
        -1.0
    );
    roundtrip_test!(
        serializers::f64_serializer,
        deserializers::f64_deserializer,
        0.5
    );
    roundtrip_test!(
        serializers::f64_serializer,
        deserializers::f64_deserializer,
        0.0
    );
}

#[test]
fn int32() {
    roundtrip_test!(
        serializers::i32_serializer,
        deserializers::i32_deserializer,
        0
    );
    roundtrip_test!(
        serializers::i32_serializer,
        deserializers::i32_deserializer,
        1
    );
    roundtrip_test!(
        serializers::i32_serializer,
        deserializers::i32_deserializer,
        -1
    );
}

#[test]
fn int64() {
    roundtrip_test!(
        serializers::i64_serializer,
        deserializers::i64_deserializer,
        0
    );
    roundtrip_test!(
        serializers::i64_serializer,
        deserializers::i64_deserializer,
        1
    );
    roundtrip_test!(
        serializers::i64_serializer,
        deserializers::i64_deserializer,
        -1
    );
}

#[test]
fn optional_cframe() {
    roundtrip_test!(
        serializers::optional_cframe_serializer,
        deserializers::optional_cframe_deserializer,
        Some(CFrame::new(
            Vector3::new(10.0, 20.0, 30.0),
            Matrix3 {
                x: Vector3::new(-10.0, -20.0, -30.0),
                y: Vector3::new(0.15625, -0.15625, 0.0),
                z: Vector3::new(f32::INFINITY, f32::NEG_INFINITY, 1337.0)
            }
        ))
    );
    roundtrip_test!(
        serializers::optional_cframe_serializer,
        deserializers::optional_cframe_deserializer,
        // I don't know why this is necessary because it's being passed
        // into a function that accepts `Option<CFrame>`. It's ultimately
        // harmless but I don't like that it's necessary.
        None::<CFrame>
    );
}

#[test]
fn physical_properties() {
    roundtrip_test!(
        serializers::physical_properties_serializer,
        deserializers::physical_properties_deserializer,
        PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 0.5,
            friction: -1.0,
            elasticity: 1337.0,
            friction_weight: f32::INFINITY,
            elasticity_weight: f32::NEG_INFINITY,
        })
    );
    roundtrip_test!(
        serializers::physical_properties_serializer,
        deserializers::physical_properties_deserializer,
        PhysicalProperties::Default
    );
}

#[test]
fn ray() {
    roundtrip_test!(
        serializers::ray_serializer,
        deserializers::ray_deserializer,
        Ray::new(
            Vector3::new(10.0, -20.0, 0.0),
            Vector3::new(0.15625, f32::INFINITY, f32::NEG_INFINITY)
        )
    )
}

#[test]
fn rect() {
    roundtrip_test!(
        serializers::rect_serializer,
        deserializers::rect_deserializer,
        Rect::new(
            Vector2::new(f32::INFINITY, f32::NEG_INFINITY),
            Vector2::new(0.1525, -1.0)
        )
    )
}

#[test]
fn udim() {
    roundtrip_test!(
        serializers::udim_serializer,
        deserializers::udim_deserializer,
        UDim::new(0.5, 10)
    );
    roundtrip_test!(
        serializers::udim_serializer,
        deserializers::udim_deserializer,
        UDim::new(-1.0, -20)
    );
    roundtrip_test!(
        serializers::udim_serializer,
        deserializers::udim_deserializer,
        UDim::new(f32::INFINITY, 0)
    );
}

#[test]
fn udim2() {
    roundtrip_test!(
        serializers::udim2_serializer,
        deserializers::udim2_deserializer,
        UDim2::new(UDim::new(1.0, 10), UDim::new(-0.15625, -10))
    );
    roundtrip_test!(
        serializers::udim2_serializer,
        deserializers::udim2_deserializer,
        UDim2::new(
            UDim::new(f32::INFINITY, i32::MAX),
            UDim::new(f32::NEG_INFINITY, i32::MIN)
        )
    );
}

#[test]
fn unique_id() {
    roundtrip_test!(
        serializers::unique_id_serializer,
        deserializers::unique_id_deserializer,
        UniqueId::new(0x0C0FFEE, 0xDEADBEEF, 0x01234567_89ABCDEF)
    );
    roundtrip_test!(
        serializers::unique_id_serializer,
        deserializers::unique_id_deserializer,
        UniqueId::new(0, 0, 0)
    );
}

#[test]
fn vector2() {
    roundtrip_test!(
        serializers::vector2_serializer,
        deserializers::vector2_deserializer,
        Vector2::new(10.0, -0.15625)
    );
    roundtrip_test!(
        serializers::vector2_serializer,
        deserializers::vector2_deserializer,
        Vector2::new(f32::INFINITY, f32::NEG_INFINITY)
    );
}

#[test]
fn vector3() {
    roundtrip_test!(
        serializers::vector3_serializer,
        deserializers::vector3_deserializer,
        Vector3::new(10.0, -0.15625, 0.0)
    );
    roundtrip_test!(
        serializers::vector3_serializer,
        deserializers::vector3_deserializer,
        Vector3::new(f32::INFINITY, f32::NEG_INFINITY, 1337.0)
    );
}

#[test]
fn vector3int16() {
    roundtrip_test!(
        serializers::vector3int16_serializer,
        deserializers::vector3int16_deserializer,
        Vector3int16::new(i16::MAX, i16::MIN, 0)
    )
}
