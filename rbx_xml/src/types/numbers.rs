use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

macro_rules! float_type {
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
                let value = *value;
                if value == 1.0 / 0.0 {
                    writer.write_characters("INF")?;
                } else if value == -1.0 / 0.0 {
                    writer.write_characters("-INF")?;
                } else if value.is_nan() {
                    writer.write_characters("NAN")?;
                } else {
                    writer.write_characters(value)?;
                }
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            #[allow(clippy::zero_divided_by_zero)]
            fn read_xml<R: Read>(
                reader: &mut XmlEventReader<R>,
            ) -> Result<RbxValue, DecodeError> {
                let value_text = reader.read_tag_contents(Self::XML_TAG_NAME)?;
                let value: $rust_type = match value_text.as_str() {
                    "INF" => 1.0 / 0.0,
                    "-INF" => -1.0 / 0.0,
                    "NAN" => 0.0 / 0.0,
                    number => number.parse()?,
                };

                Ok(RbxValue::$rbx_type {
                    value,
                })
            }
        }
    };
}


macro_rules! int_type {
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

float_type!(Float32, Float32Type, f32, "float");
float_type!(Float64, Float64Type, f64, "double");
int_type!(Int32, Int32Type, i32, "int");
int_type!(Int64, Int64Type, i64, "int64");

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

    #[test]
    fn test_inf_and_nan() {
        test_util::test_xml_serialize::<Float32Type, _>(
            r#"<float name="foo">INF</float>"#,
            &std::f32::INFINITY,
        );

        test_util::test_xml_serialize::<Float32Type, _>(
            r#"<float name="foo">-INF</float>"#,
            &std::f32::NEG_INFINITY,
        );

        test_util::test_xml_serialize::<Float32Type, _>(
            r#"<float name="foo">NAN</float>"#,
            &std::f32::NAN,
        );
    }
}