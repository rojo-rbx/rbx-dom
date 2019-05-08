use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    core::NewXmlType as XmlType,
    error::{EncodeError, DecodeError},
    deserializer_core::{XmlEventReader},
    serializer_core::{XmlWriteEvent, XmlEventWriter},
};

pub struct UDimType;
type UDimValue = (f32, i32);

impl XmlType<UDimValue> for UDimType {
    const XML_TAG_NAME: &'static str = "UDim";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &UDimValue,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        writer.write_tag_characters("S", value.0)?;
        writer.write_tag_characters("O", value.1)?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let scale: f32 = reader.read_tag_contents_parse("S")?;
        let offset: i32 = reader.read_tag_contents_parse("O")?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::UDim {
            value: (scale, offset),
        })
    }
}

pub struct UDim2Type;
type UDim2Value = (f32, i32, f32, i32);

impl XmlType<UDim2Value> for UDim2Type {
    const XML_TAG_NAME: &'static str = "UDim2";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &UDim2Value,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        writer.write_tag_characters("XS", value.0)?;
        writer.write_tag_characters("XO", value.1)?;
        writer.write_tag_characters("YS", value.2)?;
        writer.write_tag_characters("YO", value.3)?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let x_scale: f32 = reader.read_tag_contents_parse("XS")?;
        let x_offset: i32 = reader.read_tag_contents_parse("XO")?;
        let y_scale: f32 = reader.read_tag_contents_parse("YS")?;
        let y_offset: i32 = reader.read_tag_contents_parse("YO")?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::UDim2 {
            value: (x_scale, x_offset, y_scale, y_offset),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_udim() {
        let test_input = (0.5, 1);

        test_util::test_xml_round_trip::<UDimType, _>(
            &test_input,
            RbxValue::UDim {
                value: test_input,
            }
        );
    }

    #[test]
    fn round_trip_udim2() {
        let test_input = (0.5, 1, 1.5, 2);

        test_util::test_xml_round_trip::<UDim2Type, _>(
            &test_input,
            RbxValue::UDim2 {
                value: test_input,
            }
        );
    }

    #[test]
    fn de_udim() {
        test_util::test_xml_deserialize::<UDimType, _>(
            r#"
                <UDim>
                    <S>0.5</S>
                    <O>1</O>
                </UDim>
            "#,
            RbxValue::UDim {
                value: (0.5, 1),
            }
        );
    }

    #[test]
    fn de_udim2() {
        test_util::test_xml_deserialize::<UDim2Type, _>(
            r#"
                <UDim2>
                    <XS>0.5</XS>
                    <XO>1</XO>
                    <YS>1.5</YS>
                    <YO>2</YO>
                </UDim2>
            "#,
            RbxValue::UDim2 {
                value: (0.5, 1, 1.5, 2),
            }
        );
    }
}