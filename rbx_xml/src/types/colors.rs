use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    error::{DecodeError, EncodeError},
    deserializer_core::XmlEventReader,
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

static TAG_NAMES: [&str; 3] = ["R", "G", "B"];

pub struct Color3Type;

impl XmlType<[f32; 3]> for Color3Type {
    const XML_TAG_NAME: &'static str = "Color3";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[f32; 3],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array_f32(value, &TAG_NAMES)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let contents = reader.read_characters()?;

        // Color3s have two possibilities:
        // They are either a packed int (like Color3uint8) or they are a triple of
        // <R>, <G>, and <B> tags with floating-point values inside them.
        // First we have to find out if we have a packed int in.
        let value = if contents.is_empty() {
            let r: f32 = reader.read_tag_contents_f32("R")?;
            let g: f32 = reader.read_tag_contents_f32("G")?;
            let b: f32 = reader.read_tag_contents_f32("B")?;

            RbxValue::Color3 {
                value: [ r, g, b ],
            }
        } else {
            let packed_value: u32 = contents.parse()
                .map_err(|e| reader.error(e))?;

            let [r, g, b] = decode_packed_color3(packed_value)?;

            RbxValue::Color3 {
                // floating-point Color3s go from 0 to 1 instead of 0 to 255
                value: [ f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0 ],
            }
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }
}

pub struct Color3uint8Type;

impl XmlType<[u8; 3]> for Color3uint8Type {
    const XML_TAG_NAME: &'static str = "Color3uint8";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[u8; 3],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        let encoded = encode_packed_color3(*value);
        writer.write(XmlWriteEvent::characters(&encoded.to_string()))?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        // Color3uint8s are stored as packed u32s.
        let content = reader.read_characters()?;
        let packed_value: u32 = content.parse()
            .map_err(|e| reader.error(e))?;

        let value = RbxValue::Color3uint8 {
            value: decode_packed_color3(packed_value)?,
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }
}

fn encode_packed_color3(source: [u8; 3]) -> u32 {
    let [r, g, b] = source;

    (b as u32) + ((g as u32) << 8) + ((r as u32) << 16)
}

fn decode_packed_color3(packed_color: u32) -> Result<[u8; 3], DecodeError> {
    let r = (packed_color >> 16) & 0xFF;
    let g = (packed_color >> 8) & 0xFF;
    let b = packed_color & 0xFF;

    Ok([r as u8, g as u8, b as u8])
}