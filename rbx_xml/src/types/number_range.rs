use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct NumberRangeType;

impl XmlType<(f32, f32)> for NumberRangeType {
    const XML_TAG_NAME: &'static str = "NumberRange";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &(f32, f32),
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        writer.write_characters(value.0)?;
        writer.write(XmlWriteEvent::characters(" "))?;
        writer.write_characters(value.1)?;
        writer.write(XmlWriteEvent::characters(" "))?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let contents = reader.read_characters()?;
        let mut pieces = contents.split(" ").filter(|slice| !slice.is_empty());

        let min: f32 = pieces.next()
            .ok_or(DecodeError::Message("Malformed NumberRange: missing min value"))?
            .parse()?;

        let max: f32 = pieces.next()
            .ok_or(DecodeError::Message("Malformed NumberRange: missing max value"))?
            .parse()?;

        match pieces.next() {
            None => {}
            Some(_) => return Err(DecodeError::Message("Malformed NumberRange: too many values")),
        }

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::NumberRange {
            value: (min, max),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_number_range() {
        let test_input = (103.5, -30.0);

        test_util::test_xml_round_trip::<NumberRangeType, _>(
            &test_input,
            RbxValue::NumberRange {
                value: test_input,
            }
        );
    }

    #[test]
    fn deserialize_number_range() {
        test_util::test_xml_deserialize::<NumberRangeType, _>(
            r#"
                <NumberRange name="foo">80.5 -30 </NumberRange>
            "#,
            RbxValue::NumberRange {
                value: (80.5, -30.0),
            }
        );
    }

    #[test]
    fn serialize_number_range() {
        test_util::test_xml_serialize::<NumberRangeType, _>(
            r#"
                <NumberRange name="foo">80.5 -30 </NumberRange>
            "#,
            &(80.5, -30.0)
        );
    }
}