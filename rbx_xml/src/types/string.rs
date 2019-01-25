use std::io::{Read, Write};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_string<W: Write>(writer: &mut XmlEventWriter<W>, name: &str, value: &str) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("string").attr("name", name))?;
    writer.write(XmlWriteEvent::characters(&value))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_string<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("string")?;

    let value = read_event!(reader, XmlReadEvent::Characters(value) => RbxValue::String { value: value.to_owned() });

    reader.expect_end_with_name("string")?;

    Ok(value)
}

pub fn serialize_binary_string<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: &[u8]
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("BinaryString").attr("name", name))?;
    writer.write(XmlWriteEvent::cdata(&base64::encode(value)))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_binary_string<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("BinaryString")?;
    let contents = read_event!(reader, XmlReadEvent::Characters(value) => value);
    reader.expect_end_with_name("BinaryString")?;

    let value = base64::decode(&contents)?;

    Ok(RbxValue::BinaryString {
        value,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_string() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\nworld!\n";

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_string(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_string(&mut reader).unwrap();

        assert_eq!(value, RbxValue::String {
            value: test_value.to_owned(),
        });
    }

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