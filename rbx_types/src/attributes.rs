// Thanks to Anaminus! https://github.com/RobloxAPI/rbxattr/blob/master/spec.md

use crate::{basic_types::*, brick_color::BrickColor, variant::Variant};
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{self, Read},
};
use thiserror::Error;

macro_rules! create_attribute_type {
    ({
        $(
            $key:ident = $number:tt,
        )+
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum AttributeType {
            $(
                $key = $number,
            )+
        }

        impl TryFrom<u8> for AttributeType {
            type Error = AttributeDeserializeError;

            fn try_from(byte: u8) -> Result<Self, Self::Error> {
                match byte {
                    $(
                        $number => Ok(Self::$key),
                    )+

                    other => Err(AttributeDeserializeError::InvalidValueType(other))
                }
            }
        }
    };
}

create_attribute_type!({
    String = 0x02,
    Bool = 0x03,
    Float = 0x05,
    Double = 0x06,
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
pub enum AttributeDeserializeError {
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

    #[error("no value type was found")]
    NoValueType,

    #[error("couldn't read bytes to deserialize {0}")]
    Other(&'static str),
}

fn read_u8<R: Read>(mut reader: R) -> io::Result<u8> {
    let mut bytes = [0u8; 1];
    reader.read_exact(&mut bytes)?;
    Ok(bytes[0])
}

fn read_i32<R: Read>(mut reader: R) -> io::Result<i32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(i32::from_le_bytes(bytes))
}

fn read_u32<R: Read>(mut reader: R) -> io::Result<u32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_f32<R: Read>(mut reader: R) -> io::Result<f32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(f32::from_le_bytes(bytes))
}

fn read_string<R: Read>(mut reader: R) -> io::Result<String> {
    let size = read_u32(&mut reader)? as usize;
    let mut characters = vec![0u8; size];
    reader.read_exact(&mut characters)?;
    Ok(characters.into_iter().map(|byte| byte as char).collect())
}

fn read_color3<R: Read>(mut reader: R) -> io::Result<Color3> {
    Ok(Color3::new(
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
    ))
}

fn read_udim<R: Read>(mut reader: R) -> io::Result<UDim> {
    Ok(UDim::new(read_f32(&mut reader)?, read_i32(&mut reader)?))
}

fn read_vector2<R: Read>(mut reader: R) -> io::Result<Vector2> {
    Ok(Vector2::new(read_f32(&mut reader)?, read_f32(&mut reader)?))
}

/// Reads through an attribute property (AttributesSerialize) and returns a map of attribute names -> values.
pub fn get_attributes<R: Read>(
    mut value: R,
) -> Result<HashMap<String, Variant>, AttributeDeserializeError> {
    let size = read_u32(&mut value).map_err(|_| AttributeDeserializeError::InvalidSize)?;
    let mut attributes = HashMap::with_capacity(size as usize);

    for _ in 0..size {
        let name = read_string(&mut value).map_err(|_| AttributeDeserializeError::InvalidName)?;

        let attribute_key = AttributeType::try_from(
            read_u8(&mut value).map_err(|_| AttributeDeserializeError::NoValueType)?,
        )?;

        let value = match attribute_key {
            AttributeType::BrickColor => {
                let color = read_u32(&mut value)
                    .map_err(|_| AttributeDeserializeError::Other("BrickColor"))?;

                Variant::BrickColor(
                    BrickColor::from_number(color as u16)
                        .ok_or_else(|| AttributeDeserializeError::InvalidBrickColor(color))?,
                )
            }

            AttributeType::Bool => Variant::Bool(
                read_u8(&mut value).map_err(|_| AttributeDeserializeError::Other("bool"))? != 0,
            ),

            AttributeType::Color3 => Variant::Color3(
                read_color3(&mut value).map_err(|_| AttributeDeserializeError::Other("Color3"))?,
            ),

            AttributeType::ColorSequence => {
                let size =
                    read_u32(&mut value).map_err(|_| AttributeDeserializeError::InvalidSize)?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is not represented in rbx_types, apparently it's always zero.
                    let _envelope = read_f32(&mut value).map_err(|_| {
                        AttributeDeserializeError::Other("ColorSequenceKeypoint envelope")
                    })?;

                    let time = read_f32(&mut value).map_err(|_| {
                        AttributeDeserializeError::Other("ColorSequenceKeypoint time")
                    })?;

                    let color = read_color3(&mut value).map_err(|_| {
                        AttributeDeserializeError::Other("ColorSequenceKeypoint color")
                    })?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                Variant::ColorSequence(ColorSequence { keypoints })
            }

            AttributeType::Double => {
                let mut bytes = [0u8; 8];
                value
                    .read_exact(&mut bytes)
                    .map_err(|_| AttributeDeserializeError::Other("double"))?;
                Variant::Float64(f64::from_le_bytes(bytes))
            }

            AttributeType::Float => Variant::Float32(
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("falot"))?,
            ),

            AttributeType::NumberRange => Variant::NumberRange(NumberRange::new(
                read_f32(&mut value)
                    .map_err(|_| AttributeDeserializeError::Other("NumberRange min"))?,
                read_f32(&mut value)
                    .map_err(|_| AttributeDeserializeError::Other("NumberRange max"))?,
            )),

            AttributeType::NumberSequence => {
                let size =
                    read_u32(&mut value).map_err(|_| AttributeDeserializeError::InvalidSize)?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = read_f32(&mut value)
                        .map_err(|_| AttributeDeserializeError::Other("NumberSequence envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeDeserializeError::Other("NumberSequence time"))?;

                    let value = read_f32(&mut value)
                        .map_err(|_| AttributeDeserializeError::Other("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                Variant::NumberSequence(NumberSequence { keypoints })
            }

            AttributeType::Rect => Variant::Rect(Rect::new(
                read_vector2(&mut value)
                    .map_err(|_| AttributeDeserializeError::Other("Rect min"))?,
                read_vector2(&mut value)
                    .map_err(|_| AttributeDeserializeError::Other("Rect max"))?,
            )),

            AttributeType::String => Variant::String(
                read_string(&mut value).map_err(|_| AttributeDeserializeError::Other("string"))?,
            ),

            AttributeType::UDim => Variant::UDim(
                read_udim(&mut value).map_err(|_| AttributeDeserializeError::Other("UDim"))?,
            ),

            AttributeType::UDim2 => Variant::UDim2(UDim2::new(
                read_udim(&mut value).map_err(|_| AttributeDeserializeError::Other("UDim2 X"))?,
                read_udim(&mut value).map_err(|_| AttributeDeserializeError::Other("UDim2 Y"))?,
            )),

            AttributeType::Vector2 => Variant::Vector2(Vector2::new(
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("Vector2 X"))?,
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("Vector2 Y"))?,
            )),

            AttributeType::Vector3 => Variant::Vector3(Vector3::new(
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("Vector3 X"))?,
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("Vector3 Y"))?,
                read_f32(&mut value).map_err(|_| AttributeDeserializeError::Other("Vector3 Z"))?,
            )),
        };

        attributes.insert(name, value);
    }

    Ok(attributes)
}

#[cfg(all(feature = "serde", test))]
mod tests {
    use super::*;

    // This is taken from rbx-test-files/models/attributes/xml.rbxmx.
    // This is pasted raw as to not create a circular dependency in test (rbx_types -> rbx_xml/rbx_binary -> rbx_types)
    const ATTRIBUTES_BASE64: &'static str = r"
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

        insta::assert_yaml_snapshot!(attributes);
    }
}
