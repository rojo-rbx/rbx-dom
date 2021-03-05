use super::*;
use crate::{basic_types::*, variant::Variant};

fn write_f32(bytes: &mut Vec<u8>, n: f32) {
    bytes.append(&mut n.to_le_bytes().to_vec());
}

fn write_u32(bytes: &mut Vec<u8>, n: u32) {
    bytes.append(&mut n.to_le_bytes().to_vec());
}

fn write_color3(bytes: &mut Vec<u8>, color: Color3) {
    write_f32(bytes, color.r);
    write_f32(bytes, color.g);
    write_f32(bytes, color.b);
}

fn write_string(bytes: &mut Vec<u8>, string: &str) {
    write_u32(bytes, string.len() as u32);
    for byte in string.bytes() {
        bytes.push(byte);
    }
}

fn write_udim(bytes: &mut Vec<u8>, udim: UDim) {
    write_f32(bytes, udim.scale);
    bytes.append(&mut udim.offset.to_le_bytes().to_vec());
}

fn write_vector2(bytes: &mut Vec<u8>, vector2: Vector2) {
    write_f32(bytes, vector2.x);
    write_f32(bytes, vector2.y);
}

/// Writes the attribute property (AttributesSerialize) from a map of attribute names -> values.
pub fn attributes_from_map<
    K: Into<String>,
    V: Into<Variant>,
    I: Iterator<Item = (K, V)> + ExactSizeIterator,
    M: IntoIterator<IntoIter = I, Item = (K, V)>,
>(
    map: M,
) -> Result<Vec<u8>, AttributeError> {
    let map = map.into_iter();
    let mut bytes = Vec::new();

    bytes.extend((map.len() as u32).to_le_bytes().iter());
    for (name, variant) in map {
        let variant = variant.into();

        write_string(&mut bytes, &name.into());

        let attribute_type = AttributeType::try_from(variant.ty())?;
        bytes.push(attribute_type as u8);

        match (attribute_type, variant) {
            (AttributeType::Bool, Variant::Bool(bool)) => bytes.push(bool as u8),
            (AttributeType::BrickColor, Variant::BrickColor(color)) => {
                write_u32(&mut bytes, color as u32)
            }
            (AttributeType::Color3, Variant::Color3(color)) => write_color3(&mut bytes, color),
            (AttributeType::ColorSequence, Variant::ColorSequence(sequence)) => {
                write_u32(&mut bytes, sequence.keypoints.len() as u32);

                for keypoint in &sequence.keypoints {
                    write_f32(&mut bytes, 0.0); // Envelope
                    write_f32(&mut bytes, keypoint.time);
                    write_color3(&mut bytes, keypoint.color);
                }
            }
            (AttributeType::Float32, Variant::Float32(float)) => {
                write_f32(&mut bytes, float);
            }
            (AttributeType::Float64, Variant::Float64(float)) => {
                bytes.append(&mut float.to_le_bytes().to_vec());
            }
            (AttributeType::NumberRange, Variant::NumberRange(range)) => {
                write_f32(&mut bytes, range.min);
                write_f32(&mut bytes, range.max);
            }
            (AttributeType::NumberSequence, Variant::NumberSequence(sequence)) => {
                write_u32(&mut bytes, sequence.keypoints.len() as u32);

                for keypoint in &sequence.keypoints {
                    write_f32(&mut bytes, keypoint.envelope);
                    write_f32(&mut bytes, keypoint.time);
                    write_f32(&mut bytes, keypoint.value);
                }
            }
            (AttributeType::Rect, Variant::Rect(rect)) => {
                write_vector2(&mut bytes, rect.min);
                write_vector2(&mut bytes, rect.max);
            }
            (AttributeType::String, Variant::String(string)) => {
                write_string(&mut bytes, &string);
            }
            (AttributeType::UDim, Variant::UDim(udim)) => {
                write_udim(&mut bytes, udim);
            }
            (AttributeType::UDim2, Variant::UDim2(udim2)) => {
                write_udim(&mut bytes, udim2.x);
                write_udim(&mut bytes, udim2.y);
            }
            (AttributeType::Vector2, Variant::Vector2(vector2)) => {
                write_vector2(&mut bytes, vector2);
            }
            (AttributeType::Vector3, Variant::Vector3(vector3)) => {
                write_f32(&mut bytes, vector3.x);
                write_f32(&mut bytes, vector3.y);
                write_f32(&mut bytes, vector3.z);
            }

            (other_type, other_variant) => unreachable!(
                "variant {:?} didn't match attribute type {:?}",
                other_variant, other_type
            ),
        }
    }

    Ok(bytes)
}
