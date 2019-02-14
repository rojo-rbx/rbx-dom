use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_float32<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: f32,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("float").attr("name", name))?;
    writer.write(XmlWriteEvent::characters(&value.to_string()))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_float32<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let value: f32 = reader.read_tag_contents("float")?.parse()?;

    Ok(RbxValue::Float32 {
        value,
    })
}

pub fn serialize_int32<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: i32,
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("int").attr("name", name))?;
    writer.write(XmlWriteEvent::characters(&value.to_string()))?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_int32<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    let value: i32 = reader.read_tag_contents("int")?.parse()?;

    Ok(RbxValue::Int32 {
        value,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_float32() {
        let _ = env_logger::try_init();

        let test_input: f32 = 123456.0;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_float32(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_float32(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Float32 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_int32() {
        let _ = env_logger::try_init();

        let test_input: i32 = -4654321;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_int32(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_int32(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Int32 {
            value: test_input,
        });
    }
}