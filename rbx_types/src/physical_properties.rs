#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PhysicalProperties {
    Default,
    Custom(CustomPhysicalProperties),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct CustomPhysicalProperties {
    pub density: f32,
    pub friction: f32,
    pub elasticity: f32,
    pub friction_weight: f32,
    pub elasticity_weight: f32,
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn physical_properties() {
        let custom = serde_json::to_string(&PhysicalProperties::Custom(CustomPhysicalProperties {
            density: 1.0,
            friction: 0.5,
            elasticity: 0.0,
            elasticity_weight: 5.0,
            friction_weight: 6.0,
        }))
        .unwrap();

        assert_eq!(custom, "{\"Density\":1.0,\"Friction\":0.5,\"Elasticity\":0.0,\"FrictionWeight\":6.0,\"ElasticityWeight\":5.0}");

        let _default = serde_json::to_string(&PhysicalProperties::Default).unwrap();

        // TODO: Manually implement Serialize/Deserialize to ensure this is the
        // result. Currently, we get "null", which is unintuitive.

        // assert_eq!(default, "\"Default\"");
    }
}
