use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

static VECTOR2_TAGS: [&str; 2] = ["X", "Y"];
static VECTOR3_TAGS: [&str; 3] = ["X", "Y", "Z"];

pub struct Vector2Type;

impl XmlType<[f32; 2]> for Vector2Type {
    const XML_TAG_NAME: &'static str = "Vector2";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[f32; 2],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array(value, &VECTOR2_TAGS)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let x: f32 = reader.read_tag_contents("X")?.parse()?;
        let y: f32 = reader.read_tag_contents("Y")?.parse()?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Vector2 {
            value: [x, y],
        })
    }
}

pub struct Vector2int16Type;

impl XmlType<[i16; 2]> for Vector2int16Type {
    const XML_TAG_NAME: &'static str = "Vector2int16";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[i16; 2],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array(value, &VECTOR2_TAGS)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let x: i16 = reader.read_tag_contents("X")?.parse()?;
        let y: i16 = reader.read_tag_contents("Y")?.parse()?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Vector2int16 {
            value: [x, y],
        })
    }
}

pub struct Vector3Type;

impl XmlType<[f32; 3]> for Vector3Type {
    const XML_TAG_NAME: &'static str = "Vector3";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[f32; 3],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array(value, &VECTOR3_TAGS)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let x: f32 = reader.read_tag_contents("X")?.parse()?;
        let y: f32 = reader.read_tag_contents("Y")?.parse()?;
        let z: f32 = reader.read_tag_contents("Z")?.parse()?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Vector3 {
            value: [x, y, z],
        })
    }
}

pub struct Vector3int16Type;

impl XmlType<[i16; 3]> for Vector3int16Type {
    const XML_TAG_NAME: &'static str = "Vector3int16";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &[i16; 3],
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        writer.write_tag_array(value, &VECTOR3_TAGS)?;
        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let x: i16 = reader.read_tag_contents("X")?.parse()?;
        let y: i16 = reader.read_tag_contents("Y")?.parse()?;
        let z: i16 = reader.read_tag_contents("Z")?.parse()?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Vector3int16 {
            value: [x, y, z],
        })
    }
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
        Vector2Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Vector2Type::read_xml(&mut reader).unwrap();

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
        Vector2int16Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Vector2int16Type::read_xml(&mut reader).unwrap();

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
        Vector3Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Vector3Type::read_xml(&mut reader).unwrap();

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
        Vector3int16Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Vector3int16Type::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Vector3int16 {
            value: test_input,
        });
    }
}