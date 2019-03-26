use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct ContentType;

impl XmlType<str> for ContentType {
    const XML_TAG_NAME: &'static str = "Content";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        if value.len() == 0 {
            // This doesn't feel like a great XML idiom
            writer.write(XmlWriteEvent::start_element("null"))?;
            writer.write(XmlWriteEvent::end_element())?;
        } else {
            writer.write(XmlWriteEvent::start_element("url"))?;
            writer.write_string(value)?;
            writer.write(XmlWriteEvent::end_element())?;
        }

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let value = read_event!(reader, XmlReadEvent::StartElement { name, .. } => {
            match name.local_name.as_str() {
                "null" => {
                    reader.expect_end_with_name("null")?;

                    String::new()
                },
                "url" => {
                    let value = read_event!(reader, XmlReadEvent::Characters(value) => value);
                    reader.expect_end_with_name("url")?;

                    value.to_owned()
                },
                _ => return Err(DecodeError::Message("Unexpected tag inside Content")),
            }
        });

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Content { value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_content_url() {
        let _ = env_logger::try_init();

        let test_value = "url://not/really/a/url";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        ContentType::write_xml(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = ContentType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Content {
            value: test_value.to_owned(),
        });
    }

    #[test]
    fn round_trip_content_null() {
        let _ = env_logger::try_init();

        let test_value = "";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        ContentType::write_xml(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = ContentType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Content {
            value: test_value.to_owned(),
        });
    }

    #[test]
    fn de_content_url() {
        test_util::test_xml_deserialize::<ContentType, _>(
            r#"
                <Content name="something">
                    <url>Some URL</url>
                </Content>
            "#,
            RbxValue::Content {
                value: String::from("Some URL"),
            }
        );
    }

    #[test]
    fn se_content_url() {
        test_util::test_xml_serialize::<ContentType, _>(
            r#"
                <Content name="foo">
                    <url>Some URL</url>
                </Content>
            "#,
            "Some URL"
        );
    }

    #[test]
    fn de_content_null() {
        test_util::test_xml_deserialize::<ContentType, _>(
            r#"
                <Content name="something">
                    <null></null>
                </Content>
            "#,
            RbxValue::Content {
                value: String::new(),
            }
        );
    }
}