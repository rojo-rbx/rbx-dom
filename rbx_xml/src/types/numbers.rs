use std::io::{Read, Write};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

macro_rules! float_type {
    ($rbx_type: ident, $rust_type: ty, $xml_name: expr, $reader_method: ident) => {
        impl XmlType for $rust_type {
            const XML_TAG_NAME: &'static str = $xml_name;

            fn write_xml<W: Write>(
                &self,
                writer: &mut XmlEventWriter<W>,
                name: &str,
            ) -> Result<(), EncodeError> {
                writer
                    .write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
                writer.write_characters_f64(*self as f64)?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
                reader.$reader_method(Self::XML_TAG_NAME)
            }
        }
    };
}

macro_rules! int_type {
    ($rbx_type: ident, $rust_type: ty, $xml_name: expr) => {
        impl XmlType for $rust_type {
            const XML_TAG_NAME: &'static str = $xml_name;

            fn write_xml<W: Write>(
                &self,
                writer: &mut XmlEventWriter<W>,
                name: &str,
            ) -> Result<(), EncodeError> {
                writer
                    .write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
                writer.write_characters(*self)?;
                writer.write(XmlWriteEvent::end_element())?;

                Ok(())
            }

            fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
                reader
                    .read_tag_contents(Self::XML_TAG_NAME)?
                    .parse()
                    .map_err(|e| reader.error(e))
            }
        }
    };
}

float_type!(Float32, f32, "float", read_tag_contents_f32);
float_type!(Float64, f64, "double", read_tag_contents_f64);
int_type!(Int32, i32, "int");
int_type!(Int64, i64, "int64");

#[cfg(test)]
mod test {
    use super::*;

    use crate::{core::XmlType, deserializer_core::XmlEventReader, test_util};

    #[test]
    fn round_trip_f32() {
        test_util::test_xml_round_trip(&123456.0f32);
    }

    #[test]
    fn round_trip_f64() {
        test_util::test_xml_round_trip(&123456.0f64);
    }

    #[test]
    fn round_trip_i32() {
        test_util::test_xml_round_trip(&-4654321i32);
    }

    #[test]
    fn round_trip_i64() {
        test_util::test_xml_round_trip(&281474976710656i64);
    }

    #[test]
    fn test_inf_and_nan_deserialize() {
        test_util::test_xml_deserialize(r#"<float name="foo">INF</float>"#, &std::f32::INFINITY);

        test_util::test_xml_deserialize(
            r#"<float name="foo">-INF</float>"#,
            &std::f32::NEG_INFINITY,
        );

        // Can't just use test_util::test_xml_deserialize, because NaN != NaN!
        let mut reader = XmlEventReader::from_source(r#"<float name="foo">NAN</float>"#.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = f32::read_xml(&mut reader).unwrap();
        assert!(value.is_nan());
    }

    #[test]
    fn test_inf_and_nan_serialize() {
        test_util::test_xml_serialize(r#"<float name="foo">INF</float>"#, &std::f32::INFINITY);

        test_util::test_xml_serialize(r#"<float name="foo">-INF</float>"#, &std::f32::NEG_INFINITY);

        test_util::test_xml_serialize(r#"<float name="foo">NAN</float>"#, &std::f32::NAN);
    }
}
