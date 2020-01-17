use std::io::{Read, Write};

use rbx_dom_weak::{Ray, RbxValue};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

pub struct RayType;

impl XmlType<Ray> for RayType {
    const XML_TAG_NAME: &'static str = "Ray";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &Ray,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        writer.write(XmlWriteEvent::start_element("origin"))?;
        writer.write_tag_characters_f32("X", value.origin[0])?;
        writer.write_tag_characters_f32("Y", value.origin[1])?;
        writer.write_tag_characters_f32("Z", value.origin[2])?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::start_element("direction"))?;
        writer.write_tag_characters_f32("X", value.direction[0])?;
        writer.write_tag_characters_f32("Y", value.direction[1])?;
        writer.write_tag_characters_f32("Z", value.direction[2])?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        reader.expect_start_with_name("origin")?;
        let x_origin: f32 = reader.read_tag_contents_f32("X")?;
        let y_origin: f32 = reader.read_tag_contents_f32("Y")?;
        let z_origin: f32 = reader.read_tag_contents_f32("Z")?;
        reader.expect_end_with_name("origin")?;

        reader.expect_start_with_name("direction")?;
        let x_direction: f32 = reader.read_tag_contents_f32("X")?;
        let y_direction: f32 = reader.read_tag_contents_f32("Y")?;
        let z_direction: f32 = reader.read_tag_contents_f32("Z")?;
        reader.expect_end_with_name("direction")?;

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::Ray {
            value: Ray {
                origin: [x_origin, y_origin, z_origin],
                direction: [x_direction, y_direction, z_direction],
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_ray() {
        let test_input = Ray {
            origin: [-12.5, 718.5, 3.0],
            direction: [100.0, 200.0, 9000.0],
        };

        test_util::test_xml_round_trip::<RayType, _>(
            &test_input,
            RbxValue::Ray { value: test_input },
        );
    }

    #[test]
    fn deserialize_rect() {
        test_util::test_xml_deserialize::<RayType, _>(
            r#"
                <Ray name="Value">
                    <origin>
                        <X>5.0</X>
                        <Y>10.0</Y>
                        <Z>6.5</Z>
                    </origin>
                    <direction>
                        <X>2.0</X>
                        <Y>300.0</Y>
                        <Z>900.0</Z>
                    </direction>
                </Ray>
            "#,
            RbxValue::Ray {
                value: Ray {
                    origin: [5.0, 10.0, 6.5],
                    direction: [2.0, 300.0, 900.0],
                },
            },
        );
    }

    #[test]
    fn serialize_ray() {
        test_util::test_xml_serialize::<RayType, _>(
            r#"
                <Ray name="foo">
                    <origin>
                        <X>5</X>
                        <Y>10</Y>
                        <Z>6.5</Z>
                    </origin>
                    <direction>
                        <X>2</X>
                        <Y>300</Y>
                        <Z>900</Z>
                    </direction>
                </Ray>
            "#,
            &Ray {
                origin: [5.0, 10.0, 6.5],
                direction: [2.0, 300.0, 900.0],
            },
        );
    }
}
