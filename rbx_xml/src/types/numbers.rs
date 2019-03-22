use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

macro_rules! number_type {
    ($name:ident: $ty:ty, $value:ident, $element:expr, $test:expr) => {
        pub mod $name {
            use super::*;

            pub fn serialize<W: Write>(
                writer: &mut XmlEventWriter<W>,
                name: &str,
                value: $ty,
            ) -> Result<(), EncodeError> {
                writer.write(XmlWriteEvent::start_element($element).attr("name", name))?;
                writer.write(XmlWriteEvent::characters(&value.to_string()))?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            pub fn deserialize<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
                let value: $ty = reader.read_tag_contents($element)?.parse()?;

                Ok(RbxValue::$value {
                    value,
                })
            }

            #[cfg(test)]
            #[test]
            fn round_trip() {
                let _ = env_logger::try_init();

                let test_input: $ty = $test;
                let mut buffer = Vec::new();

                let mut writer = XmlEventWriter::from_output(&mut buffer);
                serialize(&mut writer, "foo", test_input).unwrap();

                println!("{}", std::str::from_utf8(&buffer).unwrap());

                let mut reader = EventIterator::from_source(buffer.as_slice());
                reader.next().unwrap().unwrap(); // Eat StartDocument event
                let value = deserialize(&mut reader).unwrap();

                assert_eq!(value, RbxValue::$value {
                    value: test_input,
                });
            }
        }
    };
}

number_type!(float32: f32, Float32, "float", 123456.0);
number_type!(float64: f64, Float64, "double", 123456.0);
number_type!(int32: i32, Int32, "int", -4654321);
number_type!(int64: i64, Int64, "int64", 281474976710656);
