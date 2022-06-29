use crate::VariantType;

macro_rules! type_ids {
    ( $( $ty:ident => $id:literal, )* ) => {
        pub(super) fn from_variant_type(ty: VariantType) -> Option<u8> {
            match ty {
                $( VariantType::$ty => Some($id), )*
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
    // Null => 0x01,
    BinaryString => 0x02,
    Bool => 0x03,
    // Int32 => 0x04,
    Float32 => 0x05,
    Float64 => 0x06,
    // Array => 0x07,
    // Dictionary => 0x08,
    UDim => 0x09,
    UDim2 => 0x0A,
    // Ray => 0x0B,
    // Faces => 0x0C,
    // Axes => 0x0D,
    BrickColor => 0x0E,
    Color3 => 0x0F,
    Vector2 => 0x10,
    Vector3 => 0x11,
    // Vector2int16 => 0x12,
    // Vector3int16 => 0x13,
    // CFrame => 0x14,
    // Enum => 0x15,
    // ??? => 0x16,
    NumberSequence => 0x17,
    // NumberSequenceKeypoint => 0x18,
    ColorSequence => 0x19,
    // ColorSequenceKeypoint => 0x1A,
    NumberRange => 0x1B,
    Rect => 0x1C,
    // PhysicalProperties => 0x1D,
    // Region3 => 0x1E,
    // Region3int16 => 0x1F,
}
