use std::io::{self, Read};

use crate::{
    BrickColor, CFrame, Color3, ColorSequence, ColorSequenceKeypoint, EnumItem, Font, FontStyle,
    FontWeight, Matrix3, NumberRange, NumberSequence, NumberSequenceKeypoint, Rect, UDim, UDim2,
    Vector2, Vector3,
};

use super::attribute::{Attribute, AttributeType};
use super::error::AttributeError;

/// Attribute reader. STATE is typestate which remembers whether the len has
/// been read. Does not track how many attributes have been read internally,
/// use the provided len value to read the correct number of attributes.
pub struct AttributeReader<R, const STATE: bool> {
    reader: R,
}

impl<R: Read> AttributeReader<R, false> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
    pub fn read_len(self) -> Result<(AttributeReader<R, true>, u32), AttributeError> {
        let mut reader = self.reader;
        let len = match read_option_u32(&mut reader) {
            Ok(Some(len)) => len,
            Ok(None) => 0,
            Err(_) => return Err(AttributeError::InvalidLength),
        };
        Ok((AttributeReader { reader }, len))
    }
}
impl<R: Read> AttributeReader<R, true> {
    pub fn read_attribute(&mut self) -> Result<(String, Attribute), AttributeError> {
        let key_buf = read_string(&mut self.reader).map_err(|_| AttributeError::NoKey)?;
        let key = String::from_utf8(key_buf).map_err(AttributeError::KeyBadUnicode)?;

        let type_id = read_u8(&mut self.reader).map_err(|_| AttributeError::NoValueType)?;
        let ty =
            AttributeType::from_u8(type_id).ok_or(AttributeError::InvalidValueType(type_id))?;

        let attribute = match ty {
            AttributeType::BrickColor => {
                let color = read_u32(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("BrickColor"))?;

                let brick_color = BrickColor::from_number(color as u16)
                    .ok_or(AttributeError::InvalidBrickColor(color))?;

                Attribute::BrickColor(brick_color)
            }

            AttributeType::Bool => Attribute::Bool(
                read_u8(&mut self.reader).map_err(|_| AttributeError::ReadType("bool"))? != 0,
            ),

            AttributeType::Color3 => Attribute::Color3(
                read_color3(&mut self.reader).map_err(|_| AttributeError::ReadType("Color3"))?,
            ),
            AttributeType::ColorSequence => {
                let size = read_u32(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("ColorSequence length"))?;
                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    // `envelope` is always zero and can be ignored.
                    let _envelope = read_f32(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint envelope"))?;

                    let time = read_f32(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint time"))?;

                    let color = read_color3(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("ColorSequenceKeypoint color"))?;

                    keypoints.push(ColorSequenceKeypoint::new(time, color));
                }

                Attribute::ColorSequence(ColorSequence { keypoints })
            }

            AttributeType::Int32 => Attribute::Int32(
                read_i32(&mut self.reader).map_err(|_| AttributeError::ReadType("int32"))?,
            ),
            AttributeType::Float32 => Attribute::Float32(
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("float32"))?,
            ),
            AttributeType::Float64 => Attribute::Float64(
                read_f64(&mut self.reader).map_err(|_| AttributeError::ReadType("float64"))?,
            ),
            AttributeType::NumberRange => Attribute::NumberRange(NumberRange::new(
                read_f32(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("NumberRange min"))?,
                read_f32(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("NumberRange max"))?,
            )),
            AttributeType::NumberSequence => {
                let size = read_u32(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("NumberSequence length"))?;

                let mut keypoints = Vec::with_capacity(size as usize);

                for _ in 0..size {
                    let envelope = read_f32(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("NumberSequence envelope"))?;

                    let time = read_f32(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("NumberSequence time"))?;

                    let value = read_f32(&mut self.reader)
                        .map_err(|_| AttributeError::ReadType("NumberSequence value"))?;

                    keypoints.push(NumberSequenceKeypoint::new(time, value, envelope));
                }

                Attribute::NumberSequence(NumberSequence { keypoints })
            }

            AttributeType::Rect => Attribute::Rect(Rect::new(
                read_vector2(&mut self.reader).map_err(|_| AttributeError::ReadType("Rect min"))?,
                read_vector2(&mut self.reader).map_err(|_| AttributeError::ReadType("Rect max"))?,
            )),
            AttributeType::BinaryString => {
                let data = read_string(&mut self.reader)
                    .map_err(|_| AttributeError::ReadType("string"))?;
                Attribute::BinaryString(data.into())
            }

            AttributeType::UDim => Attribute::UDim(
                read_udim(&mut self.reader).map_err(|_| AttributeError::ReadType("UDim"))?,
            ),
            AttributeType::UDim2 => Attribute::UDim2(UDim2::new(
                read_udim(&mut self.reader).map_err(|_| AttributeError::ReadType("UDim2 X"))?,
                read_udim(&mut self.reader).map_err(|_| AttributeError::ReadType("UDim2 Y"))?,
            )),
            AttributeType::Vector2 => Attribute::Vector2(Vector2::new(
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("Vector2 X"))?,
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("Vector2 Y"))?,
            )),
            AttributeType::Vector3 => Attribute::Vector3(Vector3::new(
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("Vector3 X"))?,
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("Vector3 Y"))?,
                read_f32(&mut self.reader).map_err(|_| AttributeError::ReadType("Vector3 Z"))?,
            )),
            AttributeType::CFrame => {
                let position = read_vector3(&mut self.reader)?;
                let rotation_id = read_u8(&mut self.reader)?;

                let rotation = if rotation_id == 0 {
                    Matrix3::new(
                        read_vector3(&mut self.reader)?,
                        read_vector3(&mut self.reader)?,
                        read_vector3(&mut self.reader)?,
                    )
                } else {
                    Matrix3::from_basic_rotation_id(rotation_id)?
                };

                Attribute::CFrame(CFrame::new(position, rotation))
            }
            AttributeType::Font => {
                let weight = read_u16(&mut self.reader)?;
                let style = read_u8(&mut self.reader)?;

                let family = {
                    let buf = read_string(&mut self.reader)?;

                    String::from_utf8(buf).map_err(|source| AttributeError::FontBadUnicode {
                        source,
                        field: "family",
                    })?
                };

                let cached_face_id = {
                    let buf = read_string(&mut self.reader)?;

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
                let enum_type = read_string(&mut self.reader)?;
                let value = read_u32(&mut self.reader)?;

                Attribute::EnumItem(EnumItem {
                    ty: String::from_utf8(enum_type)?,
                    value,
                })
            }
        };

        Ok((key, attribute))
    }
}

