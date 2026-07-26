use std::io::Write;

use super::attribute::AttributeType;
use super::error::AttributeError;

use crate::{
    basic_types::{
        CFrame, Color3, ColorSequence, EnumItem, NumberRange, NumberSequence, Rect, UDim, UDim2,
        Vector2, Vector3,
    },
    brick_color::BrickColor,
    error::Error,
    font::Font,
    variant::Variant,
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
    pub fn write_len(mut self, len: u32) -> Result<AttributeWriter<W, true>, Error> {
        self.write_u32(len)?;
        let writer = self.writer;
        Ok(AttributeWriter { writer })
    }
}

impl<W: Write, const STATE: bool> AttributeWriter<W, STATE> {
    fn write_bool(&mut self, value: bool) -> Result<(), AttributeError> {
        self.writer.write_all(&[value as u8])?;
        Ok(())
    }

    fn write_i32(&mut self, value: i32) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_f32(&mut self, value: f32) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_f64(&mut self, value: f64) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_u32(&mut self, value: u32) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_u16(&mut self, value: u16) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_u8(&mut self, value: u8) -> Result<(), AttributeError> {
        self.writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    fn write_color3(&mut self, color: Color3) -> Result<(), AttributeError> {
        self.write_f32(color.r)?;
        self.write_f32(color.g)?;
        self.write_f32(color.b)?;
        Ok(())
    }

    fn write_string(&mut self, bytes: &[u8]) -> Result<(), AttributeError> {
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
}

// Helper macro to automate 3 lines of code per method.
macro_rules! impl_write_attribute {
    (
        $($variant:ident => fn $method: ident ($self:ident, $value:ident : $ty:ty) $impl: block)*
    ) => {
        impl<W: Write> AttributeWriter<W, true> {
            $(
                pub fn $method(&mut $self, name: &str, $value: $ty) -> Result<(), Error> {
                    $self.write_string(name.as_bytes())?;
                    $self.write_u8(AttributeType::$variant.to_u8())?;
                    $impl
                    Ok(())
                }
            )*
        }
    };
}
impl_write_attribute! {
    Bool => fn write_attribute_bool(self, value: bool) {
        self.write_bool(value)?;
    }
    BrickColor => fn write_attribute_brick_color(self, value: BrickColor) {
        self.write_u32(value as u32)?;
    }
    Color3 => fn write_attribute_color3(self, value: Color3) {
        self.write_color3(value)?;
    }
    ColorSequence => fn write_attribute_color_sequence(self, sequence: &ColorSequence) {
        self.write_u32(sequence.keypoints.len() as u32)?;

        for keypoint in &sequence.keypoints {
            self.write_f32(0.0)?; // Envelope
            self.write_f32(keypoint.time)?;
            self.write_color3(keypoint.color)?;
        }
    }
    Int32 => fn write_attribute_i32(self, value: i32) {
        self.write_i32(value)?;
    }
    Float32 => fn write_attribute_f32(self, value: f32) {
        self.write_f32(value)?;
    }
    Float64 => fn write_attribute_f64(self, value: f64) {
        self.write_f64(value)?;
    }
    NumberRange => fn write_attribute_number_range(self, range: NumberRange) {
        self.write_f32(range.min)?;
        self.write_f32(range.max)?;
    }
    NumberSequence => fn write_attribute_number_sequence(self, sequence: &NumberSequence) {
        self.write_u32(sequence.keypoints.len() as u32)?;

        for keypoint in &sequence.keypoints {
            self.write_f32(keypoint.envelope)?;
            self.write_f32(keypoint.time)?;
            self.write_f32(keypoint.value)?;
        }
    }
    Rect => fn write_attribute_rect(self, rect: Rect) {
        self.write_vector2(rect.min)?;
        self.write_vector2(rect.max)?;
    }
    BinaryString => fn write_attribute_string(self, value: &[u8]) {
        self.write_string(value)?;
    }
    UDim => fn write_attribute_udim(self, udim: UDim) {
        self.write_udim(udim)?;
    }
    UDim2 => fn write_attribute_udim2(self, udim2: UDim2) {
        self.write_udim(udim2.x)?;
        self.write_udim(udim2.y)?;
    }
    Vector2 => fn write_attribute_vector2(self, vector2: Vector2) {
        self.write_vector2(vector2)?;
    }
    Vector3 => fn write_attribute_vector3(self, vector3: Vector3) {
        self.write_f32(vector3.x)?;
        self.write_f32(vector3.y)?;
        self.write_f32(vector3.z)?;
    }
    CFrame => fn write_attribute_cframe(self, cframe: CFrame) {
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
    Font => fn write_attribute_font(self, font: &Font) {
        self.write_u16(font.weight.as_u16())?;
        self.write_u8(font.style.as_u8())?;
        self.write_string(font.family.as_bytes())?;
        self.write_string(font.cached_face_id.as_deref().unwrap_or_default().as_bytes())?;
    }
    EnumItem => fn write_attribute_enum_item(self, enum_item: &EnumItem) {
        self.write_string(enum_item.ty.as_bytes())?;
        self.write_u32(enum_item.value)?;
    }
    Ref => fn write_attribute_ref(self, referent: i32) {
        self.write_i32(referent)?;
    }
}
impl<W: Write> AttributeWriter<W, true> {
    /// Write a generic attribute.  Does not support writing Ref attributes, use write_attribute_ref.
    pub fn write_attribute(&mut self, name: &str, variant: &Variant) -> Result<(), Error> {
        match variant {
            Variant::Bool(value) => self.write_attribute_bool(name, *value),
            Variant::BrickColor(value) => self.write_attribute_brick_color(name, *value),
            Variant::Color3(value) => self.write_attribute_color3(name, *value),
            Variant::ColorSequence(value) => self.write_attribute_color_sequence(name, value),
            Variant::Int32(value) => self.write_attribute_i32(name, *value),
            Variant::Float32(value) => self.write_attribute_f32(name, *value),
            Variant::Float64(value) => self.write_attribute_f64(name, *value),
            Variant::NumberRange(value) => self.write_attribute_number_range(name, *value),
            Variant::NumberSequence(value) => self.write_attribute_number_sequence(name, value),
            Variant::Rect(value) => self.write_attribute_rect(name, *value),
            Variant::BinaryString(value) => self.write_attribute_string(name, value.as_ref()),
            Variant::String(value) => self.write_attribute_string(name, value.as_ref()),
            Variant::UDim(value) => self.write_attribute_udim(name, *value),
            Variant::UDim2(value) => self.write_attribute_udim2(name, *value),
            Variant::Vector2(value) => self.write_attribute_vector2(name, *value),
            Variant::Vector3(value) => self.write_attribute_vector3(name, *value),
            Variant::CFrame(value) => self.write_attribute_cframe(name, *value),
            Variant::Font(value) => self.write_attribute_font(name, value),
            Variant::EnumItem(value) => self.write_attribute_enum_item(name, value),
            // Ref is implicitly unsupported in this function
            other_variant => Err(AttributeError::UnsupportedVariantType(other_variant.ty()).into()),
        }
    }
}
