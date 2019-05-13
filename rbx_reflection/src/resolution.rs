use std::fmt;

use rbx_dom_weak::{AmbiguousRbxValue, RbxValue, RbxValueType, UnresolvedRbxValue};

use crate::{
    reflection_database::{get_class_descriptor, get_enum_descriptor},
    reflection_types::RbxPropertyTypeDescriptor,
};

/// An error than can occur when trying to resolve an `UnresolvedRbxValue` to a
/// concrete `RbxValue` using [`try_resolve_value`][try_resolve_value].
///
/// [try_resolve_value]: fn.try_resolve_value.html
#[derive(Debug, Clone)]
pub struct ValueResolveError {
    kind: Box<ValueResolveErrorKind>,
}

impl std::error::Error for ValueResolveError {}

impl fmt::Display for ValueResolveError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(formatter)
    }
}

impl From<ValueResolveErrorKind> for ValueResolveError {
    fn from(kind: ValueResolveErrorKind) -> ValueResolveError {
        ValueResolveError {
            kind: Box::new(kind),
        }
    }
}

#[derive(Debug, Clone)]
enum ValueResolveErrorKind {
    UnknownProperty {
        class_name: String,
        property_name: String,
    },
    UnknownEnumItem {
        enum_name: String,
        item_name: String,
    },
    IncorrectAmbiguousProperty,
}

impl fmt::Display for ValueResolveErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::ValueResolveErrorKind::*;

        match self {
            UnknownProperty { class_name, property_name } =>
                write!(formatter, "Unknown property {}.{}", class_name, property_name),
            UnknownEnumItem { enum_name, item_name } =>
                write!(formatter, "Unknown enum item {}.{}", enum_name, item_name),
            IncorrectAmbiguousProperty =>
                write!(formatter, "Property type couldn't be inferred because the input did not match the property's type"),
        }
    }
}

/// Attempts to transform an `UnresolvedRbxValue` property on the given class into
/// a concrete value using reflection information.
pub fn try_resolve_value(
    class_name: &str,
    property_name: &str,
    value: &UnresolvedRbxValue,
) -> Result<RbxValue, ValueResolveError> {
    match value {
        UnresolvedRbxValue::Concrete(concrete_value) => {
            // For now, we assume that concretely-specified values are of the
            // right type. Extra validation might be more appropriate for
            // another pass.

            Ok(concrete_value.clone())
        }
        UnresolvedRbxValue::Ambiguous(inferable_value) => {
            // If we don't have reflection information for this value, we'll
            // only accept a fully-qualified property.

            let property_type = find_property_type(class_name, property_name).ok_or_else(|| {
                ValueResolveErrorKind::UnknownProperty {
                    class_name: class_name.to_owned(),
                    property_name: property_name.to_owned(),
                }
            })?;

            match inferable_value {
                AmbiguousRbxValue::String(string_value) => {
                    try_resolve_string(class_name, property_name, property_type, string_value)
                }
                AmbiguousRbxValue::Float1(x) => try_resolve_one_float(property_type, *x),
                AmbiguousRbxValue::Float2(x, y) => try_resolve_two_floats(property_type, (*x, *y)),
                AmbiguousRbxValue::Float3(x, y, z) => {
                    try_resolve_three_floats(property_type, (*x, *y, *z))
                }
            }
        }
    }
}

/// A string value can represent:
/// - String
/// - Content
/// - Enum
fn try_resolve_string(
    class_name: &str,
    property_name: &str,
    property_type: &RbxPropertyTypeDescriptor,
    value: &str,
) -> Result<RbxValue, ValueResolveError> {
    match property_type {
        RbxPropertyTypeDescriptor::Data(RbxValueType::String) => Ok(RbxValue::String {
            value: value.to_owned(),
        }),
        RbxPropertyTypeDescriptor::Data(RbxValueType::Content) => Ok(RbxValue::Content {
            value: value.to_owned(),
        }),
        RbxPropertyTypeDescriptor::Enum(enum_name) => {
            let roblox_enum = match get_enum_descriptor(enum_name) {
                Some(roblox_enum) => roblox_enum,
                None => {
                    panic!(
                        "The property {}.{} referred to an enum that does not exist: {}",
                        class_name, property_name, enum_name,
                    );
                }
            };

            let enum_value = roblox_enum.items.get(value).ok_or_else(|| {
                ValueResolveErrorKind::UnknownEnumItem {
                    enum_name: enum_name.to_string(),
                    item_name: value.to_owned(),
                }
            })?;

            Ok(RbxValue::Enum { value: *enum_value })
        }
        _ => Err(ValueResolveErrorKind::IncorrectAmbiguousProperty.into()),
    }
}

/// A single float can be a Float32, Float64, Int32, or Int64.
///
/// Note that because every number is held as a Float64, we might run into
/// precision issues for values outside a 64-bit float's integer precision.
fn try_resolve_one_float(
    property_type: &RbxPropertyTypeDescriptor,
    x: f64,
) -> Result<RbxValue, ValueResolveError> {
    match property_type {
        RbxPropertyTypeDescriptor::Data(RbxValueType::Float32) => {
            Ok(RbxValue::Float32 { value: x as f32 })
        }
        RbxPropertyTypeDescriptor::Data(RbxValueType::Float64) => {
            Ok(RbxValue::Float64 { value: x as f64 })
        }
        RbxPropertyTypeDescriptor::Data(RbxValueType::Int32) => {
            Ok(RbxValue::Int32 { value: x as i32 })
        }
        RbxPropertyTypeDescriptor::Data(RbxValueType::Int64) => {
            Ok(RbxValue::Int64 { value: x as i64 })
        }
        _ => Err(ValueResolveErrorKind::IncorrectAmbiguousProperty.into()),
    }
}

