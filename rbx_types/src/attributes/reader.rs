// Thanks to Anaminus! https://github.com/RobloxAPI/rbxattr/blob/master/spec.md

use super::*;
use crate::{basic_types::*, brick_color::BrickColor, variant::Variant};
use std::{
    collections::HashMap,
    io::{self, Read},
};

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

fn read_string<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let size = read_u32(&mut reader)? as usize;
    let mut characters = vec![0u8; size];
    reader.read_exact(&mut characters)?;
    Ok(characters)
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
pub fn get_attributes<R: Read>(mut value: R) -> Result<HashMap<Vec<u8>, Variant>, AttributeError> {
    let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;
    let mut attributes = HashMap::with_capacity(size as usize);

    for _ in 0..size {
        let name = read_string(&mut value).map_err(|_| AttributeError::InvalidName)?;

        let attribute_key =
            AttributeType::try_from(read_u8(&mut value).map_err(|_| AttributeError::NoValueType)?)?;

        let value = match attribute_key {
            AttributeType::BrickColor => {
                let color =
                    read_u32(&mut value).map_err(|_| AttributeError::Other("BrickColor"))?;

                Variant::BrickColor(
                    BrickColor::from_number(color as u16)
                        .ok_or(AttributeError::InvalidBrickColor(color))?,
                )
            }

            AttributeType::Bool => {
                Variant::Bool(read_u8(&mut value).map_err(|_| AttributeError::Other("bool"))? != 0)
            }

            AttributeType::Color3 => Variant::Color3(
                read_color3(&mut value).map_err(|_| AttributeError::Other("Color3"))?,
            ),

            AttributeType::ColorSequence => {
                let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is not represented in rbx_types, apparently it's always zero.
                    let _envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint time"))?;

                    let color = read_color3(&mut value)
                        .map_err(|_| AttributeError::Other("ColorSequenceKeypoint color"))?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                Variant::ColorSequence(ColorSequence { keypoints })
            }

            AttributeType::Float32 => {
                Variant::Float32(read_f32(&mut value).map_err(|_| AttributeError::Other("float"))?)
            }

            AttributeType::Float64 => {
                let mut bytes = [0u8; 8];
                value
                    .read_exact(&mut bytes)
                    .map_err(|_| AttributeError::Other("double"))?;
                Variant::Float64(f64::from_le_bytes(bytes))
            }

            AttributeType::NumberRange => Variant::NumberRange(NumberRange::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("NumberRange min"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("NumberRange max"))?,
            )),

            AttributeType::NumberSequence => {
                let size = read_u32(&mut value).map_err(|_| AttributeError::InvalidSize)?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence time"))?;

                    let value = read_f32(&mut value)
                        .map_err(|_| AttributeError::Other("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                Variant::NumberSequence(NumberSequence { keypoints })
            }

            AttributeType::Rect => Variant::Rect(Rect::new(
                read_vector2(&mut value).map_err(|_| AttributeError::Other("Rect min"))?,
                read_vector2(&mut value).map_err(|_| AttributeError::Other("Rect max"))?,
            )),

            AttributeType::BinaryString => Variant::BinaryString(
                read_string(&mut value)
                    .map_err(|_| AttributeError::Other("string"))?
                    .into(),
            ),

            AttributeType::UDim => {
                Variant::UDim(read_udim(&mut value).map_err(|_| AttributeError::Other("UDim"))?)
            }

            AttributeType::UDim2 => Variant::UDim2(UDim2::new(
                read_udim(&mut value).map_err(|_| AttributeError::Other("UDim2 X"))?,
                read_udim(&mut value).map_err(|_| AttributeError::Other("UDim2 Y"))?,
            )),

            AttributeType::Vector2 => Variant::Vector2(Vector2::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector2 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector2 Y"))?,
            )),

            AttributeType::Vector3 => Variant::Vector3(Vector3::new(
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 Y"))?,
                read_f32(&mut value).map_err(|_| AttributeError::Other("Vector3 Z"))?,
            )),
        };

        attributes.insert(name, value);
    }

    Ok(attributes)
}
