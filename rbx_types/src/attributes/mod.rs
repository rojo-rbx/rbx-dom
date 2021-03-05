use crate::variant::VariantType;
use std::convert::TryFrom;
use thiserror::Error;

mod reader;
mod writer;

pub use reader::get_attributes;
pub use writer::attributes_from_map;

macro_rules! create_attribute_type {
    ({
        $(
            $key:ident = $number:tt,
        )+
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) enum AttributeType {
            $(
                $key = $number,
            )+
        }

        impl TryFrom<VariantType> for AttributeType {
            type Error = AttributeError;

            fn try_from(variant_type: VariantType) -> Result<Self, Self::Error> {
                match variant_type {
                    $(
                        VariantType::$key => Ok(AttributeType::$key),
                    )+

                    _ => Err(AttributeError::InvalidVariantType),
                }
            }
        }

        impl TryFrom<u8> for AttributeType {
            type Error = AttributeError;

            fn try_from(byte: u8) -> Result<Self, Self::Error> {
                match byte {
                    $(
                        $number => Ok(Self::$key),
                    )+

                    other => Err(AttributeError::InvalidValueType(other))
                }
            }
        }
    };
}

create_attribute_type!({
    String = 0x02,
    Bool = 0x03,
    Float32 = 0x05,
    Float64 = 0x06,
    UDim = 0x09,
    UDim2 = 0x0A,
    BrickColor = 0x0E,
    Color3 = 0x0F,
    Vector2 = 0x10,
    Vector3 = 0x11,
    NumberSequence = 0x17,
    ColorSequence = 0x19,
    NumberRange = 0x1B,
    Rect = 0x1C,
});

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AttributeError {
    #[error("invalid value type: {0}")]
    InvalidValueType(u8),

    #[error("invalid brick color: {0}")]
    InvalidBrickColor(u32),

    #[error("invalid entry key")]
    InvalidEntryKey,

    #[error("invalid name")]
    InvalidName,

    #[error("invalid size")]
    InvalidSize,

    #[error("invalid variant type passed")]
    InvalidVariantType,

    #[error("no value type was found")]
    NoValueType,

    #[error("couldn't read bytes to deserialize {0}")]
    Other(&'static str),
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    // This is taken from rbx-test-files/models/attributes/xml.rbxmx.
    // This is pasted raw as to not create a circular dependency in test (rbx_types -> rbx_xml/rbx_binary -> rbx_types)
    const ATTRIBUTES_BASE64: &str = r"
    DwAAAAMAAABOYU4GAAAAAAAA+P8IAAAASW5maW5pdHkGAAAAAAAA8H8NAAAAQ29sb3JTZXF1
    ZW5jZRkDAAAAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAAAAPwAAAAAAAIA/AAAAAAAAAAAA
    AIA/AAAAAAAAAAAAAIA/BwAAAFZlY3RvcjMRAACAPwAAAEAAAEBABwAAAFZlY3RvcjIQAAAg
    QQAASEIOAAAATnVtYmVyU2VxdWVuY2UXAwAAAAAAAAAAAAAAAACAPwAAAAAAAAA/AAAAAAAA
    AAAAAIA/AACAPwYAAABDb2xvcjMPo6IiPwAAAAAAAIA/CgAAAEJyaWNrQ29sb3IO7AMAAAQA
    AABSZWN0HAAAgD8AAABAAABAQAAAgEAFAAAAVURpbTIKAAAAPwoAAAAzMzM/HgAAAAQAAABV
    RGltCQAAAD9kAAAACwAAAE51bWJlclJhbmdlGwAAoEAAACBBBgAAAE51bWJlcgYAAAAAgBzI
    QAcAAABCb29sZWFuAwEGAAAAU3RyaW5nAg0AAABIZWxsbywgd29ybGQh
    ";

    #[test]
    fn test_attributes() {
        let attributes_value =
            base64::decode(&ATTRIBUTES_BASE64.split_whitespace().collect::<String>())
                .expect("bad base64 for attributes");

        let attributes =
            get_attributes(&attributes_value[..]).expect("couldn't deserialize attributes");

        let mut attributes_stable_order = attributes.clone().into_iter().collect::<Vec<_>>();
        attributes_stable_order.sort_by_cached_key(|(key, _)| key.to_owned());
        insta::assert_yaml_snapshot!(attributes_stable_order);

        let new_attribute_bytes = attributes_from_map(attributes_stable_order.to_owned())
            .expect("couldn't get attributes from map");
        let new_attributes = get_attributes(&new_attribute_bytes[..])
            .expect("couldn't deserialize crate produced binary");
        let mut new_attributes_stable_order = new_attributes.into_iter().collect::<Vec<_>>();
        new_attributes_stable_order.sort_by_cached_key(|(key, _)| key.to_owned());

        // They are not checked directly against each other because the data contains NaN.
        assert_eq!(
            format!("{:#?}", attributes_stable_order),
            format!("{:#?}", new_attributes_stable_order)
        );
    }
}
