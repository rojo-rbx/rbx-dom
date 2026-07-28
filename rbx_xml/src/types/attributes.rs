use std::io::Write;

use rbx_dom_weak::types::{Attributes, Ref, Variant};

use crate::{
    serializer::EmitState,
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    types::referent::write_ref,
    EncodeError,
};

pub const XML_TAG_NAME: &str = "BinaryString";

// Ref attributes are not written into the attributes binary
// format because the xml format does not use i32 ref ids
struct AdditionalAttributes<'a> {
    refs: Vec<(&'a str, Ref)>,
}

fn serialize_attributes<'a>(
    buf: &mut Vec<u8>,
    attributes: &'a Attributes,
) -> Result<AdditionalAttributes<'a>, rbx_dom_weak::types::Error> {
    let attribute_writer = rbx_dom_weak::types::AttributeWriter::new(buf);
    let mut attribute_writer = attribute_writer.write_len(attributes.len() as u32)?;
    let mut additional = AdditionalAttributes { refs: Vec::new() };
    for (name, variant) in attributes {
        match variant {
            Variant::BinaryString(value) => {
                attribute_writer.write_attribute_string(name, value.as_ref())
            }
            Variant::String(value) => attribute_writer.write_attribute_string(name, value.as_ref()),
            Variant::Bool(value) => attribute_writer.write_attribute_bool(name, *value),
            Variant::Int32(value) => attribute_writer.write_attribute_i32(name, *value),
            Variant::Float32(value) => attribute_writer.write_attribute_f32(name, *value),
            Variant::Float64(value) => attribute_writer.write_attribute_f64(name, *value),
            Variant::UDim(value) => attribute_writer.write_attribute_udim(name, *value),
            Variant::UDim2(value) => attribute_writer.write_attribute_udim2(name, *value),
            Variant::BrickColor(value) => {
                attribute_writer.write_attribute_brick_color(name, *value)
            }
            Variant::Color3(value) => attribute_writer.write_attribute_color3(name, *value),
            Variant::Vector2(value) => attribute_writer.write_attribute_vector2(name, *value),
            Variant::Vector3(value) => attribute_writer.write_attribute_vector3(name, *value),
            Variant::CFrame(value) => attribute_writer.write_attribute_cframe(name, *value),
            Variant::EnumItem(value) => attribute_writer.write_attribute_enum_item(name, value),
            Variant::NumberSequence(value) => {
                attribute_writer.write_attribute_number_sequence(name, value)
            }
            Variant::ColorSequence(value) => {
                attribute_writer.write_attribute_color_sequence(name, value)
            }
            Variant::NumberRange(value) => {
                attribute_writer.write_attribute_number_range(name, *value)
            }
            Variant::Rect(value) => attribute_writer.write_attribute_rect(name, *value),
            Variant::Font(value) => attribute_writer.write_attribute_font(name, value),
            Variant::Ref(referent) => {
                additional.refs.push((name, *referent));
                continue;
            }
            _ => todo!("How to return AttributeError::UnsupportedVariantType?"),
        }?;
    }
    Ok(additional)
}

pub fn write_attributes<W: Write>(
    writer: &mut XmlEventWriter<W>,
    property_name: &str,
    value: &Attributes,
    state: &mut EmitState,
) -> Result<(), EncodeError> {
    let mut buffer = Vec::new();

    let additional = match serialize_attributes(&mut buffer, value) {
        Ok(additional) => additional,
        Err(write_error) => return Err(writer.error(write_error)),
    };

    // Roblox requires PropertiesSerialize to write its length even when there
    // are no attributes. An empty attributes value serializes to nothing, so we
    // write a 0 count in its place.
    if buffer.is_empty() && property_name == "PropertiesSerialize" {
        buffer.extend_from_slice(&0u32.to_le_bytes());
    }

    writer.write(XmlWriteEvent::start_element(XML_TAG_NAME).attr("name", property_name))?;
    writer.write_string(&base64::encode(&buffer))?;
    writer.write(XmlWriteEvent::end_element())?;

    let mut name_scratch_string = String::new();
    for (name, referent) in additional.refs {
        name_scratch_string.clear();
        name_scratch_string.push_str("__attrRef_");
        name_scratch_string.push_str(name);
        write_ref(writer, &name_scratch_string, referent, state)?;
    }

    Ok(())
}
