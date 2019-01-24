use std::io::{Read, Write};

use xml::{
    writer::{XmlEvent as XmlWriteEvent, EventWriter},
    reader::{XmlEvent as XmlReadEvent},
};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::EncodeError,
};

pub fn serialize_string<W: Write>(writer: &mut EventWriter<W>, name: &str, value: &str) -> Result<(), EncodeError> {
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

#[cfg(test)]
mod test {
    use crate::serializer::create_writer;

    use super::*;

    #[test]
    fn round_trip() {
        let _ = env_logger::try_init();

        let test_value = "Hello,\nworld!\n";

        let mut buffer = Vec::new();

        let mut writer = create_writer(&mut buffer);
        serialize_string(&mut writer, "foo", test_value).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_string(&mut reader).unwrap();

        assert_eq!(value, RbxValue::String {
            value: test_value.to_owned(),
        });
    }
}