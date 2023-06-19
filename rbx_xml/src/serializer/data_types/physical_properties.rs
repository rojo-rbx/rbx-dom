use std::io;

use rbx_dom_weak::types::PhysicalProperties;

use super::{EncodeError, XmlWriter};

pub fn physical_properties_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &PhysicalProperties,
) -> Result<(), EncodeError> {
    match value {
        PhysicalProperties::Custom(properties) => {
            writer.write_rbx("CustomPhysics", true)?;
            writer.write_rbx("Density", properties.density)?;
            writer.write_rbx("Friction", properties.friction)?;
            writer.write_rbx("Elasticity", properties.elasticity)?;
            writer.write_rbx("FrictionWeight", properties.friction_weight)?;
            writer.write_rbx("ElasticityWeight", properties.elasticity_weight)?;
        }
        PhysicalProperties::Default => writer.write_rbx("CustomPhysics", false)?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use rbx_dom_weak::types::CustomPhysicalProperties;

    use super::*;
    use crate::serialize_test;

    #[test]
    fn default() {
        serialize_test!(
            physical_properties_serializer,
            PhysicalProperties::Default,
            "<CustomPhysics>false</CustomPhysics>"
        )
    }

    #[test]
    fn custom() {
        serialize_test!(
            physical_properties_serializer,
            PhysicalProperties::Custom(CustomPhysicalProperties {
                density: 0.5,
                friction: 1.0,
                elasticity: 1.5,
                friction_weight: 2.0,
                elasticity_weight: 2.5,
            }),
            "<CustomPhysics>true</CustomPhysics>
<Density>0.5</Density>
<Friction>1</Friction>
<Elasticity>1.5</Elasticity>
<FrictionWeight>2</FrictionWeight>
<ElasticityWeight>2.5</ElasticityWeight>"
        )
    }
}
