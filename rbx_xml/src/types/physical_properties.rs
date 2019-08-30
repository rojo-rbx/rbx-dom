use std::io::{Read, Write};

use rbx_dom_weak::{PhysicalProperties, RbxValue};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

pub struct PhysicalPropertiesType;

impl XmlType<Option<PhysicalProperties>> for PhysicalPropertiesType {
    const XML_TAG_NAME: &'static str = "PhysicalProperties";

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &Option<PhysicalProperties>,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;

        match value {
            Some(properties) => {
                writer.write_tag_characters("CustomPhysics", "true")?;
                writer.write_tag_characters_f32("Density", properties.density)?;
                writer.write_tag_characters_f32("Friction", properties.friction)?;
                writer.write_tag_characters_f32("Elasticity", properties.elasticity)?;
                writer.write_tag_characters_f32("FrictionWeight", properties.friction_weight)?;
                writer
                    .write_tag_characters_f32("ElasticityWeight", properties.elasticity_weight)?;
            }
            None => {
                writer.write_tag_characters("CustomPhysics", "false")?;
            }
        }

        writer.write(XmlWriteEvent::end_element())?;

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<RbxValue, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;

        let has_custom_physics = reader.read_tag_contents("CustomPhysics")?;

        let value = match has_custom_physics.as_str() {
            "true" => {
                let density: f32 = reader.read_tag_contents_f32("Density")?;
                let friction: f32 = reader.read_tag_contents_f32("Friction")?;
                let elasticity: f32 = reader.read_tag_contents_f32("Elasticity")?;
                let friction_weight: f32 = reader.read_tag_contents_f32("FrictionWeight")?;
                let elasticity_weight: f32 = reader.read_tag_contents_f32("ElasticityWeight")?;

                Some(PhysicalProperties {
                    density,
                    friction,
                    elasticity,
                    friction_weight,
                    elasticity_weight,
                })
            }
            "false" => None,
            _ => {
                return Err(reader.error(DecodeErrorKind::InvalidContent(
                    "expected CustomPhysics to be true or false",
                )));
            }
        };

        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(RbxValue::PhysicalProperties { value })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_physical_properties_normal() {
        test_util::test_xml_round_trip::<PhysicalPropertiesType, _>(
            &None,
            RbxValue::PhysicalProperties { value: None },
        );
    }

    #[test]
    fn round_trip_physical_properties_custom() {
        let test_value = Some(PhysicalProperties {
            density: 0.5,
            friction: 1.0,
            elasticity: 1.5,
            friction_weight: 2.0,
            elasticity_weight: 2.5,
        });

        test_util::test_xml_round_trip::<PhysicalPropertiesType, _>(
            &test_value,
            RbxValue::PhysicalProperties { value: test_value },
        );
    }

    #[test]
    fn deserialize_physical_properties_normal() {
        test_util::test_xml_deserialize::<PhysicalPropertiesType, _>(
            r#"
                <PhysicalProperties name="CustomPhysicalProperties">
                    <CustomPhysics>false</CustomPhysics>
                </PhysicalProperties>
            "#,
            RbxValue::PhysicalProperties { value: None },
        );
    }

    #[test]
    fn deserialize_physical_properties_custom() {
        test_util::test_xml_deserialize::<PhysicalPropertiesType, _>(
            r#"
                <PhysicalProperties name="CustomPhysicalProperties">
                    <CustomPhysics>true</CustomPhysics>
                    <Density>0.5</Density>
                    <Friction>1</Friction>
                    <Elasticity>1.5</Elasticity>
                    <FrictionWeight>2</FrictionWeight>
                    <ElasticityWeight>2.5</ElasticityWeight>
                </PhysicalProperties>
            "#,
            RbxValue::PhysicalProperties {
                value: Some(PhysicalProperties {
                    density: 0.5,
                    friction: 1.0,
                    elasticity: 1.5,
                    friction_weight: 2.0,
                    elasticity_weight: 2.5,
                }),
            },
        );
    }

    #[test]
    fn serialize_physical_properties_normal() {
        test_util::test_xml_serialize::<PhysicalPropertiesType, _>(
            r#"
                <PhysicalProperties name="foo">
                    <CustomPhysics>false</CustomPhysics>
                </PhysicalProperties>
            "#,
            &None,
        );
    }

    #[test]
    fn serialize_physical_properties_custom() {
        test_util::test_xml_serialize::<PhysicalPropertiesType, _>(
            r#"
                <PhysicalProperties name="foo">
                    <CustomPhysics>true</CustomPhysics>
                    <Density>0.5</Density>
                    <Friction>1</Friction>
                    <Elasticity>1.5</Elasticity>
                    <FrictionWeight>2</FrictionWeight>
                    <ElasticityWeight>2.5</ElasticityWeight>
                </PhysicalProperties>
            "#,
            &Some(PhysicalProperties {
                density: 0.5,
                friction: 1.0,
                elasticity: 1.5,
                friction_weight: 2.0,
                elasticity_weight: 2.5,
            }),
        );
    }
}
