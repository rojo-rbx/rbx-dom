use crate::VariantType;

macro_rules! type_ids {
    ( $( $ty:ident => $id:literal, )* ) => {
        pub(super) fn from_variant_type(ty: VariantType) -> Option<u8> {
            match ty {
                $( VariantType::$ty => Some($id), )*

                VariantType::String => Some(0x02),
                _ => None,
            }
        }

        pub(super) fn to_variant_type(id: u8) -> Option<VariantType> {
            match id {
                $( $id => Some(VariantType::$ty), )*
                _ => None,
            }
        }
    };
}

type_ids! {
    // ??? => 0x01,
    BinaryString => 0x02,
    Bool => 0x03,
    // ??? => 0x04,
    Float32 => 0x05,
    Float64 => 0x06,
    // ??? => 0x07,
    // ??? => 0x08,
    UDim => 0x09,
    UDim2 => 0x0A,
    // ??? => 0x0B,
    // ??? => 0x0C,
    // ??? => 0x0D,
    BrickColor => 0x0E,
    Color3 => 0x0F,
    Vector2 => 0x10,
    Vector3 => 0x11,
    // ??? => 0x12,
    // ??? => 0x13,
    CFrame => 0x14,
    // ??? => 0x15,
    // ??? => 0x16,
    NumberSequence => 0x17,
    // ??? => 0x18,
    ColorSequence => 0x19,
    // ??? => 0x1A,
    NumberRange => 0x1B,
    Rect => 0x1C,
    Font => 0x21,
}
