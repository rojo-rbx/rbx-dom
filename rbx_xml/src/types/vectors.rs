use std::io::{Read, Write};

use rbx_tree::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn deserialize_vector2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector2")?;

    let x: f32 = reader.read_tag_contents("X")?.parse()?;
    let y: f32 = reader.read_tag_contents("Y")?.parse()?;

    reader.expect_end_with_name("Vector2")?;

    Ok(RbxValue::Vector2 {
        value: [x, y],
    })
}

pub fn serialize_vector2<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [f32; 2],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Vector2").attr("name", name))?;

    writer.write(XmlWriteEvent::start_element("X"))?;
    writer.write(XmlWriteEvent::characters(&value[0].to_string()))?;
    writer.write(XmlWriteEvent::end_element())?;

    writer.write(XmlWriteEvent::start_element("Y"))?;
    writer.write(XmlWriteEvent::characters(&value[1].to_string()))?;
    writer.write(XmlWriteEvent::end_element())?;

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_vector2int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector2int16")?;

    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;

    reader.expect_end_with_name("Vector2int16")?;

    Ok(RbxValue::Vector2int16 {
        value: [x, y],
    })
}

pub fn deserialize_vector3<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector3")?;

    let x: f32 = reader.read_tag_contents("X")?.parse()?;
    let y: f32 = reader.read_tag_contents("Y")?.parse()?;
    let z: f32 = reader.read_tag_contents("Z")?.parse()?;

    reader.expect_end_with_name("Vector3")?;

    Ok(RbxValue::Vector3 {
        value: [x, y, z],
    })
}

pub fn deserialize_vector3int16<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector3int16")?;

    let x: i16 = reader.read_tag_contents("X")?.parse()?;
    let y: i16 = reader.read_tag_contents("Y")?.parse()?;
    let z: i16 = reader.read_tag_contents("Z")?.parse()?;

    reader.expect_end_with_name("Vector3int16")?;

    Ok(RbxValue::Vector3int16 {
        value: [x, y, z],
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_vector2() {
        let _ = env_logger::try_init();

        let test_input: [f32; 2] = [123.0, 456.0];
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_vector2(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_vector2(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Vector2 {
            value: test_input,
        });
    }
}