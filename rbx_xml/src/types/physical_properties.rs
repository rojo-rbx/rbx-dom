use std::io::{Read, Write};

use rbx_dom_weak::types::{CustomPhysicalProperties, PhysicalProperties};
use xml::reader::XmlEvent;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for PhysicalProperties {
    const XML_TAG_NAME: &'static str = "PhysicalProperties";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        match self {
            PhysicalProperties::Custom(properties) => {
                writer.write_value_in_tag(&true, "CustomPhysics")?;
                writer.write_value_in_tag(&properties.density(), "Density")?;
                writer.write_value_in_tag(&properties.friction(), "Friction")?;
                writer.write_value_in_tag(&properties.elasticity(), "Elasticity")?;
                writer.write_value_in_tag(&properties.friction_weight(), "FrictionWeight")?;
                writer.write_value_in_tag(&properties.elasticity_weight(), "ElasticityWeight")?;
                if let Some(value) = properties.acoustic_absorption() {
                    writer.write_value_in_tag(&value, "AcousticAbsorption")?;
                }
            }
            PhysicalProperties::Default => {
                writer.write_value_in_tag(&false, "CustomPhysics")?;
            }
        }

        Ok(())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let has_custom_physics: bool = reader.read_value_in_tag("CustomPhysics")?;

        if has_custom_physics {
            let density: f32 = reader.read_value_in_tag("Density")?;
            let friction: f32 = reader.read_value_in_tag("Friction")?;
            let elasticity: f32 = reader.read_value_in_tag("Elasticity")?;
            let friction_weight: f32 = reader.read_value_in_tag("FrictionWeight")?;
            let elasticity_weight: f32 = reader.read_value_in_tag("ElasticityWeight")?;

            let acoustic_absorption: Option<f32> = match reader.expect_peek()? {
                XmlEvent::StartElement { name, .. } if name.local_name == "AcousticAbsorption" => {
                    Some(reader.read_value_in_tag("AcousticAbsorption")?)
                }
                _ => None,
            };

            Ok(PhysicalProperties::Custom(CustomPhysicalProperties::new(
                density,
                friction,
                elasticity,
                friction_weight,
                elasticity_weight,
                acoustic_absorption,
            )))
        } else {
            Ok(PhysicalProperties::Default)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_util;

    #[test]
    fn round_trip_physical_properties_default() {
        test_util::test_xml_round_trip(&PhysicalProperties::Default);
    }

    #[test]
    fn round_trip_physical_properties_custom() {
        test_util::test_xml_round_trip(&PhysicalProperties::Custom(CustomPhysicalProperties::new(
            0.5, 1.0, 1.5, 2.0, 2.5, None,
        )));
    }

    #[test]
    fn deserialize_physical_properties_default() {
        test_util::test_xml_deserialize(
            r#"
                <PhysicalProperties name="CustomPhysicalProperties">
                    <CustomPhysics>false</CustomPhysics>
                </PhysicalProperties>
            "#,
            &PhysicalProperties::Default,
        );
    }

    #[test]
    fn deserialize_physical_properties_custom() {
        test_util::test_xml_deserialize(
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
            &PhysicalProperties::Custom(CustomPhysicalProperties::new(
                0.5, 1.0, 1.5, 2.0, 2.5, None,
            )),
        );
    }

    #[test]
    fn serialize_physical_properties_default() {
        test_util::test_xml_serialize(
            r#"
                <PhysicalProperties name="foo">
                    <CustomPhysics>false</CustomPhysics>
                </PhysicalProperties>
            "#,
            &PhysicalProperties::Default,
        );
    }

    #[test]
    fn serialize_physical_properties_custom() {
        test_util::test_xml_serialize(
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
            &PhysicalProperties::Custom(CustomPhysicalProperties::new(
                0.5, 1.0, 1.5, 2.0, 2.5, None,
            )),
        );
    }
}
