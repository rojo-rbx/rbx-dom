use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

pub struct StringType;

impl XmlType<str> for StringType {
    const XML_TAG_NAME: &'static str = "string";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_string(value)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<RbxValue, DecodeError> {
        let value = reader.read_tag_contents(Self::XML_TAG_NAME)?;

        Ok(RbxValue::String { value })
    }
}

pub struct ProtectedStringType;

impl XmlType<str> for ProtectedStringType {
    const XML_TAG_NAME: &'static str = "ProtectedString";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_string(value)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<RbxValue, DecodeError> {
        let value = reader.read_tag_contents(Self::XML_TAG_NAME)?;

        Ok(RbxValue::String { value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_string() {
        let test_value = "Hello,\n\tworld!\n";
        let expected_value = RbxValue::String {
            value: test_value.to_owned(),
        };

        test_util::test_xml_round_trip::<StringType, _>(test_value, expected_value);
    }

    #[test]
    fn round_trip_empty_string() {
        let test_value = "";
        let expected_value = RbxValue::String {
            value: test_value.to_owned(),
        };

        test_util::test_xml_round_trip::<StringType, _>(test_value, expected_value);
    }

    #[test]
    fn serialize_simple_string() {
        test_util::test_xml_serialize::<StringType, _>(
            r#"
                <string name="foo">Hello!</string>
            "#,
            "Hello!",
        );
    }

    #[test]
    fn serialize_sensitive_whitespace_string() {
        test_util::test_xml_serialize::<StringType, _>(
            "<string name=\"foo\"><![CDATA[hello\n]]></string>",
            "hello\n",
        );
    }

    #[test]
    fn round_trip_just_whitespace_string() {
        let test_value = "\n\t";
        let expected_value = RbxValue::String {
            value: test_value.to_owned(),
        };

        test_util::test_xml_round_trip::<StringType, _>(test_value, expected_value);
    }

    #[test]
    fn de_protected_string() {
        let test_value = "Hello,\n\tworld!\n";
        let test_source = format!(
            r#"
            <ProtectedString name="something">{}</ProtectedString>
        "#,
            test_value
        );

        test_util::test_xml_deserialize::<ProtectedStringType, _>(
            &test_source,
            RbxValue::String {
                value: test_value.to_owned(),
            },
        );
    }
}
