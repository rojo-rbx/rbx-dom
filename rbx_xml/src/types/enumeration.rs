use std::io::{Read, Write};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_enum<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: u32,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("token").attr("name", name))?;
    writer.write(XmlWriteEvent::characters(&value.to_string()))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_enum<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let value: u32 = reader.read_tag_contents("token")?.parse()?;

    Ok(RbxValue::Enum {
        value,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip() {
        let _ = env_logger::try_init();

        let test_input: u32 = 4654321;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_enum(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_enum(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Enum {
            value: test_input,
        });
    }
}