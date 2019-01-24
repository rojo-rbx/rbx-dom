use std::io::{Read, Write};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, XmlReadEvent, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_bool<W: Write>(writer: &mut XmlEventWriter<W>, name: &str, value: bool) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("bool").attr("name", name))?;

    let value_as_str = if value {
        "true"
    } else {
        "false"
    };

    writer.write(XmlWriteEvent::characters(value_as_str))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_bool<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("bool")?;

    let value = read_event!(reader, XmlReadEvent::Characters(content) => {
        match content.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(DecodeError::Message("invalid boolean value, expected true or false")),
        }
    });

    reader.expect_end_with_name("bool")?;

    Ok(RbxValue::Bool {
        value
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip() {
        let _ = env_logger::try_init();

        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_bool(&mut writer, "foo", true).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_bool(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Bool {
            value: true,
        });
    }
}