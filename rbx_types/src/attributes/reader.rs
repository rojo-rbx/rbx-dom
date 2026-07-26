use std::io::{self, Read};

use crate::{
    BrickColor, CFrame, Color3, ColorSequence, ColorSequenceKeypoint, EnumItem, Error, Font,
    FontStyle, FontWeight, Matrix3, NumberRange, NumberSequence, NumberSequenceKeypoint, Rect,
    UDim, UDim2, Vector2, Vector3,
};

use super::attribute::{Attribute, AttributeType};
use super::error::AttributeError;

/// Attribute reader for the binary attributes format. STATE is typestate which
/// remembers whether the len has been read. Does not track how many attributes
/// have been read internally, use the provided len value to read the correct
/// number of attributes.
pub struct AttributeReader<R, const STATE: bool> {
    reader: R,
}

impl<R: Read> AttributeReader<R, false> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn read_len(self) -> Result<(AttributeReader<R, true>, u32), Error> {
        let reader = self.reader;
        // This is technicaly an invalid state.  We haven't read the len yet,
        // but say that we have to get access to the complex read_option_u32
        // function.
        let mut scary_reader = AttributeReader { reader };
        let len = match scary_reader.read_option_u32() {
            Ok(Some(len)) => len,
            Ok(None) => 0,
            Err(_) => return Err(AttributeError::InvalidLength.into()),
        };
        Ok((scary_reader, len))
    }
}
impl<R: Read> AttributeReader<R, true> {
    fn read_u8(&mut self) -> Result<u8, AttributeError> {
        let mut bytes = [0u8; 1];
        self.reader.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    fn read_u16(&mut self) -> Result<u16, AttributeError> {
        let mut bytes = [0u8; 2];
        self.reader.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_i32(&mut self) -> Result<i32, AttributeError> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(i32::from_le_bytes(bytes))
    }

    fn read_option_u32(&mut self) -> Result<Option<u32>, AttributeError> {
        let mut bytes = [0u8; 4];
        if self.read_exact_or_none(&mut bytes)? {
            Ok(Some(u32::from_le_bytes(bytes)))
        } else {
            Ok(None)
        }
    }

    fn read_u32(&mut self) -> Result<u32, AttributeError> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_f32(&mut self) -> Result<f32, AttributeError> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }

    fn read_f64(&mut self) -> Result<f64, AttributeError> {
        let mut bytes = [0u8; 8];
        self.reader.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }

    fn read_string(&mut self) -> Result<Vec<u8>, AttributeError> {
        let size = self.read_u32()? as usize;
        let mut characters = vec![0u8; size];
        self.reader.read_exact(&mut characters)?;
        Ok(characters)
    }

    fn read_color3(&mut self) -> Result<Color3, AttributeError> {
        Ok(Color3::new(
            self.read_f32()?,
            self.read_f32()?,
            self.read_f32()?,
        ))
    }

    fn read_udim(&mut self) -> Result<UDim, AttributeError> {
        Ok(UDim::new(self.read_f32()?, self.read_i32()?))
    }

    fn read_vector2(&mut self) -> Result<Vector2, AttributeError> {
        Ok(Vector2::new(self.read_f32()?, self.read_f32()?))
    }

    fn read_vector3(&mut self) -> Result<Vector3, AttributeError> {
        Ok(Vector3::new(
            self.read_f32()?,
            self.read_f32()?,
            self.read_f32()?,
        ))
    }

    /// Implementation taken from read_exact, but allowing an empty buffer by
    /// returning `Ok(false)` instead of an EOF error.
    fn read_exact_or_none(&mut self, mut buf: &mut [u8]) -> Result<bool, AttributeError> {
        let initial_len = buf.len();

        while !buf.is_empty() {
            match self.reader.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(AttributeError::Io(e)),
            }
        }