/// Two floats can result in a Vector2 or Vector2int16.
fn try_resolve_two_floats(
    property_type: &RbxPropertyTypeDescriptor,
    (x, y): (f64, f64),
) -> Result<RbxValue, ValueResolveError> {
    match property_type {
        RbxPropertyTypeDescriptor::Data(RbxValueType::Vector2) => Ok(RbxValue::Vector2 {
            value: [x as f32, y as f32],
        }),
        RbxPropertyTypeDescriptor::Data(RbxValueType::Vector2int16) => Ok(RbxValue::Vector2int16 {
            value: [x as i16, y as i16],
        }),
        _ => Err(ValueResolveErrorKind::IncorrectAmbiguousProperty.into()),
    }
}

/// Three floats can turn into a Vector3, a Vector3int16, or a Color3.
///
/// Color3uint8 is another value to handle here, but shouldn't come up in the
/// resolution case since no user-reflected values have that has a type.
fn try_resolve_three_floats(
    property_type: &RbxPropertyTypeDescriptor,
    (x, y, z): (f64, f64, f64),
) -> Result<RbxValue, ValueResolveError> {
    match property_type {
        RbxPropertyTypeDescriptor::Data(RbxValueType::Vector3) => Ok(RbxValue::Vector3 {
            value: [x as f32, y as f32, z as f32],
        }),
        RbxPropertyTypeDescriptor::Data(RbxValueType::Vector3int16) => Ok(RbxValue::Vector3int16 {
            value: [x as i16, y as i16, z as i16],
        }),
        RbxPropertyTypeDescriptor::Data(RbxValueType::Color3) => Ok(RbxValue::Color3 {
            value: [x as f32, y as f32, z as f32],
        }),
        _ => Err(ValueResolveErrorKind::IncorrectAmbiguousProperty.into()),
    }
}

fn find_property_type(
    class_name: &str,
    property_name: &str,
) -> Option<&'static RbxPropertyTypeDescriptor> {
    let mut current_class = class_name;

    loop {
        let class = get_class_descriptor(current_class)?;

        match class.properties.get(property_name) {
            Some(property) => return Some(&property.value_type),
            None => match &class.superclass {
                Some(superclass) => current_class = &superclass,
                None => return None,
            },
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
            Some(&RbxPropertyTypeDescriptor::Data(RbxValueType::String))
        );
        assert_eq!(
            find_property_type("Part", "Name"),
            find_property_type("Instance", "Name")
        );
        assert_eq!(
            find_property_type("Part", "Position"),
            Some(&RbxPropertyTypeDescriptor::Data(RbxValueType::Vector3))
        );
    }

    #[test]
    fn resolve_concrete_known_property() {
        let concrete_value = RbxValue::String {
            value: String::from("Hey! Listen!"),
        };

        let untagged_value = UnresolvedRbxValue::Concrete(concrete_value.clone());

        assert_eq!(
            try_resolve_value("Instance", "Name", &untagged_value).unwrap(),
            concrete_value
        );
    }

    #[test]
    fn resolve_concrete_unknown_class() {
        // Makes sure that a concretely-specified value works even if the class
        // name is unknown.

        let concrete_value = RbxValue::String {
            value: String::from("Hey! Listen!"),
        };

        let untagged_value = UnresolvedRbxValue::Concrete(concrete_value.clone());

        assert_eq!(
            try_resolve_value("Bogus Instance Name", "Blah", &untagged_value).unwrap(),
            concrete_value
        );
    }

    #[test]
    fn resolve_concrete_unknown_property() {
        // Ensures that concretely-specified values resolve correctly even if
        // the property name is unknown.

        let concrete_value = RbxValue::String {
            value: String::from("Hey! Listen!"),
        };

        let untagged_value = UnresolvedRbxValue::Concrete(concrete_value.clone());

        assert_eq!(
            try_resolve_value("Instance", "Bogus Property Name", &untagged_value).unwrap(),
            concrete_value
        );
    }

    #[test]
    fn resolve_inferred_unknown_property() {
        let untagged_value =
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(String::from("HEY!")));

        assert!(try_resolve_value("Nonsense Class", "Value", &untagged_value).is_err());
    }

    #[test]
    fn resolve_inferred_color3() {
        let concrete_value = RbxValue::Color3 {
            // Hopefully these values will behave mercifully as floats.
            value: [1.0, 0.5, 0.0],
        };

        let untagged_value =
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(1.0, 0.5, 0.0));

        assert_eq!(
            try_resolve_value("Color3Value", "Value", &untagged_value).unwrap(),
            concrete_value
        );
    }

    #[test]
    fn resolve_inferred_enum() {
        let concrete_value = RbxValue::Enum {
            value: 2, // Enum.SortOrder.LayoutOrder
        };

        let untagged_value =
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(String::from("LayoutOrder")));

        assert_eq!(
            try_resolve_value("UIListLayout", "SortOrder", &untagged_value).unwrap(),
            concrete_value
        );
    }

    #[test]
    fn resolve_inferred_content() {
        let concrete_value = RbxValue::Content {
            value: String::from("Hello!"),
        };

        let untagged_value =
            UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(String::from("Hello!")));

        assert_eq!(
            try_resolve_value("Decal", "Texture", &untagged_value).unwrap(),
            concrete_value
        );
    }
}
