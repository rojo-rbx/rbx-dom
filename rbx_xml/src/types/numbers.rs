use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

macro_rules! number_type {
    ($rbx_type: ident, $rust_type: ty, $xml_name: expr, $test_value: expr) => {
        pub struct $rbx_type;

        impl XmlType<$rust_type> for $rbx_type {
            const XML_NAME: &'static str = $xml_name;

            fn write_xml<W: Write>(
                writer: &mut XmlEventWriter<W>,
                name: &str,
                value: &$rust_type,
            ) -> Result<(), EncodeError> {
                writer.write(XmlWriteEvent::start_element(Self::XML_NAME).attr("name", name))?;
                writer.write(XmlWriteEvent::characters(&value.to_string()))?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            fn read_xml<R: Read>(
                reader: &mut EventIterator<R>,
            ) -> Result<RbxValue, DecodeError> {
                let value: $rust_type = reader.read_tag_contents(Self::XML_NAME)?.parse()?;

                Ok(RbxValue::$rbx_type {
                    value,
                })
            }
        }

        // #[cfg(test)]
        // #[test]
        // fn round_trip() {
        //     let _ = env_logger::try_init();

        //     let test_input: $rust_type = $test_value;
        //     let mut buffer = Vec::new();

        //     let mut writer = XmlEventWriter::from_output(&mut buffer);
        //     serialize(&mut writer, "foo", test_input).unwrap();

        //     println!("{}", std::str::from_utf8(&buffer).unwrap());

        //     let mut reader = EventIterator::from_source(buffer.as_slice());
        //     reader.next().unwrap().unwrap(); // Eat StartDocument event
        //     let value = deserialize(&mut reader).unwrap();

        //     assert_eq!(value, RbxValue::$rbx_type {
        //         value: test_input,
        //     });
        // }
    };
}

number_type!(Float32, f32, "float", 123456.0);
number_type!(Float64, f64, "double", 123456.0);
number_type!(Int32, i32, "int", -4654321);
number_type!(Int64, i64, "int64", 281474976710656);
