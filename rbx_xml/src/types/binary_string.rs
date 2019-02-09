use std::io::{Read, Write};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

const BASE64_CONFIG: base64::Config = base64::STANDARD_NO_PAD;

pub fn serialize_binary_string<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: &[u8]
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("BinaryString").attr("name", name))?;
    writer.write(XmlWriteEvent::cdata(&base64::encode_config(value, BASE64_CONFIG)))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_binary_string<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("BinaryString")?;

    let contents = match reader.next().ok_or(DecodeError::Message("Unexpected EOF"))?? {
        XmlReadEvent::Characters(contents) => contents,
        XmlReadEvent::EndElement { name } => {
            if name.local_name == "BinaryString" {
                return Ok(RbxValue::BinaryString {
                    value: Vec::new()
                });
            } else {
                return Err(DecodeError::Message("Unexpected closing tag"));
            }
        },
        _ => return Err(DecodeError::Message("Unexpected stuff in BinaryString")),
    };

    reader.expect_end_with_name("BinaryString")?;

    let contents = contents.replace("\n", "");

    let value = base64::decode_config(&contents, BASE64_CONFIG)?;

    Ok(RbxValue::BinaryString {
        value,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_binary_string() {
        let _ = env_logger::try_init();

        static TEST_VALUE: &[u8] = b"\x00\x01hello,\n\x7Fworld";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_binary_string(&mut writer, "foo", TEST_VALUE).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_binary_string(&mut reader).unwrap();

        assert_eq!(value, RbxValue::BinaryString {
            value: TEST_VALUE.to_owned(),
        });
    }
}