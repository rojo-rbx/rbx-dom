use std::io::{self, Write};

use super::attribute::AttributeType;
use super::error::AttributeError;

use crate::{
    basic_types::{Color3, UDim, Vector2},
    variant::Variant,
    Vector3,
};

/// Attribute writer for the binary attributes format.
/// Call write_len and then write_attribute to write each attribute.
/// Does not enforce the number of attributes written.
pub struct AttributeWriter<W, const STATE: bool> {
    writer: W,
}

impl<W: Write> AttributeWriter<W, false> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
    pub fn write_len(self, len: u32) -> Result<AttributeWriter<W, true>, AttributeError> {
        let mut writer = self.writer;
        write_u32(&mut writer, len)?;
        Ok(AttributeWriter { writer })
    }
}
impl<W: Write> AttributeWriter<W, true> {
    pub fn write_attribute(&mut self, name: &str, variant: &Variant) -> Result<(), AttributeError> {
        write_string(&mut self.writer, name)?;

        let attribute_type = AttributeType::from_variant_type(variant.ty())
            .ok_or_else(|| AttributeError::UnsupportedVariantType(variant.ty()))?;
        write_u8(&mut self.writer, attribute_type.to_u8())?;

        match variant {
            Variant::Bool(bool) => self.writer.write_all(&[*bool as u8])?,
            Variant::BrickColor(color) => write_u32(&mut self.writer, *color as u32)?,
            Variant::Color3(color) => write_color3(&mut self.writer, *color)?,
            Variant::ColorSequence(sequence) => {
                write_u32(&mut self.writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut self.writer, 0.0)?; // Envelope
                    write_f32(&mut self.writer, keypoint.time)?;
                    write_color3(&mut self.writer, keypoint.color)?;
                }
            }
            Variant::Int32(int) => write_i32(&mut self.writer, *int)?,
            Variant::Float32(float) => write_f32(&mut self.writer, *float)?,
            Variant::Float64(float) => write_f64(&mut self.writer, *float)?,
            Variant::NumberRange(range) => {
                write_f32(&mut self.writer, range.min)?;
                write_f32(&mut self.writer, range.max)?;
            }
            Variant::NumberSequence(sequence) => {
                write_u32(&mut self.writer, sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    write_f32(&mut self.writer, keypoint.envelope)?;
                    write_f32(&mut self.writer, keypoint.time)?;
                    write_f32(&mut self.writer, keypoint.value)?;
                }
            }
            Variant::Rect(rect) => {
                write_vector2(&mut self.writer, rect.min)?;
                write_vector2(&mut self.writer, rect.max)?
            }
            Variant::BinaryString(string) => write_string(&mut self.writer, string)?,
            Variant::String(string) => write_string(&mut self.writer, string)?,
            Variant::UDim(udim) => write_udim(&mut self.writer, *udim)?,
            Variant::UDim2(udim2) => {
                write_udim(&mut self.writer, udim2.x)?;
                write_udim(&mut self.writer, udim2.y)?
            }
            Variant::Vector2(vector2) => write_vector2(&mut self.writer, *vector2)?,
            Variant::Vector3(vector3) => {
                write_f32(&mut self.writer, vector3.x)?;
                write_f32(&mut self.writer, vector3.y)?;
                write_f32(&mut self.writer, vector3.z)?
            }
            Variant::CFrame(cframe) => {
                write_vector3(&mut self.writer, cframe.position)?;

                let matrix = cframe.orientation;

                if let Some(rotation_id) = matrix.to_basic_rotation_id() {
                    write_u8(&mut self.writer, rotation_id)?;
                } else {
                    write_u8(&mut self.writer, 0x00)?;

                    write_vector3(&mut self.writer, matrix.x)?;
                    write_vector3(&mut self.writer, matrix.y)?;
                    write_vector3(&mut self.writer, matrix.z)?;
                }
            }
            Variant::Font(font) => {
                write_u16(&mut self.writer, font.weight.as_u16())?;
                write_u8(&mut self.writer, font.style.as_u8())?;
                write_string(&mut self.writer, &font.family)?;
                write_string(
                    &mut self.writer,
                    font.cached_face_id.as_deref().unwrap_or_default(),
                )?;
            }
            Variant::EnumItem(enum_item) => {
                write_string(&mut self.writer, &enum_item.ty)?;
                write_u32(&mut self.writer, enum_item.value)?;
            }

            other_variant => unreachable!("variant {:?} was not implemented", other_variant),
        }

        Ok(())
    }
}

fn write_i32<W: Write>(mut writer: W, n: i32) -> io::Result<()> {
    writer.write_all(&n.to_le_bytes()[..])
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
