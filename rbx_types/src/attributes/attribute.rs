use super::error::AttributeError;
use crate::{
    BinaryString, BrickColor, CFrame, Color3, ColorSequence, EnumItem, Font, NumberRange,
    NumberSequence, Rect, UDim, UDim2, Variant, VariantType, Vector2, Vector3,
};

macro_rules! impl_attribute {
    (
        happy{
            $( $happy_id:literal => $happy_variant:ident($happy_ty:ty), )*
        }
        sad{
            $( $sad_id:literal => $sad_variant:ident($sad_ty:ty), )*
        }
    ) => {
        #[derive(Debug, Clone)]
        pub(super) enum AttributeType {
            $(
                $happy_variant,
            )*
            $(
                $sad_variant,
            )*
        }
        impl AttributeType {
            pub(super) fn from_u8(id: u8) -> Option<Self> {
                match id {
                    $(
                        $happy_id => Some(Self::$happy_variant),
                    )*
                    _ => None
                }
            }
            pub(super) fn to_u8(&self) -> u8 {
                match self {
                    $(
                        Self::$happy_variant => $happy_id,
                    )*
                    $(
                        Self::$sad_variant => $sad_id,
                    )*
                }
            }
        }
        #[derive(Debug, Clone)]
        #[non_exhaustive]
        pub enum Attribute {
            $(
                $happy_variant($happy_ty),
            )*
            $(
                $sad_variant($sad_ty),
            )*
        }
        impl Attribute {
            pub(super) fn try_into_variant(self) -> Result<Variant, AttributeError> {
                match self {
                    $(
                        Self::$happy_variant(ty) => Ok(Variant::$happy_variant(ty)),
                    )*
                    $(
                        Self::$sad_variant(_) => Err(AttributeError::UnsupportedVariantType(VariantType::$sad_variant)),
                    )*
                }
            }
        }
    };
}

impl_attribute! {
    happy {
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
        // 34 => ???
        // 35 => ???
        // 36 => ???
    }
    sad {
        37 => Ref(i32),
    }
}
