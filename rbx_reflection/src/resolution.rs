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

            match find_property_type(class_name, property_name) {
                Some(property_type) => {
                    match inferable_value {
                        InferableRbxValue::String(string_value) => {
                            // String or Enum

                            match property_type {
                                RbxPropertyType::Data(RbxValueType::String) => {
                                    Ok(RbxValue::String {
                                        value: string_value.clone(),
                                    })
                                }
                                RbxPropertyType::Enum(enum_name) => {
                                    // TODO: Look up this enum to pull the u32
                                    // representation from this name.
                                    unimplemented!();
                                }
                                _ => Err(ValueResolveError::IncorrectInferableProperty),
                            }
                        }
                        InferableRbxValue::Float1(x) => {
                            // Float32, Float64, Int32, Int64, or Enum
                            unimplemented!();
                        }
                        InferableRbxValue::Float2(x, y) => {
                            // Vector2 or Vector2int16
                            unimplemented!();
                        }
                        InferableRbxValue::Float3(x, y, z) => {
                            // Vector3, Vector3int16, Color3, or Color3uint8
                            unimplemented!();
                        }
                    }
                }
                None => {
                    let fully_qualified_name = format!("{}.{}", class_name, property_name);
                    Err(ValueResolveError::UnknownPropertyNotInferable(
                        fully_qualified_name,
                    ))
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
