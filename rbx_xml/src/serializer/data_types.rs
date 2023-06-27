mod binary_string;
mod bitfield;
mod cframe;
mod colors;
mod font;
mod option;
mod physical_properties;
mod ray;
mod rect;
mod sequences;
mod simple_types;
mod udims;
mod unique_id;
mod vectors;

use std::io;

use super::writer::XmlWriter;
use rbx_dom_weak::types::Variant;

use super::error::{EncodeError, ErrorKind};

pub use self::{
    binary_string::{attributes_serializer, tags_serializer},
    bitfield::{axes_serializer, faces_serializer},
    cframe::cframe_serializer,
    colors::{brickcolor_serializer, color3_serializer, color3uint8_serializer},
    font::font_serializer,
    option::optional_cframe_serializer,
    physical_properties::physical_properties_serializer,
    ray::ray_serializer,
    rect::rect_serializer,
    sequences::{color_sequence_serializer, number_range_serializer, number_sequence_serializer},
    simple_types::{
        binary_string_serializer, bool_serializer, content_serializer, enum_serializer,
        f32_serializer, f64_serializer, i32_serializer, i64_serializer, string_serializer,
    },
    udims::{udim2_serializer, udim_serializer},
    unique_id::unique_id_serializer,
    vectors::{vector2_serializer, vector3_serializer, vector3int16_serializer},
};

pub fn serialize_ref<W: io::Write>(
    writer: &mut XmlWriter<W>,
    prop_name: &str,
    referent: &str,
) -> Result<(), EncodeError> {
    writer
        .start_element("Ref")
        .attribute("name", prop_name)
        .finalize()?;
    writer.write_text(referent)?;
    writer.end_element("Ref")?;
    Ok(())
}

pub fn serialize_shared_string<W: io::Write>(
    writer: &mut XmlWriter<W>,
    prop_name: &str,
    key: &[u8],
) -> Result<(), EncodeError> {
    writer
        .start_element("SharedString")
        .attribute("name", prop_name)
        .finalize()?;
    // We've historically written only the first 16 bytes of shared string
    // hashes. This isn't really recommended but collisions are unlikely
    // and we can't really change it now because it'd break diffs
    writer.write_base64(&key[0..16])?;
    writer.end_element("SharedString")?;

    Ok(())
}

macro_rules! serializers {
    ($($name:ident: $elem:literal => $serializer:path),*$(,)?) => {
        pub fn is_known_type(value: &Variant) -> bool {
            match value {
                $(
                    Variant::$name(_) => true,
                )*
                _ => false,
            }
        }

        pub fn try_serialize_value<W: io::Write>(
            writer: &mut XmlWriter<W>,
            value: &Variant,
        ) -> Result<(), EncodeError> {
            match value {
                Variant::Ref(_) | Variant::SharedString(_) => {
                    Err(ErrorKind::TypeNeedsState(value.ty()).err())
                }
                $(
                    Variant::$name(v) => $serializer(writer, v),
                )*
                _ => Err(ErrorKind::CannotSerialize(value.ty()).err()),
            }
        }

        pub fn attempt_serialization<W: io::Write>(
            writer: &mut XmlWriter<W>,
            prop_name: &str,
            prop_value: &Variant,
        ) -> Result<(), EncodeError> {
            log::trace!("serializing {prop_name} of type {:?}", prop_value.ty());
            let element_name = match prop_value {
                Variant::Ref(_) | Variant::SharedString(_) => {
                    return Err(ErrorKind::TypeNeedsState(prop_value.ty()).err())
                }
                $(
                    Variant::$name(_) => $elem,
                )*
                _ => return Ok(())
            };
            log::trace!("serializing with element name {element_name}");
            writer
                .start_element(element_name)
                .attribute("name", prop_name)
                .finalize()?;

            try_serialize_value(writer, prop_value)?;

            writer.end_element(element_name)?;

            Ok(())
        }
    };
}

// TODO validate tag names
serializers! {
    Tags: "BinaryString" => tags_serializer,
    Attributes: "BinaryString" => attributes_serializer,
    String: "string" => string_serializer,
    BinaryString: "BinaryString" => binary_string_serializer,
    Bool: "bool" => bool_serializer,
    Int32: "int" => i32_serializer,
    Int64: "int64" => i64_serializer,
    Float32: "float" => f32_serializer,
    Float64: "double" => f64_serializer,
    Vector3: "Vector3" => vector3_serializer,
    Vector2: "Vector2" => vector2_serializer,
    Vector3int16: "Vector3int16" => vector3int16_serializer,
    Ray: "Ray" => ray_serializer,
    Faces: "Faces" => faces_serializer,
    Axes: "Axes" => axes_serializer,
    CFrame: "CoordinateFrame" => cframe_serializer,
    ColorSequence: "ColorSequence" => color_sequence_serializer,
    NumberSequence: "NumberSequence" => number_sequence_serializer,
    NumberRange: "NumberRange" => number_range_serializer,
    Color3: "Color3" => color3_serializer,
    Color3uint8: "Color3uint8" => color3uint8_serializer,
    UDim: "UDim" => udim_serializer,
    UDim2: "UDim2" => udim2_serializer,
    UniqueId: "UniqueId" => unique_id_serializer,
    Content: "Content" => content_serializer,
    Font: "Font" => font_serializer,
    Rect: "Rect2D" => rect_serializer,
    PhysicalProperties: "PhysicalProperties" => physical_properties_serializer,
    OptionalCFrame: "OptionalCoordinateFrame" => optional_cframe_serializer,
    Enum: "token" => enum_serializer,
    BrickColor: "int" => brickcolor_serializer,
}
