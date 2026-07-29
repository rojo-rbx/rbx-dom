use std::io::{Read, Write};

use rbx_dom_weak::types::{Attributes, BinaryString, Ref, Variant};

use crate::{
    core::XmlType,
    deserializer::ParseState,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer::EmitState,
    serializer_core::{XmlEventWriter, XmlWriteEvent},
    types::referent::write_ref,
};

pub const XML_TAG_NAME: &str = "BinaryString";

struct AdditionalAttributes<'a> {
    // Ref attributes are not written into the attributes binary
    // format because the xml format does not use i32 ref ids
    refs: Vec<(&'a str, Ref)>,
}

fn serialize_attributes<'a>(
    buf: &mut Vec<u8>,
    attributes: &'a Attributes,
) -> Result<AdditionalAttributes<'a>, rbx_dom_weak::types::Error> {
    let mut additional = AdditionalAttributes { refs: Vec::new() };
    if attributes.is_empty() {
        return Ok(additional);
    }

    let attribute_writer = rbx_dom_weak::types::AttributeWriter::new(buf);
    let mut attribute_writer = attribute_writer.write_len(attributes.len() as u32)?;
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
    //
    // TODO: reconcile this with if attributes.is_empty() in serialize_attributes
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

pub fn read_attributes<R: Read>(
    reader: &mut XmlEventReader<R>,
    id: Ref,
    state: &mut ParseState,
) -> Result<Variant, DecodeError> {
    let value = BinaryString::read_outer_xml(reader)?;

    let bytes: &[u8] = value.as_ref();
    let attributes = match Attributes::from_reader(bytes) {
        Ok(attributes) => Variant::Attributes(attributes),
        Err(err) => {
            log::warn!(
                "Failed to parse Attributes because {err:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning."
            );

            Variant::BinaryString(value)
        }
    };

    // peek and consume Ref tags that appear directly after the AttributesSerialize tag
    while let XmlReadEvent::StartElement {
        name, attributes, ..
    } = reader.expect_peek()?
    {
        // TODO: rust 2024 if let chains
        if name.local_name == "Ref" {
            let mut xml_property_name = None;

            for attribute in attributes {
                if attribute.name.local_name == "name" {
                    xml_property_name = Some(attribute.value.as_str());
                    break;
                }
            }

            let xml_property_name = match xml_property_name {
                Some(value) => value,
                None => return Err(reader.error(DecodeErrorKind::MissingAttribute("name"))),
            };

            if let Some(("__attrRef_", name)) =
                xml_property_name.split_at_checked("__attrRef_".len())
            {
                let name = name.to_owned();
                let ref_contents = reader.read_tag_contents(super::referent::XML_TAG_NAME)?;

                // We need to rewrite this attribute as part of a follow-up pass.
                //
                // We might not know which ID this referent points to yet, so instead of
                // trying to handle the case where we do here, we just let all referents
                // get written later.
                state.add_attribute_referent_rewrite(id, name, ref_contents);

                // look for another Ref attribute
                continue;
            }
        }
        break;
    }

    Ok(attributes)
}
