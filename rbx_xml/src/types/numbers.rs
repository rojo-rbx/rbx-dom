use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
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
                reader: &mut XmlEventReader<R>,
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

    use crate::test_util;

    #[test]
    fn round_trip_f32() {
        test_util::test_xml_round_trip::<Float32Type, _>(
            &123456.0,
            RbxValue::Float32 {
                value: 123456.0,
            }
        );
    }

    #[test]
    fn round_trip_f64() {
        test_util::test_xml_round_trip::<Float64Type, _>(
            &123456.0,
            RbxValue::Float64 {
                value: 123456.0,
            }
        );
    }

    #[test]
    fn round_trip_i32() {
        test_util::test_xml_round_trip::<Int32Type, _>(
            &-4654321,
            RbxValue::Int32 {
                value: -4654321,
            }
        );
    }

    #[test]
    fn round_trip_i64() {
        test_util::test_xml_round_trip::<Int64Type, _>(
            &281474976710656,
            RbxValue::Int64 {
                value: 281474976710656,
            }
        );
    }
}