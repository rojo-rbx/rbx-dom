use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

macro_rules! number_type {
    ($rbx_type: ident, $type_struct: ident, $rust_type: ty, $xml_name: expr) => {
        pub struct $type_struct;

        impl XmlType<$rust_type> for $type_struct {
            const XML_TAG_NAME: &'static str = $xml_name;

            fn write_xml<W: Write>(
                writer: &mut XmlEventWriter<W>,
                name: &str,
                value: &$rust_type,
            ) -> Result<(), EncodeError> {
                writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
                writer.write_characters(*value)?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            fn read_xml<R: Read>(
                reader: &mut EventIterator<R>,
            ) -> Result<RbxValue, DecodeError> {
                let value: $rust_type = reader.read_tag_contents(Self::XML_TAG_NAME)?.parse()?;

                Ok(RbxValue::$rbx_type {
                    value,
                })
            }
        }
    };
}

number_type!(Float32, Float32Type, f32, "float");
number_type!(Float64, Float64Type, f64, "double");
number_type!(Int32, Int32Type, i32, "int");
number_type!(Int64, Int64Type, i64, "int64");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_f32() {
        let _ = env_logger::try_init();

        let test_input = 123456.0;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        Float32Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Float32Type::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Float32 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_f64() {
        let _ = env_logger::try_init();

        let test_input = 123456.0;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        Float64Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Float64Type::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Float64 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_i32() {
        let _ = env_logger::try_init();

        let test_input = -4654321;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        Int32Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Int32Type::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Int32 {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_i64() {
        let _ = env_logger::try_init();

        let test_input = 281474976710656;
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        Int64Type::write_xml(&mut writer, "foo", &test_input).unwrap();

        println!("{}", std::str::from_utf8(&buffer).unwrap());

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Int64Type::read_xml(&mut reader).unwrap();

        assert_eq!(value, RbxValue::Int64 {
            value: test_input,
        });
    }
}