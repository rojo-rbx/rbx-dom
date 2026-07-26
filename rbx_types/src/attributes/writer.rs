use std::io::Write;

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
        writer.write_all(&len.to_le_bytes())?;
        Ok(AttributeWriter { writer })
    }
}
impl<W: Write> AttributeWriter<W, true> {
    fn write_i32(&mut self, n: i32) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_f32(&mut self, n: f32) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_f64(&mut self, n: f64) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_u32(&mut self, n: u32) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_u16(&mut self, n: u16) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_u8(&mut self, n: u8) -> Result<(), AttributeError> {
        self.writer.write_all(&n.to_le_bytes())?;
        Ok(())
    }

    fn write_color3(&mut self, color: Color3) -> Result<(), AttributeError> {
        self.write_f32(color.r)?;
        self.write_f32(color.g)?;
        self.write_f32(color.b)?;
        Ok(())
    }

    fn write_string<T: AsRef<[u8]>>(&mut self, string: T) -> Result<(), AttributeError> {
        let bytes = string.as_ref();
        self.write_u32(bytes.len() as u32)?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn write_udim(&mut self, udim: UDim) -> Result<(), AttributeError> {
        self.write_f32(udim.scale)?;
        self.writer.write_all(&udim.offset.to_le_bytes())?;
        Ok(())
    }

    fn write_vector2(&mut self, vector2: Vector2) -> Result<(), AttributeError> {
        self.write_f32(vector2.x)?;
        self.write_f32(vector2.y)?;
        Ok(())
    }

    fn write_vector3(&mut self, vector3: Vector3) -> Result<(), AttributeError> {
        self.write_f32(vector3.x)?;
        self.write_f32(vector3.y)?;
        self.write_f32(vector3.z)?;
        Ok(())
    }

    pub fn write_attribute(&mut self, name: &str, variant: &Variant) -> Result<(), AttributeError> {
        self.write_string(name)?;

        let attribute_type = AttributeType::from_variant_type(variant.ty())
            .ok_or_else(|| AttributeError::UnsupportedVariantType(variant.ty()))?;
        self.write_u8(attribute_type.to_u8())?;

        match variant {
            Variant::Bool(bool) => self.writer.write_all(&[*bool as u8])?,
            Variant::BrickColor(color) => self.write_u32(*color as u32)?,
            Variant::Color3(color) => self.write_color3(*color)?,
            Variant::ColorSequence(sequence) => {
                self.write_u32(sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    self.write_f32(0.0)?; // Envelope
                    self.write_f32(keypoint.time)?;
                    self.write_color3(keypoint.color)?;
                }
            }
            Variant::Int32(int) => self.write_i32(*int)?,
            Variant::Float32(float) => self.write_f32(*float)?,
            Variant::Float64(float) => self.write_f64(*float)?,
            Variant::NumberRange(range) => {
                self.write_f32(range.min)?;
                self.write_f32(range.max)?;
            }
            Variant::NumberSequence(sequence) => {
                self.write_u32(sequence.keypoints.len() as u32)?;

                for keypoint in &sequence.keypoints {
                    self.write_f32(keypoint.envelope)?;
                    self.write_f32(keypoint.time)?;
                    self.write_f32(keypoint.value)?;
                }
            }
            Variant::Rect(rect) => {
                self.write_vector2(rect.min)?;
                self.write_vector2(rect.max)?
            }
            Variant::BinaryString(string) => self.write_string(string)?,
            Variant::String(string) => self.write_string(string)?,
            Variant::UDim(udim) => self.write_udim(*udim)?,
            Variant::UDim2(udim2) => {
                self.write_udim(udim2.x)?;
                self.write_udim(udim2.y)?
            }
            Variant::Vector2(vector2) => self.write_vector2(*vector2)?,
            Variant::Vector3(vector3) => {
                self.write_f32(vector3.x)?;
                self.write_f32(vector3.y)?;
                self.write_f32(vector3.z)?
            }
            Variant::CFrame(cframe) => {
                self.write_vector3(cframe.position)?;

                let matrix = cframe.orientation;

                if let Some(rotation_id) = matrix.to_basic_rotation_id() {
                    self.write_u8(rotation_id)?;
                } else {
                    self.write_u8(0x00)?;

                    self.write_vector3(matrix.x)?;
                    self.write_vector3(matrix.y)?;
                    self.write_vector3(matrix.z)?;
                }
            }
            Variant::Font(font) => {
                self.write_u16(font.weight.as_u16())?;
                self.write_u8(font.style.as_u8())?;
                self.write_string(&font.family)?;
                self.write_string(font.cached_face_id.as_deref().unwrap_or_default())?;
            }
            Variant::EnumItem(enum_item) => {
                self.write_string(&enum_item.ty)?;
                self.write_u32(enum_item.value)?;
            }

            other_variant => unreachable!("variant {:?} was not implemented", other_variant),
        }

        Ok(())
    }
}
