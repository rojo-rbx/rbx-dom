// Thanks to Anaminus! https://github.com/RobloxAPI/rbxattr/blob/master/spec.md

use std::{
    collections::HashMap,
    io::{self, Read},
};

use crate::{
    BinaryString, BrickColor, Color3, ColorSequence, ColorSequenceKeypoint, NumberRange,
    NumberSequence, NumberSequenceKeypoint, Rect, UDim, UDim2, Variant, VariantType, Vector2,
    Vector3,
};

use super::{type_id, AttributeError};

/// Reads through an attribute property (AttributesSerialize) and returns a map of attribute names -> values.
pub(crate) fn read_attributes<R: Read>(
    mut value: R,
) -> Result<HashMap<String, Variant>, AttributeError> {
    let len = read_u32(&mut value).map_err(|_| AttributeError::InvalidLength)?;
    let mut attributes = HashMap::with_capacity(len as usize);

    for _ in 0..len {
        let key_buf = read_string(&mut value).map_err(|_| AttributeError::NoKey)?;
        let key = String::from_utf8(key_buf).map_err(AttributeError::KeyBadUnicode)?;

        let type_id = read_u8(&mut value).map_err(|_| AttributeError::NoValueType)?;
        let ty = type_id::to_variant_type(type_id)
            .ok_or_else(|| AttributeError::InvalidValueType(type_id))?;

        let value = match ty {
            VariantType::BrickColor => {
                let color =
                    read_u32(&mut value).map_err(|_| AttributeError::ReadType("BrickColor"))?;

                BrickColor::from_number(color as u16)
                    .ok_or_else(|| AttributeError::InvalidBrickColor(color))?
                    .into()
            }

            VariantType::Bool => {
                (read_u8(&mut value).map_err(|_| AttributeError::ReadType("bool"))? != 0).into()
            }

            VariantType::Color3 => read_color3(&mut value)
                .map_err(|_| AttributeError::ReadType("Color3"))?
                .into(),

            VariantType::ColorSequence => {
                let size = read_u32(&mut value)
                    .map_err(|_| AttributeError::ReadType("ColorSequence length"))?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is always zero and can be ignored.
                    let _envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint time"))?;

                    let color = read_color3(&mut value)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint color"))?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                ColorSequence { keypoints }.into()
            }

            VariantType::Float32 => read_f32(&mut value)
                .map_err(|_| AttributeError::ReadType("float32"))?
                .into(),

            VariantType::Float64 => read_f64(&mut value)
                .map_err(|_| AttributeError::ReadType("float64"))?
                .into(),

            VariantType::NumberRange => NumberRange::new(
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("NumberRange min"))?,
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("NumberRange max"))?,
            )
            .into(),

            VariantType::NumberSequence => {
                let size = read_u32(&mut value)
                    .map_err(|_| AttributeError::ReadType("NumberSequence length"))?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = read_f32(&mut value)
                        .map_err(|_| AttributeError::ReadType("NumberSequence envelope"))?;

                    let time = read_f32(&mut value)
                        .map_err(|_| AttributeError::ReadType("NumberSequence time"))?;

                    let value = read_f32(&mut value)
                        .map_err(|_| AttributeError::ReadType("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                NumberSequence { keypoints }.into()
            }

            VariantType::Rect => Rect::new(
                read_vector2(&mut value).map_err(|_| AttributeError::ReadType("Rect min"))?,
                read_vector2(&mut value).map_err(|_| AttributeError::ReadType("Rect max"))?,
            )
            .into(),

            VariantType::BinaryString => {
                let binary_string: BinaryString = read_string(&mut value)
                    .map_err(|_| AttributeError::ReadType("string"))?
                    .into();
                binary_string.into()
            }

            VariantType::UDim => read_udim(&mut value)
                .map_err(|_| AttributeError::ReadType("UDim"))?
                .into(),

            VariantType::UDim2 => UDim2::new(
                read_udim(&mut value).map_err(|_| AttributeError::ReadType("UDim2 X"))?,
                read_udim(&mut value).map_err(|_| AttributeError::ReadType("UDim2 Y"))?,
            )
            .into(),

            VariantType::Vector2 => Vector2::new(
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("Vector2 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("Vector2 Y"))?,
            )
            .into(),

            VariantType::Vector3 => Vector3::new(
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("Vector3 X"))?,
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("Vector3 Y"))?,
                read_f32(&mut value).map_err(|_| AttributeError::ReadType("Vector3 Z"))?,
            )
            .into(),

            other => return Err(AttributeError::UnsupportedVariantType(other)),
        };

        attributes.insert(key, value);
    }

    Ok(attributes)
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

fn read_f64<R: Read>(mut reader: R) -> io::Result<f64> {
    let mut bytes = [0u8; 8];
    reader.read_exact(&mut bytes)?;
    Ok(f64::from_le_bytes(bytes))
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
