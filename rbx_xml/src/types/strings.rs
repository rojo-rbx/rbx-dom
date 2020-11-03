use std::io::{Read, Write};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for String {
    const XML_TAG_NAME: &'static str = "string";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_string(self)
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        reader.read_characters()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProtectedStringDummy(pub String);

impl XmlType for ProtectedStringDummy {
    const XML_TAG_NAME: &'static str = "ProtectedString";

    fn write_xml<W: Write>(&self, _writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        panic!("ProtectedString values are only read, never written.");
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        Ok(ProtectedStringDummy(reader.read_characters()?))
    }
}

#[cfg(test)]
mod test {
    use super::ProtectedStringDummy;

    use crate::test_util;

    #[test]
    fn round_trip_string() {
        test_util::test_xml_round_trip(&"Hello,\n\tworld!\n".to_owned());
    }

    #[test]
    fn round_trip_empty_string() {
        test_util::test_xml_round_trip(&String::new());
    }

    #[test]
    fn serialize_simple_string() {
        test_util::test_xml_serialize(
            r#"
                <string name="foo">Hello!</string>
            "#,
            &"Hello!".to_owned(),
        );
    }

    #[test]
    fn serialize_sensitive_whitespace_string() {
        test_util::test_xml_serialize(
            "<string name=\"foo\"><![CDATA[hello\n]]></string>",
            &"hello\n".to_owned(),
        );
    }

    #[test]
    fn round_trip_just_whitespace_string() {
        test_util::test_xml_round_trip(&"\n\t".to_owned());
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

        test_util::test_xml_deserialize(&test_source, &ProtectedStringDummy(test_value.to_owned()));
    }
}