        if buf.len() == initial_len {
            Ok(false)
        } else if !buf.is_empty() {
            Err(AttributeError::Io(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            )))
        } else {
            Ok(true)
        }
    }

    pub fn read_attribute(&mut self) -> Result<(String, Attribute), Error> {
        let key_buf = self.read_string().map_err(|_| AttributeError::NoKey)?;
        let key = String::from_utf8(key_buf).map_err(AttributeError::KeyBadUnicode)?;

        let type_id = self.read_u8().map_err(|_| AttributeError::NoValueType)?;
        let ty =
            AttributeType::from_u8(type_id).ok_or(AttributeError::InvalidValueType(type_id))?;

        let attribute = match ty {
            AttributeType::BrickColor => {
                let color = self
                    .read_u32()
                    .map_err(|_| AttributeError::ReadType("BrickColor"))?;

                let brick_color = BrickColor::from_number(color as u16)
                    .ok_or(AttributeError::InvalidBrickColor(color))?;

                Attribute::BrickColor(brick_color)
            }

            AttributeType::Bool => Attribute::Bool(
                self.read_u8()
                    .map_err(|_| AttributeError::ReadType("bool"))?
                    != 0,
            ),

            AttributeType::Color3 => Attribute::Color3(
                self.read_color3()
                    .map_err(|_| AttributeError::ReadType("Color3"))?,
            ),
            AttributeType::ColorSequence => {
                let size = self
                    .read_u32()
                    .map_err(|_| AttributeError::ReadType("ColorSequence length"))?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is always zero and can be ignored.
                    let _envelope = self
                        .read_f32()
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint envelope"))?;

                    let time = self
                        .read_f32()
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint time"))?;

                    let color = self
                        .read_color3()
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint color"))?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                Attribute::ColorSequence(ColorSequence { keypoints })
            }

            AttributeType::Int32 => Attribute::Int32(
                self.read_i32()
                    .map_err(|_| AttributeError::ReadType("int32"))?,
            ),
            AttributeType::Float32 => Attribute::Float32(
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("float32"))?,
            ),
            AttributeType::Float64 => Attribute::Float64(
                self.read_f64()
                    .map_err(|_| AttributeError::ReadType("float64"))?,
            ),
            AttributeType::NumberRange => Attribute::NumberRange(NumberRange::new(
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("NumberRange min"))?,
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("NumberRange max"))?,
            )),
            AttributeType::NumberSequence => {
                let size = self
                    .read_u32()
                    .map_err(|_| AttributeError::ReadType("NumberSequence length"))?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = self
                        .read_f32()
                        .map_err(|_| AttributeError::ReadType("NumberSequence envelope"))?;

                    let time = self
                        .read_f32()
                        .map_err(|_| AttributeError::ReadType("NumberSequence time"))?;

                    let value = self
                        .read_f32()
                        .map_err(|_| AttributeError::ReadType("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                Attribute::NumberSequence(NumberSequence { keypoints })
            }

            AttributeType::Rect => Attribute::Rect(Rect::new(
                self.read_vector2()
                    .map_err(|_| AttributeError::ReadType("Rect min"))?,
                self.read_vector2()
                    .map_err(|_| AttributeError::ReadType("Rect max"))?,
            )),
            AttributeType::BinaryString => {
                let data = self
                    .read_string()
                    .map_err(|_| AttributeError::ReadType("string"))?;
                Attribute::BinaryString(data.into())
            }

            AttributeType::UDim => Attribute::UDim(
                self.read_udim()
                    .map_err(|_| AttributeError::ReadType("UDim"))?,
            ),
            AttributeType::UDim2 => Attribute::UDim2(UDim2::new(
                self.read_udim()
                    .map_err(|_| AttributeError::ReadType("UDim2 X"))?,
                self.read_udim()
                    .map_err(|_| AttributeError::ReadType("UDim2 Y"))?,
            )),
            AttributeType::Vector2 => Attribute::Vector2(Vector2::new(
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("Vector2 X"))?,
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("Vector2 Y"))?,
            )),
            AttributeType::Vector3 => Attribute::Vector3(Vector3::new(
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("Vector3 X"))?,
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("Vector3 Y"))?,
                self.read_f32()
                    .map_err(|_| AttributeError::ReadType("Vector3 Z"))?,
            )),
            AttributeType::CFrame => {
                let position = self.read_vector3()?;
                let rotation_id = self.read_u8()?;

                let rotation = if rotation_id == 0 {
                    Matrix3::new(
                        self.read_vector3()?,
                        self.read_vector3()?,
                        self.read_vector3()?,
                    )
                } else {
                    Matrix3::from_basic_rotation_id(rotation_id)?
                };

                Attribute::CFrame(CFrame::new(position, rotation))
            }
            AttributeType::Font => {
                let weight = self.read_u16()?;
                let style = self.read_u8()?;

                let family = {
                    let buf = self.read_string()?;

                    String::from_utf8(buf).map_err(|source| AttributeError::FontBadUnicode {
                        source,
                        field: "family",
                    })?
                };

                let cached_face_id = {
                    let buf = self.read_string()?;

                    if buf.is_empty() {
                        None
                    } else {
                        Some(String::from_utf8(buf).map_err(|source| {
                            AttributeError::FontBadUnicode {
                                source,
                                field: "cached_face_id",
                            }
                        })?)
                    }
                };

                Attribute::Font(Font {
                    family,
                    weight: FontWeight::from_u16(weight).unwrap_or_default(),
                    style: FontStyle::from_u8(style).unwrap_or_default(),
                    cached_face_id,
                })
            }
            AttributeType::EnumItem => {
                let enum_type = self.read_string()?;
                let value = self.read_u32()?;

                Attribute::EnumItem(EnumItem {
                    ty: String::from_utf8(enum_type).map_err(AttributeError::Utf8)?,
                    value,
                })
            }
        };

        Ok((key, attribute))
    }
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn exact_or_none() {
    let mut buf = [0u8; 4];

    macro_rules! reader {
        ($bytes: expr) => {
            // fudge a reader which has STATE = true
            AttributeReader::<&[u8], true> {
                reader: $bytes.as_slice(),
            }
        };
    }

    // Nothing in the buffer
    assert_eq!(reader!([]).read_exact_or_none(&mut buf).unwrap(), false);

    // Something in the buffer: error!
    assert!(reader!([0]).read_exact_or_none(&mut buf).is_err());
    assert!(reader!([0, 1]).read_exact_or_none(&mut buf).is_err());
    assert!(reader!([0, 1, 2]).read_exact_or_none(&mut buf).is_err());

    // Success!
    assert_eq!(
        reader!([0, 1, 2, 3]).read_exact_or_none(&mut buf).unwrap(),
        true
    );

    // Extra stuff, also success!
    assert_eq!(
        reader!([0, 1, 2, 3, 4])
            .read_exact_or_none(&mut buf)
            .unwrap(),
        true
    );
}
