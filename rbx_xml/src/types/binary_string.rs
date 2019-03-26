use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct BinaryStringType;

impl XmlType<[u8]> for BinaryStringType {
    const XML_TAG_NAME: &'static str = "BinaryString";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[u8],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write(XmlWriteEvent::cdata(&base64::encode(value)))?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        let contents = reader.read_tag_contents(Self::XML_TAG_NAME)?;

        // Roblox wraps base64 BinaryString data at the 72 byte mark. The base64
        // crate doesn't like that very much.
        let contents = contents.replace("\n", "");

        let value = base64::decode(&contents)?;

        Ok(RbxValue::BinaryString {
            value,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_binary_string() {
        let test_value = b"\x00\x01hello,\n\x7Fworld, from a fairly sizable binary string literal.\n";

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
}