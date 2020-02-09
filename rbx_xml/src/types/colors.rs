use std::io::{Read, Write};

use rbx_dom_weak::types::{Color3, Color3uint8};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

impl XmlType for Color3 {
    const XML_TAG_NAME: &'static str = "Color3";

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_characters_f32("R", self.r)?;
        writer.write_tag_characters_f32("G", self.g)?;
        writer.write_tag_characters_f32("B", self.b)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
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

            Color3::new(r, g, b)
        } else {
            let packed_value: u32 = contents.parse().map_err(|e| reader.error(e))?;

            let unpacked = decode_packed_color3(packed_value)?;

            Color3::new(
                f32::from(unpacked.r) / 255.0,
                f32::from(unpacked.g) / 255.0,
                f32::from(unpacked.b) / 255.0,
            )
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }
}

impl XmlType for Color3uint8 {
    const XML_TAG_NAME: &'static str = "Color3uint8";

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        let encoded = encode_packed_color3(*self);
        writer.write(XmlWriteEvent::characters(&encoded.to_string()))?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        // Color3uint8s are stored as packed u32s.
        let content = reader.read_characters()?;
        let packed_value: u32 = content.parse().map_err(|e| reader.error(e))?;

        let value = decode_packed_color3(packed_value)?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }
}

fn encode_packed_color3(source: Color3uint8) -> u32 {
    let Color3uint8 { r, g, b } = source;

    (b as u32) + ((g as u32) << 8) + ((r as u32) << 16)
}

fn decode_packed_color3(packed_color: u32) -> Result<Color3uint8, DecodeError> {
    let r = (packed_color >> 16) & 0xFF;
    let g = (packed_color >> 8) & 0xFF;
    let b = packed_color & 0xFF;

    Ok(Color3uint8::new(r as u8, g as u8, b as u8))
}
