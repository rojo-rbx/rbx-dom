//! Deserializer that reads a file and creates a debug representation of it.
//! It's intended to be used to snapshot test the binary serializer without
//! suffering from same-inverse-bug problems.

#![allow(missing_docs)]

use std::{collections::HashMap, convert::TryInto, fmt::Write, io::Read};

use rbx_dom_weak::types::{
    Axes, BrickColor, CFrame, Color3, Color3uint8, ColorSequence, ColorSequenceKeypoint,
    CustomPhysicalProperties, Enum, Faces, Font, FontStyle, FontWeight, Matrix3, NumberRange,
    NumberSequence, NumberSequenceKeypoint, PhysicalProperties, Ray, Rect, SharedString, UDim,
    UDim2, Vector2, Vector3, Vector3int16,
};
use serde::{ser::SerializeSeq, Serialize, Serializer};

use crate::{cframe, chunk::Chunk, core::RbxReadExt, deserializer::FileHeader, types::Type};

#[derive(Debug, Serialize)]
pub struct DecodedModel {
    pub num_types: u32,
    pub num_instances: u32,
    pub chunks: Vec<DecodedChunk>,
}

impl DecodedModel {
    pub fn from_reader<R: Read>(mut reader: R) -> Self {
        let header = FileHeader::decode(&mut reader).expect("invalid file header");
        let mut chunks = Vec::new();

        // The number of instance with a given type ID. Used to correctly decode
        // lists of properties from the PROP chunk.
        let mut count_by_type_id = HashMap::new();

        loop {
            let chunk = Chunk::decode(&mut reader).expect("invalid chunk");

            match &chunk.name {
                b"META" => chunks.push(decode_meta_chunk(chunk.data.as_slice())),
                b"SSTR" => chunks.push(decode_sstr_chunk(chunk.data.as_slice())),
                b"INST" => chunks.push(decode_inst_chunk(
                    chunk.data.as_slice(),
                    &mut count_by_type_id,
                )),
                b"PROP" => chunks.push(decode_prop_chunk(
                    chunk.data.as_slice(),
                    &mut count_by_type_id,
                )),
                b"PRNT" => chunks.push(decode_prnt_chunk(chunk.data.as_slice())),
                b"END\0" => {
                    chunks.push(DecodedChunk::End);
                    break;
                }
                _ => {
                    chunks.push(DecodedChunk::Unknown {
                        name: String::from_utf8_lossy(&chunk.name[..]).to_string(),
                        contents: chunk.data,
                    });
                }
            }
        }

        DecodedModel {
            num_types: header.num_types,
            num_instances: header.num_instances,
            chunks,
        }
    }
}

fn decode_meta_chunk<R: Read>(mut reader: R) -> DecodedChunk {
    let num_entries = reader.read_le_u32().unwrap();
    let mut entries = Vec::with_capacity(num_entries as usize);

    for _ in 0..num_entries {
        let key = reader.read_string().unwrap();
        let value = reader.read_string().unwrap();
        entries.push((key, value));
    }

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Meta { entries, remaining }
}

fn decode_sstr_chunk<R: Read>(mut reader: R) -> DecodedChunk {
    let version = reader.read_le_u32().unwrap();
    let num_entries = reader.read_le_u32().unwrap();
    let mut entries = Vec::with_capacity(num_entries as usize);

    for _ in 0..num_entries {
        let mut hash = [0; 16];
        reader.read_exact(&mut hash).unwrap();
        let data = reader.read_binary_string().unwrap();
        entries.push(SharedString::new(data));
    }

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Sstr {
        version,
        entries,
        remaining,
    }
}

fn decode_inst_chunk<R: Read>(
    mut reader: R,
    count_by_type_id: &mut HashMap<u32, usize>,
) -> DecodedChunk {
    let type_id = reader.read_le_u32().unwrap();
    let type_name = reader.read_string().unwrap();
    let object_format = reader.read_u8().unwrap();
    let num_instances = reader.read_le_u32().unwrap();

    count_by_type_id.insert(type_id, num_instances as usize);

    let mut referents = vec![0; num_instances as usize];
    reader.read_referent_array(&mut referents).unwrap();

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Inst {
        type_id,
        type_name,
        object_format,
        referents,
        remaining,
    }
}

