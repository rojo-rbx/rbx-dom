use std::io::{Read, Write};

use rbx_dom_weak::types::Content;

use crate::{
    core::XmlType,
    deserializer_core::{XmlEventReader, XmlReadEvent},
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

// A ContentId type is serialized as either:
// <null></null>, which indicates an empty content value
// <url>something</url>, where 'something' is a URL to use for content.
impl XmlType for Content {
    const XML_TAG_NAME: &'static str = "ContentId";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        // FIXME: Content should have a method for this
        let as_str: &str = self.as_ref();

        if as_str.is_empty() {
            // This doesn't feel like a great XML idiom
            writer.write(XmlWriteEvent::start_element("null"))?;
        } else {
            writer.write(XmlWriteEvent::start_element("url"))?;
            writer.write_string(self.as_ref())?;
        }

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let value = match reader.expect_next()? {
            XmlReadEvent::StartElement {
                name,
                attributes,
                namespace,
            } => match name.local_name.as_str() {
                "null" => {
                    reader.expect_end_with_name("null")?;

                    String::new()
                }
                "url" => {
                    let value = reader.read_characters()?;
                    reader.expect_end_with_name("url")?;

                    value
                }
                _ => {
                    let event = XmlReadEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    };
                    return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(event)));
                }
            },
            unexpected => return Err(reader.error(DecodeErrorKind::UnexpectedXmlEvent(unexpected))),
        };

        Ok(Content::from(value))
    }
}

/// In release 645, Roblox changed `Content` to serialize as `ContentId`.
/// However, we still need to deserialize older models, so we have to support
/// `Content`.
///
/// This may need to be replaced in the future if Roblox implements a new
/// `Content` type, but right now they haven't.
#[derive(Debug, PartialEq, Eq)]
pub struct ContentDummy(pub Content);

impl XmlType for ContentDummy {
    const XML_TAG_NAME: &'static str = "Content";

    fn write_xml<W: Write>(&self, _writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        panic!("Content values are only read, never written.");
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        // We just want to use the same deserializer as ContentId
        Content::read_xml(reader).map(ContentDummy)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_content_url() {
        test_util::test_xml_round_trip(&Content::from("url://not/really/a/url"));
    }

    #[test]
    fn round_trip_content_null() {
        test_util::test_xml_round_trip(&Content::new());
    }

    #[test]
    fn deserialize_content_url() {
        test_util::test_xml_deserialize(
            r#"
                <ContentId name="something">
                    <url>Some URL</url>
                </ContentId>
            "#,
            &Content::from("Some URL"),
        );
    }

    #[test]
    fn deserialize_content_null() {
        test_util::test_xml_deserialize(
            r#"
                <ContentId name="something">
                    <null></null>
                </ContentId>
            "#,
            &Content::new(),
        );
    }

    #[test]
    fn serialize_content_url() {
        test_util::test_xml_serialize(
            r#"
                <ContentId name="foo">
                    <url>Some URL</url>
                </ContentId>
            "#,
            &Content::from("Some URL"),
        );
    }

    #[test]
    fn serialize_content_null() {
        test_util::test_xml_serialize(
            r#"
                <ContentId name="foo">
                    <null></null>
                </ContentId>
            "#,
            &Content::new(),
        );
    }
}