fn read_u8<R: Read>(mut reader: R) -> io::Result<u8> {
    let mut bytes = [0u8; 1];
    reader.read_exact(&mut bytes)?;
    Ok(bytes[0])
}

fn read_u16<R: Read>(mut reader: R) -> io::Result<u16> {
    let mut bytes = [0u8; 2];
    reader.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

fn read_i32<R: Read>(mut reader: R) -> io::Result<i32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(i32::from_le_bytes(bytes))
}

fn read_option_u32<R: Read>(reader: R) -> io::Result<Option<u32>> {
    let mut bytes = [0u8; 4];
    if read_exact_or_none(reader, &mut bytes)? {
        Ok(Some(u32::from_le_bytes(bytes)))
    } else {
        Ok(None)
    }
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

fn read_vector3<R: Read>(mut reader: R) -> io::Result<Vector3> {
    Ok(Vector3::new(
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
        read_f32(&mut reader)?,
    ))
}

/// Implementation taken from read_exact, but allowing an empty buffer by
/// returning `Ok(false)` instead of an EOF error.
fn read_exact_or_none<R: Read>(mut reader: R, mut buf: &mut [u8]) -> io::Result<bool> {
    let initial_len = buf.len();

    while !buf.is_empty() {
        match reader.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    if buf.len() == initial_len {
        Ok(false)
    } else if !buf.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn exact_or_none() {
        let mut buf = [0u8; 4];

        // Nothing in the buffer
        assert_eq!(read_exact_or_none(&[][..], &mut buf).unwrap(), false);

        // Something in the buffer: error!
        assert!(read_exact_or_none(&[0][..], &mut buf).is_err());
        assert!(read_exact_or_none(&[0, 1][..], &mut buf).is_err());
        assert!(read_exact_or_none(&[0, 1, 2][..], &mut buf).is_err());

        // Success!
        assert_eq!(
            read_exact_or_none(&[0, 1, 2, 3][..], &mut buf).unwrap(),
            true
        );

        // Extra stuff, also success!
        assert_eq!(
            read_exact_or_none(&[0, 1, 2, 3, 4][..], &mut buf).unwrap(),
            true
        );
    }
}
