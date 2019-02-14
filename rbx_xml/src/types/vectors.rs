use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

static VECTOR2_TAGS: [&str; 2] = ["X", "Y"];
static VECTOR3_TAGS: [&str; 3] = ["X", "Y", "Z"];

pub fn serialize_vector2<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [f32; 2],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Vector2").attr("name", name))?;
    writer.write_tag_array(&value, &VECTOR2_TAGS)?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_vector2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("Vector2")?;

    let x: f32 = reader.read_tag_contents("X")?.parse()?;
    let y: f32 = reader.read_tag_contents("Y")?.parse()?;

    reader.expect_end_with_name("Vector2")?;

    Ok(RbxValue::Vector2 {
        value: [x, y],
    })
}

pub fn serialize_vector2int16<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [i16; 2],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Vector2int16").attr("name", name))?;
    writer.write_tag_array(&value, &VECTOR2_TAGS)?;
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

pub fn serialize_vector3<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [f32; 3],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Vector3").attr("name", name))?;
    writer.write_tag_array(&value, &VECTOR3_TAGS)?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
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

pub fn serialize_vector3int16<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: [i16; 3],
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("Vector3int16").attr("name", name))?;
    writer.write_tag_array(&value, &VECTOR3_TAGS)?;
    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
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

    #[test]
    fn round_trip_vector2int16() {
        let _ = env_logger::try_init();

        let test_input: [i16; 2] = [12345, -24321];
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_vector2int16(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_vector2int16(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Vector2int16 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_vector3() {
        let _ = env_logger::try_init();

        let test_input: [f32; 3] = [123.0, 456.0, -52349.0];
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_vector3(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_vector3(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Vector3 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_vector3int16() {
        let _ = env_logger::try_init();

        let test_input: [i16; 3] = [12345, -24321, 321];
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_vector3int16(&mut writer, "foo", test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_vector3int16(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Vector3int16 {
            value: test_input,
        });
    }
}