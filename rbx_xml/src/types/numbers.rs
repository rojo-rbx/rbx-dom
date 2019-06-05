use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    error::{EncodeError, DecodeError},
    deserializer_core::{XmlEventReader},
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

macro_rules! float_type {
    ($rbx_type: ident, $type_struct: ident, $rust_type: ty, $xml_name: expr, $reader_method: ident) => {
        pub struct $type_struct;

        impl XmlType<$rust_type> for $type_struct {
            const XML_TAG_NAME: &'static str = $xml_name;

            fn write_xml<W: Write>(
                writer: &mut XmlEventWriter<W>,
                name: &str,
                value: &$rust_type,
            ) -> Result<(), EncodeError> {
                writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
                writer.write_characters_f64(*value as f64)?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            fn read_xml<R: Read>(
                reader: &mut XmlEventReader<R>,
            ) -> Result<RbxValue, DecodeError> {
                Ok(RbxValue::$rbx_type {
                    value: reader.$reader_method(Self::XML_TAG_NAME)?,
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
                let value: $rust_type = reader.read_tag_contents(Self::XML_TAG_NAME)?
                    .parse().map_err(|e| reader.error(e))?;

                Ok(RbxValue::$rbx_type {
                    value,
                })
            }
        }
    };
}

float_type!(Float32, Float32Type, f32, "float", read_tag_contents_f32);
float_type!(Float64, Float64Type, f64, "double", read_tag_contents_f64);
int_type!(Int32, Int32Type, i32, "int");
int_type!(Int64, Int64Type, i64, "int64");

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        core::XmlType,
        deserializer_core::XmlEventReader,
        test_util,
    };

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
    fn test_inf_and_nan_deserialize() {
        test_util::test_xml_deserialize::<Float32Type, _>(
            r#"<float name="foo">INF</float>"#,
            RbxValue::Float32 { value: std::f32::INFINITY },
        );

        test_util::test_xml_deserialize::<Float32Type, _>(
            r#"<float name="foo">-INF</float>"#,
            RbxValue::Float32 { value: std::f32::NEG_INFINITY },
        );

        // Can't just use test_util::test_xml_deserialize, because NaN != NaN!
        let mut reader = XmlEventReader::from_source(
            r#"<float name="foo">NAN</float>"#.as_bytes()
        );
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = Float32Type::read_xml(&mut reader).unwrap();
        match value {
            RbxValue::Float32 { value } => assert!(value.is_nan()),
            _ => panic!("Did not get a float32 from NaN test"),
        };
    }

    #[test]
    fn test_inf_and_nan_serialize() {
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