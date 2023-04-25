use std::io::{Read, Write};

use rbx_dom_weak::types::{Font, FontStyle, FontWeight};

use crate::{
    core::XmlType,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

fn write_content(
    writer: &mut XmlEventWriter<impl Write>,
    content: &str,
    tag: &str,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element(tag))?;
    if content.is_empty() {
        // This doesn't feel like a great XML idiom
        writer.write(XmlWriteEvent::start_element("null"))?;
    } else {
        writer.write(XmlWriteEvent::start_element("url"))?;
        writer.write_string(content)?;
    }
    writer.write(XmlWriteEvent::end_element())?;
    writer.write(XmlWriteEvent::end_element())?;
    Ok(())
}

fn read_content_inner(reader: &mut XmlEventReader<impl Read>) -> Result<String, DecodeError> {
    match reader.expect_next()? {
        XmlReadEvent::StartElement {
            name,
            attributes,
            namespace,
        } => match name.local_name.as_str() {
            "null" => {
                reader.expect_end_with_name("null")?;
                Ok(String::new())
            }
            "url" => {
                let value = reader.read_characters()?;
                reader.expect_end_with_name("url")?;
                Ok(value)
            }
            _ => {
                let event = XmlReadEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                };
                Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)))
            }
        },
        event => Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event))),
    }
}

fn read_content(reader: &mut XmlEventReader<impl Read>, tag: &str) -> Result<String, DecodeError> {
    match reader.expect_next()? {
        XmlReadEvent::StartElement {
            name,
            attributes,
            namespace,
        } => {
            if name.local_name.as_str() != tag {
                let event = XmlReadEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                };
                return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
            }
        }
        event => return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event))),
    };

    let value = read_content_inner(reader)?;

    reader.expect_end_with_name(tag)?;

    Ok(value)
}

impl XmlType for Font {
    const XML_TAG_NAME: &'static str = "Font";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        write_content(writer, &self.family, "Family")?;

        writer.write_value_in_tag(&self.weight.as_u16(), "Weight")?;

        let style = match self.style {
            FontStyle::Normal => "Normal",
            FontStyle::Italic => "Italic",
        };
        writer.write_tag_characters("Style", style)?;

        if let Some(ref cached_face_id) = self.cached_face_id {
            write_content(writer, cached_face_id, "CachedFaceId")?;
        }

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        // Patchwork fix for older Roblox files that were written with invalid
        // `Font` tags
        if let XmlReadEvent::EndElement { .. } = reader.expect_peek()? {
            return Ok(Font::default());
        }

        let family = read_content(reader, "Family")?;

        let weight: u16 = reader.read_value_in_tag("Weight")?;
        let weight = FontWeight::from_u16(weight).unwrap_or_default();

        let style = match reader.read_tag_contents("Style")?.as_str() {
            "Normal" => FontStyle::Normal,
            "Italic" => FontStyle::Italic,
            _ => FontStyle::Normal,
        };

        let cached_face_id = match reader.expect_peek()? {
            XmlReadEvent::StartElement { name, .. } if name.local_name == "CachedFaceId" => {
                Some(read_content(reader, "CachedFaceId")?)
            }
            _ => None,
        };

        Ok(Font {
            family,
            weight,
            style,
            cached_face_id,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_font_face() {
        test_util::test_xml_round_trip(&Font {
            family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            cached_face_id: Some("rbxasset://fonts/SourceSansPro-Regular.ttf".to_owned()),
        });
    }

    #[test]
    fn deserialize_font_face() {
        test_util::test_xml_deserialize(
            r#"
                <Font name="foo">
                    <Family><url>rbxasset://fonts/families/SourceSansPro.json</url></Family>
                    <Weight>400</Weight>
                    <Style>Normal</Style>
                    <CachedFaceId><url>rbxasset://fonts/SourceSansPro-Regular.ttf</url></CachedFaceId>
                </Font>
            "#,
            &Font {
                family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
                weight: FontWeight::Regular,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/SourceSansPro-Regular.ttf".to_owned()),
            },
        );
    }

    #[test]
    fn serialize_font_face() {
        test_util::test_xml_serialize(
            r#"
            <Font name="foo">
                <Family><url>rbxasset://fonts/families/SourceSansPro.json</url></Family>
                <Weight>400</Weight>
                <Style>Normal</Style>
                <CachedFaceId><url>rbxasset://fonts/SourceSansPro-Regular.ttf</url></CachedFaceId>
            </Font>
            "#,
            &Font {
                family: "rbxasset://fonts/families/SourceSansPro.json".to_owned(),
                weight: FontWeight::Regular,
                style: FontStyle::Normal,
                cached_face_id: Some("rbxasset://fonts/SourceSansPro-Regular.ttf".to_owned()),
            },
        );
    }
}
