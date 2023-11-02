use std::{
    collections::BTreeMap,
    io::{self, Write},
};

use super::{type_id, AttributeError};

use crate::{
    basic_types::{Color3, UDim, Vector2},
    variant::Variant,
    Vector3,
};

/// Writes the attribute property (AttributesSerialize) from a map of attribute names -> values.
pub(crate) fn write_attributes<W: Write>(
    map: &BTreeMap<String, Variant>,
    mut writer: W,
) -> Result<(), AttributeError> {
    if map.is_empty() {
        return Ok(());
    }

    writer.write_all(&(map.len() as u32).to_le_bytes())?;

    for (name, variant) in map {
        write_string(&mut writer, name)?;

        let type_id = type_id::from_variant_type(variant.ty())
            .ok_or_else(|| AttributeError::UnsupportedVariantType(variant.ty()))?;
        writer.write_all(&[type_id])?;

        match variant {
            Variant::Bool(bool) => writer.write_all(&[*bool as u8])?,
            Variant::BrickColor(color) => write_u32(&mut writer, *color as u32)?,
            Variant::Color3(color) => write_color3(&mut writer, *color)?,
            Variant::ColorSequence(sequence) => {
                write_u32(&mut writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut writer, 0.0)?; // Envelope
                    write_f32(&mut writer, keypoint.time)?;
                    write_color3(&mut writer, keypoint.color)?;
                }
            }
            Variant::Float32(float) => write_f32(&mut writer, *float)?,
            Variant::Float64(float) => write_f64(&mut writer, *float)?,
            Variant::NumberRange(range) => {
                write_f32(&mut writer, range.min)?;
                write_f32(&mut writer, range.max)?;
            }
            Variant::NumberSequence(sequence) => {
                write_u32(&mut writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut writer, keypoint.envelope)?;
                    write_f32(&mut writer, keypoint.time)?;
                    write_f32(&mut writer, keypoint.value)?;
                }
            }
            Variant::Rect(rect) => {
                write_vector2(&mut writer, rect.min)?;
                write_vector2(&mut writer, rect.max)?
            }
            Variant::BinaryString(string) => write_string(&mut writer, string)?,
            Variant::String(string) => write_string(&mut writer, string)?,
            Variant::UDim(udim) => write_udim(&mut writer, *udim)?,
            Variant::UDim2(udim2) => {
                write_udim(&mut writer, udim2.x)?;
                write_udim(&mut writer, udim2.y)?
            }
            Variant::Vector2(vector2) => write_vector2(&mut writer, *vector2)?,
            Variant::Vector3(vector3) => {
                write_f32(&mut writer, vector3.x)?;
                write_f32(&mut writer, vector3.y)?;
                write_f32(&mut writer, vector3.z)?
            }
            Variant::CFrame(cframe) => {
                write_vector3(&mut writer, cframe.position)?;

                let matrix = cframe.orientation;

                if let Some(rotation_id) = matrix.to_basic_rotation_id() {
                    write_u8(&mut writer, rotation_id)?;
                } else {
                    write_u8(&mut writer, 0x00)?;

                    write_vector3(&mut writer, matrix.x)?;
                    write_vector3(&mut writer, matrix.y)?;
                    write_vector3(&mut writer, matrix.z)?;
                }
            }
            Variant::Font(font) => {
                write_u16(&mut writer, font.weight.as_u16())?;
                write_u8(&mut writer, font.style.as_u8())?;
                write_string(&mut writer, &font.family)?;
                write_string(
                    &mut writer,
                    &font.cached_face_id.clone().unwrap_or_default(),
                )?;
            }

            other_variant => unreachable!("variant {:?} was not implemented", other_variant),
        }
    }

    Ok(())
}

fn write_f32<W: Write>(mut writer: W, n: f32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_f64<W: Write>(mut writer: W, n: f64) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_u32<W: Write>(mut writer: W, n: u32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_u16<W: Write>(mut writer: W, n: u16) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_u8<W: Write>(mut writer: W, n: u8) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
}

fn write_color3<W: Write>(mut writer: W, color: Color3) -> io::Result<()> {
    write_f32(&mut writer, color.r)?;
    write_f32(&mut writer, color.g)?;
    write_f32(&mut writer, color.b)
}

fn write_string<T: AsRef<[u8]>, W: Write>(mut writer: W, string: T) -> io::Result<()> {
    let bytes = string.as_ref();
    write_u32(&mut writer, bytes.len() as u32)?;
    writer.write_all(bytes)
}

fn write_udim<W: Write>(mut writer: W, udim: UDim) -> io::Result<()> {
    write_f32(&mut writer, udim.scale)?;
    writer.write_all(&udim.offset.to_le_bytes()[..])
}

fn write_vector2<W: Write>(mut writer: W, vector2: Vector2) -> io::Result<()> {
    write_f32(&mut writer, vector2.x)?;
    write_f32(&mut writer, vector2.y)
}

fn write_vector3<W: Write>(mut writer: W, vector3: Vector3) -> io::Result<()> {
    write_f32(&mut writer, vector3.x)?;
    write_f32(&mut writer, vector3.y)?;
    write_f32(&mut writer, vector3.z)
}
