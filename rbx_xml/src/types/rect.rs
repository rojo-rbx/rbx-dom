use std::io::{Read, Write};

use rbx_dom_weak::{RbxValue, Rect};

use crate::{
    core::XmlType,
    deserializer::{DecodeError, XmlEventReader},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub struct RectType;

impl XmlType<Rect> for RectType {
    const XML_TAG_NAME: &'static str = "Rect2D";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &Rect,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        writer.write(XmlWriteEvent::start_element("min"))?;
        writer.write_tag_characters("X", value.min.0)?;
        writer.write_tag_characters("Y", value.min.1)?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::start_element("max"))?;
        writer.write_tag_characters("X", value.max.0)?;
        writer.write_tag_characters("Y", value.max.1)?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(
        reader: &mut XmlEventReader<R>,
    ) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        reader.expect_start_with_name("min")?;
        let x_min: f32 = reader.read_tag_contents("X")?.parse()?;
        let y_min: f32 = reader.read_tag_contents("Y")?.parse()?;
        reader.expect_end_with_name("min")?;

        reader.expect_start_with_name("max")?;
        let x_max: f32 = reader.read_tag_contents("X")?.parse()?;
        let y_max: f32 = reader.read_tag_contents("Y")?.parse()?;
        reader.expect_end_with_name("max")?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Rect {
            value: Rect {
                min: (x_min, y_min),
                max: (x_max, y_max),
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_rect() {
        let test_input = Rect {
            min: (-12.5, 718.5),
            max: (100.0, 200.0),
        };

        test_util::test_xml_round_trip::<RectType, _>(
            &test_input,
            RbxValue::Rect {
                value: test_input,
            }
        );
    }

    #[test]
    fn deserialize_rect() {
        test_util::test_xml_deserialize::<RectType, _>(
            r#"
                <Rect2D name="SliceCenter">
                    <min>
                        <X>12.5</X>
                        <Y>-30.5</Y>
                    </min>
                    <max>
                        <X>23</X>
                        <Y>9</Y>
                    </max>
                </Rect2D>
            "#,
            RbxValue::Rect {
                value: Rect {
                    min: (12.5, -30.5),
                    max: (23.0, 9.0),
                },
            }
        );
    }

    #[test]
    fn serialize_rect() {
        test_util::test_xml_serialize::<RectType, _>(
            r#"
                <Rect2D name="foo">
                    <min>
                        <X>12.5</X>
                        <Y>-30.5</Y>
                    </min>
                    <max>
                        <X>23</X>
                        <Y>9</Y>
                    </max>
                </Rect2D>
            "#,
            &Rect {
                min: (12.5, -30.5),
                max: (23.0, 9.0),
            }
        );
    }
}