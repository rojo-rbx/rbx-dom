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

// Protected strings are asymmetrical -- they deserialize to regular string
// values, since their existence is a historical artifact.
pub fn deserialize_protected_string<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("ProtectedString")?;
    let value = read_event!(reader, XmlReadEvent::Characters(value) => RbxValue::String { value: value.to_owned() });
    reader.expect_end_with_name("ProtectedString")?;

    Ok(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_string() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\n\tworld!\n";

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
    fn de_protected_string() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\n\tworld!\n";
        let source = format!("<ProtectedString name=\"foo\">{}</ProtectedString>", test_value);

        let mut reader = EventIterator::from_source(source.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_protected_string(&mut reader).unwrap();

        assert_eq!(value, RbxValue::String {
            value: test_value.to_owned(),
        });
    }
}