fn decode_prop_chunk<R: Read>(
    mut reader: R,
    count_by_type_id: &mut HashMap<u32, usize>,
) -> DecodedChunk {
    let type_id = reader.read_le_u32().unwrap();
    let prop_name = reader.read_string().unwrap();

    let prop_type_value = reader.read_u8().unwrap();
    let (prop_type, values) = match prop_type_value.try_into() {
        Ok(prop_type) => {
            // If this type ID is unknown, we'll default to assuming that type
            // has no members and thus has no values of this property.
            let values = count_by_type_id
                .get(&type_id)
                .map(|&prop_count| DecodedValues::decode(&mut reader, prop_count, prop_type))
                .unwrap_or(None);

            (DecodedPropType::Known(prop_type), values)
        }
        Err(_) => (DecodedPropType::Unknown(prop_type_value), None),
    };

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Prop {
        type_id,
        prop_name,
        prop_type,
        values,
        remaining,
    }
}

fn decode_prnt_chunk<R: Read>(mut reader: R) -> DecodedChunk {
    let version = reader.read_u8().unwrap();
    let num_referents = reader.read_le_u32().unwrap();

    let mut subjects = vec![0; num_referents as usize];
    let mut parents = vec![0; num_referents as usize];

    reader.read_referent_array(&mut subjects).unwrap();
    reader.read_referent_array(&mut parents).unwrap();

    let links = subjects
        .iter()
        .copied()
        .zip(parents.iter().copied())
        .collect();

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Prnt {
        version,
        links,
        remaining,
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DecodedValues {
    String(Vec<RobloxString>),
    Bool(Vec<bool>),
    Int32(Vec<i32>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
    UDim(Vec<UDim>),
    UDim2(Vec<UDim2>),
    Ray(Vec<Ray>),
    Faces(Vec<Faces>),
    Axes(Vec<Axes>),
    BrickColor(Vec<BrickColor>),
    Color3(Vec<Color3>),
    Vector2(Vec<Vector2>),
    Vector3(Vec<Vector3>),
    CFrame(Vec<CFrame>),
    Enum(Vec<Enum>),
    Ref(Vec<i32>),
    Vector3int16(Vec<Vector3int16>),
    NumberSequence(Vec<NumberSequence>),
    ColorSequence(Vec<ColorSequence>),
    NumberRange(Vec<NumberRange>),
    Rect(Vec<Rect>),
    PhysicalProperties(Vec<PhysicalProperties>),
    Color3uint8(Vec<Color3uint8>),
    Int64(Vec<i64>),
    SharedString(Vec<u32>), // For the text deserializer, we only show the index in the shared string array.
    OptionalCFrame(Vec<Option<CFrame>>),
    Font(Vec<Font>),
}

impl DecodedValues {
    fn decode<R: Read>(mut reader: R, prop_count: usize, prop_type: Type) -> Option<Self> {
        match prop_type {
            Type::String => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(reader.read_binary_string().unwrap().into());
                }

                Some(DecodedValues::String(values))
            }
            Type::Bool => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(reader.read_bool().unwrap());
                }

                Some(DecodedValues::Bool(values))
            }
            Type::Int32 => {
                let mut values = vec![0; prop_count];

                reader.read_interleaved_i32_array(&mut values).unwrap();

                Some(DecodedValues::Int32(values))
            }
            Type::Float32 => {
                let mut values = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut values).unwrap();

                Some(DecodedValues::Float32(values))
            }
            Type::Float64 => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(reader.read_le_f64().unwrap());
                }

                Some(DecodedValues::Float64(values))
            }
            Type::UDim => {
                let mut scale = vec![0.0; prop_count];
                let mut offset = vec![0; prop_count];

                reader.read_interleaved_f32_array(&mut scale).unwrap();
                reader.read_interleaved_i32_array(&mut offset).unwrap();

                let values = scale
                    .into_iter()
                    .zip(offset)
                    .map(|(scale, offset)| UDim::new(scale, offset))
                    .collect();

                Some(DecodedValues::UDim(values))
            }
            Type::UDim2 => {
                let mut scale_x = vec![0.0; prop_count];
                let mut scale_y = vec![0.0; prop_count];
                let mut offset_x = vec![0; prop_count];
                let mut offset_y = vec![0; prop_count];

                reader.read_interleaved_f32_array(&mut scale_x).unwrap();
                reader.read_interleaved_f32_array(&mut scale_y).unwrap();
                reader.read_interleaved_i32_array(&mut offset_x).unwrap();
                reader.read_interleaved_i32_array(&mut offset_y).unwrap();

                let x_values = scale_x
                    .into_iter()
                    .zip(offset_x)
                    .map(|(scale, offset)| UDim::new(scale, offset));
                let y_values = scale_y
                    .into_iter()
                    .zip(offset_y)
                    .map(|(scale, offset)| UDim::new(scale, offset));

                let values = x_values
                    .zip(y_values)
                    .map(|(x, y)| UDim2::new(x, y))
                    .collect();

                Some(DecodedValues::UDim2(values))
            }
            Type::Font => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    let family = reader.read_string().unwrap();
                    let weight =
                        FontWeight::from_u16(reader.read_le_u16().unwrap()).unwrap_or_default();
                    let style = FontStyle::from_u8(reader.read_u8().unwrap()).unwrap_or_default();
                    let cached_face_id = reader.read_string().unwrap();

                    let cached_face_id = if cached_face_id.is_empty() {
                        None
                    } else {
                        Some(cached_face_id)
                    };

                    values.push(Font {
                        family,
                        weight,
                        style,
                        cached_face_id,
                    })
                }

                Some(DecodedValues::Font(values))
            }
            Type::Ray => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    let origin_x = reader.read_le_f32().unwrap();
                    let origin_y = reader.read_le_f32().unwrap();
                    let origin_z = reader.read_le_f32().unwrap();
                    let direction_x = reader.read_le_f32().unwrap();
                    let direction_y = reader.read_le_f32().unwrap();
                    let direction_z = reader.read_le_f32().unwrap();

                    values.push(Ray::new(
                        Vector3::new(origin_x, origin_y, origin_z),
                        Vector3::new(direction_x, direction_y, direction_z),
                    ))
                }

                Some(DecodedValues::Ray(values))
            }
            Type::Faces => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(Faces::from_bits(reader.read_u8().unwrap())?)
                }

                Some(DecodedValues::Faces(values))
            }
            Type::Axes => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(Axes::from_bits(reader.read_u8().unwrap())?)
                }

                Some(DecodedValues::Axes(values))
            }
            Type::BrickColor => {
                let mut values = vec![0; prop_count];
                reader.read_interleaved_u32_array(&mut values).unwrap();

                let values = values
                    .into_iter()
                    .map(|value| BrickColor::from_number(value.try_into().unwrap()).unwrap())
                    .collect();

                Some(DecodedValues::BrickColor(values))
            }
            Type::CFrame => {
                let mut rotations = vec![Matrix3::identity(); prop_count];

                for rotation in rotations.iter_mut() {
                    let id = reader.read_u8().unwrap();
                    if id == 0 {
                        *rotation = Matrix3::new(
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                        );
                    } else {
                        *rotation = cframe::from_basic_rotation_id(id).unwrap();
                    }
                }

                let mut x = vec![0.0; prop_count];
                let mut y = vec![0.0; prop_count];
                let mut z = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut x).unwrap();
                reader.read_interleaved_f32_array(&mut y).unwrap();
                reader.read_interleaved_f32_array(&mut z).unwrap();

                let values = x
                    .into_iter()
                    .zip(y)
                    .zip(z)
                    .zip(rotations)
                    .map(|(((x, y), z), rotation)| CFrame::new(Vector3::new(x, y, z), rotation))
                    .collect();

                Some(DecodedValues::CFrame(values))
            }
            Type::Enum => {
                let mut ints = vec![0; prop_count];
                reader.read_interleaved_u32_array(&mut ints).unwrap();

                let values = ints.into_iter().map(Enum::from_u32).collect();

                Some(DecodedValues::Enum(values))
            }
            Type::Ref => {
                let mut refs = vec![0; prop_count];
                reader.read_referent_array(&mut refs).unwrap();

                Some(DecodedValues::Ref(refs))
            }
            Type::Color3 => {
                let mut r = vec![0.0; prop_count];
                let mut g = vec![0.0; prop_count];
                let mut b = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut r).unwrap();
                reader.read_interleaved_f32_array(&mut g).unwrap();
                reader.read_interleaved_f32_array(&mut b).unwrap();

                let values = r
                    .into_iter()
                    .zip(g)
                    .zip(b)
                    .map(|((r, g), b)| Color3::new(r, g, b))
                    .collect();

                Some(DecodedValues::Color3(values))
            }
            Type::Vector2 => {
                let mut x = vec![0.0; prop_count];
                let mut y = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut x).unwrap();
                reader.read_interleaved_f32_array(&mut y).unwrap();

                let values = x
                    .into_iter()
                    .zip(y)
                    .map(|(x, y)| Vector2::new(x, y))
                    .collect();

                Some(DecodedValues::Vector2(values))
            }
            Type::Vector3 => {
                let mut x = vec![0.0; prop_count];
                let mut y = vec![0.0; prop_count];
                let mut z = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut x).unwrap();
                reader.read_interleaved_f32_array(&mut y).unwrap();
                reader.read_interleaved_f32_array(&mut z).unwrap();

                let values = x
                    .into_iter()
                    .zip(y)
                    .zip(z)
                    .map(|((x, y), z)| Vector3::new(x, y, z))
                    .collect();

                Some(DecodedValues::Vector3(values))
            }
            Type::ColorSequence => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    let keypoint_count = reader.read_le_u32().unwrap() as usize;
                    let mut keypoints = Vec::with_capacity(keypoint_count);

                    for _ in 0..keypoint_count {
                        keypoints.push(ColorSequenceKeypoint::new(
                            reader.read_le_f32().unwrap(),
                            Color3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                        ));

                        // envelope is serialized but doesn't do anything; don't do anything with it
                        reader.read_le_f32().unwrap();
                    }

                    values.push(ColorSequence { keypoints })
                }

                Some(DecodedValues::ColorSequence(values))
            }
            Type::Vector3int16 => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(Vector3int16::new(
                        reader.read_le_i16().unwrap(),
                        reader.read_le_i16().unwrap(),
                        reader.read_le_i16().unwrap(),
                    ));
                }

                Some(DecodedValues::Vector3int16(values))
            }
            Type::NumberRange => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    values.push(NumberRange::new(
                        reader.read_le_f32().unwrap(),
                        reader.read_le_f32().unwrap(),
                    ));
                }

                Some(DecodedValues::NumberRange(values))
            }
            Type::NumberSequence => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    let keypoint_count = reader.read_le_u32().unwrap();
                    let mut keypoints = Vec::with_capacity(keypoint_count as usize);

                    for _ in 0..keypoint_count {
                        keypoints.push(NumberSequenceKeypoint::new(
                            reader.read_le_f32().unwrap(),
                            reader.read_le_f32().unwrap(),
                            reader.read_le_f32().unwrap(),
                        ))
                    }

                    values.push(NumberSequence { keypoints })
                }

                Some(DecodedValues::NumberSequence(values))
            }
            Type::Rect => {
                let mut x_min = vec![0.0; prop_count];
                let mut y_min = vec![0.0; prop_count];
                let mut x_max = vec![0.0; prop_count];
                let mut y_max = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut x_min).unwrap();
                reader.read_interleaved_f32_array(&mut y_min).unwrap();
                reader.read_interleaved_f32_array(&mut x_max).unwrap();
                reader.read_interleaved_f32_array(&mut y_max).unwrap();

                let values = x_min
                    .into_iter()
                    .zip(y_min)
                    .zip(x_max)
                    .zip(y_max)
                    .map(|(((x_min, y_min), x_max), y_max)| {
                        Rect::new(Vector2::new(x_min, y_min), Vector2::new(x_max, y_max))
                    })
                    .collect();

                Some(DecodedValues::Rect(values))
            }
            Type::PhysicalProperties => {
                let mut values = Vec::with_capacity(prop_count);

                for _ in 0..prop_count {
                    if reader.read_u8().unwrap() == 1 {
                        values.push(PhysicalProperties::Custom(CustomPhysicalProperties {
                            density: reader.read_le_f32().unwrap(),
                            friction: reader.read_le_f32().unwrap(),
                            elasticity: reader.read_le_f32().unwrap(),
                            friction_weight: reader.read_le_f32().unwrap(),
                            elasticity_weight: reader.read_le_f32().unwrap(),
                        }))
                    } else {
                        values.push(PhysicalProperties::Default)
                    }
                }

                Some(DecodedValues::PhysicalProperties(values))
            }
            Type::Color3uint8 => {
                let mut r = vec![0; prop_count];
                let mut g = vec![0; prop_count];
                let mut b = vec![0; prop_count];

                reader.read_exact(r.as_mut_slice()).unwrap();
                reader.read_exact(g.as_mut_slice()).unwrap();
                reader.read_exact(b.as_mut_slice()).unwrap();

                let values = r
                    .into_iter()
                    .zip(g)
                    .zip(b)
                    .map(|((r, g), b)| Color3uint8::new(r, g, b))
                    .collect();

                Some(DecodedValues::Color3uint8(values))
            }
            Type::Int64 => {
                let mut values = vec![0; prop_count];

                reader.read_interleaved_i64_array(&mut values).unwrap();

                Some(DecodedValues::Int64(values))
            }
            Type::SharedString => {
                let mut values = vec![0; prop_count];

                reader.read_interleaved_u32_array(&mut values).unwrap();

                Some(DecodedValues::SharedString(values))
            }
            Type::OptionalCFrame => {
                let mut rotations = vec![Matrix3::identity(); prop_count];

                reader.read_u8().unwrap();

                for rotation in rotations.iter_mut() {
                    let id = reader.read_u8().unwrap();
                    if id == 0 {
                        *rotation = Matrix3::new(
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                            Vector3::new(
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                                reader.read_le_f32().unwrap(),
                            ),
                        );
                    } else {
                        *rotation = cframe::from_basic_rotation_id(id).unwrap();
                    }
                }

                let mut x = vec![0.0; prop_count];
                let mut y = vec![0.0; prop_count];
                let mut z = vec![0.0; prop_count];

                reader.read_interleaved_f32_array(&mut x).unwrap();
                reader.read_interleaved_f32_array(&mut y).unwrap();
                reader.read_interleaved_f32_array(&mut z).unwrap();

                reader.read_u8().unwrap();

                let values = x
                    .into_iter()
                    .zip(y)
                    .zip(z)
                    .zip(rotations)
                    .map(|(((x, y), z), rotation)| {
                        if reader.read_u8().unwrap() == 0 {
                            None
                        } else {
                            Some(CFrame::new(Vector3::new(x, y, z), rotation))
                        }
                    })
                    .collect();

                Some(DecodedValues::OptionalCFrame(values))
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DecodedPropType {
    Known(Type),
    Unknown(u8),
}

/// Holds a string with the same semantics as Roblox does. It can be UTF-8, but
/// might not be.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RobloxString {
    String(String),
    BinaryString(#[serde(with = "unknown_buffer")] Vec<u8>),
}

impl From<Vec<u8>> for RobloxString {
    fn from(value: Vec<u8>) -> Self {
        match String::from_utf8(value) {
            Ok(string) => RobloxString::String(string),
            Err(err) => RobloxString::BinaryString(err.into_bytes()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum DecodedChunk {
    Meta {
        entries: Vec<(String, String)>,

        #[serde(with = "unknown_buffer", skip_serializing_if = "Vec::is_empty")]
        remaining: Vec<u8>,
    },

    Sstr {
        version: u32,
        #[serde(serialize_with = "shared_string_serializer")]
        entries: Vec<SharedString>,

        #[serde(with = "unknown_buffer", skip_serializing_if = "Vec::is_empty")]
        remaining: Vec<u8>,
    },

    Inst {
        type_id: u32,
        type_name: String,
        object_format: u8,
        referents: Vec<i32>,

        #[serde(with = "unknown_buffer", skip_serializing_if = "Vec::is_empty")]
        remaining: Vec<u8>,
    },

    Prop {
        type_id: u32,
        prop_name: String,
        prop_type: DecodedPropType,

        #[serde(skip_serializing_if = "Option::is_none")]
        values: Option<DecodedValues>,

        #[serde(with = "unknown_buffer", skip_serializing_if = "Vec::is_empty")]
        remaining: Vec<u8>,
    },

    Prnt {
        version: u8,
        links: Vec<(i32, i32)>,

        #[serde(with = "unknown_buffer", skip_serializing_if = "Vec::is_empty")]
        remaining: Vec<u8>,
    },

    End,

    Unknown {
        name: String,

        #[serde(with = "unknown_buffer")]
        contents: Vec<u8>,
    },
}

#[derive(Serialize)]
struct SerializedSharedString<'a> {
    len: usize,
    hash: &'a str,
}

fn shared_string_serializer<S>(values: &[SharedString], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hash = String::with_capacity(64);

    let mut state = serializer.serialize_seq(Some(values.len()))?;
    for value in values {
        for byte in value.hash().as_bytes() {
            write!(hash, "{:02x}", byte).unwrap();
        }
        state.serialize_element(&SerializedSharedString {
            len: value.data().len(),
            hash: hash.as_str(),
        })?;

        hash.clear()
    }

    state.end()
}

/// Contains data that we haven't decoded for a chunk. Using `unknown_buffer`
/// should generally be a placeholder since it's results are opaque, but stable.
mod unknown_buffer {
    use std::fmt;

    use serde::Serializer;

    pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&SliceBytes(value))
    }

    struct SliceBytes<'a>(&'a [u8]);

    impl fmt::Display for SliceBytes<'_> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            for (index, byte) in self.0.iter().enumerate() {
                if index < self.0.len() - 1 {
                    write!(formatter, "{:02x} ", byte)?;
                } else {
                    write!(formatter, "{:02x}", byte)?;
                }
            }

            Ok(())
        }
    }
}
