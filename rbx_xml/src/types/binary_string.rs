use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
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
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let contents = match reader.next().ok_or(DecodeError::Message("Unexpected EOF"))?? {
            XmlReadEvent::Characters(contents) => contents,
            XmlReadEvent::EndElement { name } => {
                if name.local_name == Self::XML_TAG_NAME {
                    return Ok(RbxValue::BinaryString {
                        value: Vec::new()
                    });
                } else {
                    return Err(DecodeError::Message("Unexpected closing tag"));
                }
            },
            _ => return Err(DecodeError::Message("Unexpected stuff in BinaryString")),
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

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

    #[test]
    fn round_trip_binary_string() {
        let _ = env_logger::try_init();

        static TEST_VALUE: &[u8] = b"\x00\x01hello,\n\x7Fworld, from a fairly sizable binary string literal.\n";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        BinaryStringType::write_xml(&mut writer, "foo", TEST_VALUE).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = BinaryStringType::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::BinaryString {
            value: TEST_VALUE.to_owned(),
        });
    }
}