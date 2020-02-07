use std::io::{Read, Write};

use rbx_dom_weak::types::{BinaryString, Variant};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

impl XmlType for BinaryString {
    const XML_TAG_NAME: &'static str = "BinaryString";

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        // FIXME: BinaryString should have an is_empty method.
        let contents: &[u8] = self.as_ref();
        if !contents.is_empty() {
            writer.write(XmlWriteEvent::cdata(&base64::encode(self)))?;
        }

        writer.end_element()?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<BinaryString, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;
        let value = reader.read_base64_characters()?;
        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_binary_string() {
        let test_value =
            b"\x00\x01hello,\n\x7Fworld, from a fairly sizable binary string literal.\n";

        let wrapped_value = RbxValue::BinaryString {
            value: test_value.to_vec(),
        };

        test_util::test_xml_round_trip::<BinaryStringType, _>(test_value, wrapped_value);
    }

    #[test]
    fn round_trip_empty() {
        let test_value = b"";

        let wrapped_value = RbxValue::BinaryString {
            value: test_value.to_vec(),
        };

        test_util::test_xml_round_trip::<BinaryStringType, _>(test_value, wrapped_value);
    }

    #[test]
    fn decode_simple() {
        test_util::test_xml_deserialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\">SGVsbG8sIHdvcmxkIQ==</BinaryString>",
            RbxValue::BinaryString {
                value: "Hello, world!".into(),
            },
        );
    }

    #[test]
    fn decode_lf() {
        test_util::test_xml_deserialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\">SGVsbG8s\nIHdv\n\ncmxkIQ==</BinaryString>",
            RbxValue::BinaryString {
                value: "Hello, world!".into(),
            },
        );
    }

    #[test]
    fn decode_crlf() {
        test_util::test_xml_deserialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\">SGVsbG8s\r\nIHdv\r\n\r\ncmxk\nIQ==</BinaryString>",
            RbxValue::BinaryString {
                value: "Hello, world!".into(),
            },
        );
    }

    #[test]
    fn decode_spaces() {
        test_util::test_xml_deserialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\">SGVsbG8s IHdv  cmxkIQ= =</BinaryString>",
            RbxValue::BinaryString {
                value: "Hello, world!".into(),
            },
        );
    }

    #[test]
    fn cdata_serialize() {
        test_util::test_xml_serialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\"><![CDATA[SGVsbG8sIHdvcmxkIQ==]]></BinaryString>",
            b"Hello, world!",
        );
    }

    #[test]
    fn no_cdata_empty() {
        test_util::test_xml_serialize::<BinaryStringType, _>(
            "<BinaryString name=\"foo\"></BinaryString>",
            b"",
        );
    }
}
