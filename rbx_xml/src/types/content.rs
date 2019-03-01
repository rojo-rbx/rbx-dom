use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_content<W: Write>(writer: &mut XmlEventWriter<W>, name: &str, value: &str) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Content").attr("name", name))?;

    if value.len() == 0 {
        // This doesn't feel like a great XML idiom
        writer.write(XmlWriteEvent::start_element("null"))?;
        writer.write(XmlWriteEvent::end_element())?;
    } else {
        writer.write(XmlWriteEvent::start_element("url"))?;
        writer.write(XmlWriteEvent::characters(&value))?;
        writer.write(XmlWriteEvent::end_element())?;
    }

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_content<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Content")?;

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

    reader.expect_end_with_name("Content")?;

    Ok(RbxValue::Content { value })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_content_url() {
        let _ = env_logger::try_init();

        let test_value = "url://not/really/a/url";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_content(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_content(&mut reader).unwrap();

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
        serialize_content(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_content(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Content {
            value: test_value.to_owned(),
        });
    }

    #[test]
    fn de_content_url() {
        let buffer = r#"
            <Content name="something">
                <url>Some URL</url>
            </Content>
        "#;

        let mut reader = EventIterator::from_source(buffer.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_content(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Content {
            value: String::from("Some URL"),
        });
    }

    #[test]
    fn de_content_null() {
        let buffer = r#"
            <Content name="something">
                <null></null>
            </Content>
        "#;

        let mut reader = EventIterator::from_source(buffer.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_content(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Content {
            value: String::new(),
        });
    }
}