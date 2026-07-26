use crate::{
    BinaryString, BrickColor, CFrame, Color3, ColorSequence, EnumItem, Font, NumberRange,
    NumberSequence, Rect, UDim, UDim2, Variant, VariantType, Vector2, Vector3,
};

macro_rules! impl_attribute {
    ( $( $id:literal => $variant:ident($ty:ty), )* ) => {
        #[derive(Debug, Clone)]
        pub(super) enum AttributeType {
            $(
                $variant,
            )*
        }
        impl AttributeType {
            pub(super) fn from_u8(id: u8) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None
                }
            }
            pub(super) fn from_variant_type(ty: VariantType) -> Option<Self> {
                match ty {
                    $(
                        VariantType::$variant => Some(Self::$variant),
                    )*
                    VariantType::String => Some(Self::BinaryString),
                    _ => None
                }
            }
            pub(super) fn to_u8(&self) -> u8 {
                match self {
                    $(
                        Self::$variant => $id,
                    )*
                }
            }
        }
        #[derive(Debug, Clone)]
        pub enum Attribute {
            $(
                $variant($ty),
            )*
        }
        impl Attribute {
            pub(super) fn into_variant(self) -> Variant {
                match self {
                    $(
                        Self::$variant(ty) => Variant::$variant(ty),
                    )*
                }
            }
        }
    };
}

impl_attribute! {
    // 1 => Null,
    2 => BinaryString(BinaryString),
    3 => Bool(bool),
    4 => Int32(i32),
    5 => Float32(f32),
    6 => Float64(f64),
    // 7 => Array,
    // 8 => Dictionary,
    9 => UDim(UDim),
    10 => UDim2(UDim2),
    // 11 => Ray,
    // 12 => Faces,
    // 13 => Axes
    14 => BrickColor(BrickColor),
    15 => Color3(Color3),
    16 => Vector2(Vector2),
    17 => Vector3(Vector3),
    // 18 => Vector2int16,
    // 19 => Vector3int16,
    20 => CFrame(CFrame),
    21 => EnumItem(EnumItem),
    23 => NumberSequence(NumberSequence),
    // 24 => NumberSequenceKeypoint,
    25 => ColorSequence(ColorSequence),
    // 26 => ColorSequenceKeypoint,
    27 => NumberRange(NumberRange),
    28 => Rect(Rect),
    // 29 => PhysicalProperties
    // 31 => Region3,
    // 32 => Region3int16,
    33 => Font(Font),
}
