use crate::{
    Axes, BrickColor, CFrame, Color3, Color3uint8, ColorSequence, EnumValue, Faces, NumberRange,
    NumberSequence, PhysicalProperties, Ray, Rect, Ref, SharedString, UDim, UDim2, Vector2,
    Vector2int16, Vector3, Vector3int16,
};

/// Represents any Roblox type. Useful for operating generically on Roblox
/// instances.
///
/// ## Stability
///
/// New variants may be added to `Variant` in minor releases. As
/// such, it is marked `#[non_exhaustive]`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Variant {
    Axes(Axes),
    BinaryString(Vec<u8>),
    BrickColor(BrickColor),
    Bool(bool),
    CFrame(CFrame),
    Color3(Color3),
    Color3uint8(Color3uint8),
    ColorSequence(ColorSequence),
    Content(String),
    Enum(EnumValue),
    Faces(Faces),
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    NumberRange(NumberRange),
    NumberSequence(NumberSequence),
    PhysicalProperties(PhysicalProperties),
    Ray(Ray),
    Rect(Rect),
    Ref(Ref),
    SharedString(SharedString),
    String(String),
    UDim(UDim),
    UDim2(UDim2),
    Vector2(Vector2),
    Vector2int16(Vector2int16),
    Vector3(Vector3),
    Vector3int16(Vector3int16),
}

/// Implement conversions from rbx_types types into the equivalent `Variant`
/// value.
macro_rules! trivial_variant_from {
    ( $( $type: ident, )* ) => {
        $(
            impl From<$type> for Variant {
                fn from(value: $type) -> Self {
                    Self::$type(value)
                }
            }
        )*
    };
}

trivial_variant_from! {
    BrickColor,
    CFrame,
    Color3,
    Color3uint8,
    ColorSequence,
    NumberRange,
    NumberSequence,
    PhysicalProperties,
    Ray,
    Rect,
    UDim,
    UDim2,
    Vector2,
    Vector2int16,
    Vector3,
    Vector3int16,
}

/// Implement conversions from common Rust types into their equivalent `Variant`
/// value.
macro_rules! primitive_variant_from {
    ( $( $prim_type: ident => $rbx_type: ident, )* ) => {
        $(
            impl From<$prim_type> for Variant {
                fn from(value: $prim_type) -> Self {
                    Self::$rbx_type(value)
                }
            }
        )*
    };
}

primitive_variant_from! {
    bool => Bool,
    f32 => Float32,
    f64 => Float64,
    i32 => Int32,
    i64 => Int64,
}
