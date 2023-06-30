//! Implements deserialization for `PhysicalProperties`

use std::io::BufRead;

use rbx_dom_weak::types::{CustomPhysicalProperties, PhysicalProperties};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

use super::{bool_deserializer, f32_deserializer};

pub fn physical_properties_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<PhysicalProperties, DecodeError> {
    let custom = reader.read_named_with("CustomPhysics", bool_deserializer)?;

    if custom {
        Ok(PhysicalProperties::Custom(CustomPhysicalProperties {
            density: reader.read_named_with("Density", f32_deserializer)?,
            friction: reader.read_named_with("Friction", f32_deserializer)?,
            elasticity: reader.read_named_with("Elasticity", f32_deserializer)?,
            friction_weight: reader.read_named_with("FrictionWeight", f32_deserializer)?,
            elasticity_weight: reader.read_named_with("ElasticityWeight", f32_deserializer)?,
        }))
    } else {
        Ok(PhysicalProperties::Default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize_test;
    use rbx_dom_weak::types::CustomPhysicalProperties;

    #[test]
    fn default() {
        deserialize_test!(
            physical_properties_deserializer,
            PhysicalProperties::Default,
            "<CustomPhysics>false</CustomPhysics>"
        )
    }

    #[test]
    fn custom() {
        deserialize_test!(
            physical_properties_deserializer,
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
