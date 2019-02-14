use failure::Fail;
use rbx_tree::{InferableRbxValue, RbxValue, RbxValueType, UntaggedRbxValue};

use crate::{
    core::{get_classes, get_enums},
    types::RbxPropertyType,
};

fn find_property_type(class_name: &str, property_name: &str) -> Option<RbxPropertyType> {
    let classes = get_classes();

    let mut current_class = class_name;

    loop {
        let class = classes.get(current_class)?;

        match class.properties.get(property_name) {
            Some(property) => return Some(property.value_type),
            None => {
                let superclass = class.superclass?;
                current_class = superclass;
            }
        }
    }
}

#[derive(Debug, Fail)]
pub enum ValueResolveError {
    #[fail(
        display = "The property {} is unknown and cannot have its type inferred",
        _0
    )]
    UnknownPropertyNotInferable(String),

    #[fail(
        display = "The enum {} does not have a member named {}",
        enum_name, item_name
    )]
    InvalidEnumItem {
        enum_name: String,
        item_name: String,
    },

    // FIXME: Need a more useful error message here.
    #[fail(display = "Property tried to be inferred but the input was wrong")]
    IncorrectInferableProperty,
}

/// Attempts to transform an `UntaggedRbxValue` property on the given class into
/// a concrete value using reflection information.
pub fn try_resolve_value(
    class_name: &str,
    property_name: &str,
    value: &UntaggedRbxValue,
) -> Result<RbxValue, ValueResolveError> {
    match value {
        UntaggedRbxValue::Concrete(concrete_value) => {
            // For now, we assume that concretely-specified values are of the
            // right type. Extra validation might be more appropriate for
            // another pass.

            Ok(concrete_value.clone())
        }
        UntaggedRbxValue::Inferable(inferable_value) => {
            // If we don't have reflection information for this value, we'll
            // only accept a fully-qualified property.

            let property_type = find_property_type(class_name, property_name).ok_or_else(|| {
                let fully_qualified_name = format!("{}.{}", class_name, property_name);

                ValueResolveError::UnknownPropertyNotInferable(fully_qualified_name)
            })?;

            match inferable_value {
                InferableRbxValue::String(string_value) => {
                    // String or Enum

                    match property_type {
                        RbxPropertyType::Data(RbxValueType::String) => Ok(RbxValue::String {
                            value: string_value.clone(),
                        }),
                        RbxPropertyType::Enum(enum_name) => {
                            let enums = get_enums();
                            let roblox_enum = match enums.get(enum_name) {
                                Some(roblox_enum) => roblox_enum,
                                None => {
                                    panic!(
                                        "The property {}.{} referred to an enum that does not exist: {}",
                                        class_name,
                                        property_name,
                                        enum_name,
                                    );
                                }
                            };

                            let enum_value = roblox_enum
                                .items
                                .get(string_value.as_str())
                                .ok_or_else(|| ValueResolveError::InvalidEnumItem {
                                    enum_name: enum_name.to_owned(),
                                    item_name: string_value.to_owned(),
                                })?;

                            Ok(RbxValue::Enum { value: *enum_value })
                        }
                        _ => Err(ValueResolveError::IncorrectInferableProperty),
                    }
                }
                InferableRbxValue::Float1(x) => {
                    // Float32, Float64, Int32, or Int64

                    match property_type {
                        RbxPropertyType::Data(RbxValueType::Float32) => {
                            Ok(RbxValue::Float32 { value: *x as f32 })
                        }
                        RbxPropertyType::Data(RbxValueType::Int32) => {
                            Ok(RbxValue::Int32 { value: *x as i32 })
                        }
                        // TODO: Float64, Int64 when they're added
                        _ => Err(ValueResolveError::IncorrectInferableProperty),
                    }
                }
                InferableRbxValue::Float2(x, y) => {
                    // Vector2 or Vector2int16

                    match property_type {
                        RbxPropertyType::Data(RbxValueType::Vector2) => Ok(RbxValue::Vector2 {
                            value: [*x as f32, *y as f32],
                        }),
                        RbxPropertyType::Data(RbxValueType::Vector2int16) => {
                            Ok(RbxValue::Vector2int16 {
                                value: [*x as i16, *y as i16],
                            })
                        }
                        _ => Err(ValueResolveError::IncorrectInferableProperty),
                    }
                }
                InferableRbxValue::Float3(x, y, z) => {
                    // Vector3, Vector3int16, Color3

                    match property_type {
                        RbxPropertyType::Data(RbxValueType::Vector3) => Ok(RbxValue::Vector3 {
                            value: [*x as f32, *y as f32, *z as f32],
                        }),
                        RbxPropertyType::Data(RbxValueType::Vector3int16) => {
                            Ok(RbxValue::Vector3int16 {
                                value: [*x as i16, *y as i16, *z as i16],
                            })
                        }
                        RbxPropertyType::Data(RbxValueType::Color3) => Ok(RbxValue::Color3 {
                            value: [*x as f32, *y as f32, *z as f32],
                        }),
                        _ => Err(ValueResolveError::IncorrectInferableProperty),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_inherited_property_types() {
        assert_eq!(
            find_property_type("Instance", "Name"),
            Some(RbxPropertyType::Data(RbxValueType::String))
        );
        assert_eq!(
            find_property_type("Part", "Name"),
            find_property_type("Instance", "Name")
        );
        assert_eq!(
            find_property_type("Part", "Position"),
            Some(RbxPropertyType::Data(RbxValueType::Vector3))
        );
    }
}
