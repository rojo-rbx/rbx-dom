//! Deserializer that reads a file and creates a debug representation of it.
//! It's intended to be used to snapshot test the binary serializer without
//! suffering from same-inverse-bug problems.

#![allow(missing_docs)]

use std::{collections::HashMap, convert::TryInto, io::Read};

use rbx_dom_weak::types::{
    Axes, CFrame, Color3, EnumValue, Faces, Matrix3, UDim, UDim2, Vector2, Vector3,
};
use serde::Serialize;

use crate::{
    chunk::Chunk, core::RbxReadExt, deserializer::special_case_to_rotation,
    deserializer::FileHeader, types::Type,
};

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
    Faces(Vec<Faces>),
    Axes(Vec<Axes>),
    Color3(Vec<Color3>),
    Vector2(Vec<Vector2>),
    Vector3(Vec<Vector3>),
    CFrame(Vec<CFrame>),
    Enum(Vec<EnumValue>),
    Int64(Vec<i64>),
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
            Type::CFrame => {
                let mut rotations = vec![Matrix3::identity(); prop_count];

                for i in 0..prop_count {
                    let id = reader.read_u8().unwrap();
                    if id == 0 {
                        rotations[i] = Matrix3::new(
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
                        rotations[i] = special_case_to_rotation(id).unwrap();
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

                let values = ints
                    .into_iter()
                    .map(|int| EnumValue::from_u32(int))
                    .collect();

                Some(DecodedValues::Enum(values))
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
            Type::Int64 => {
                let mut values = vec![0; prop_count];

                reader.read_interleaved_i64_array(&mut values).unwrap();

                Some(DecodedValues::Int64(values))
            }
            _ => None,
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

/// Contains data that we haven't decoded for a chunk. Using `unknown_buffer`
/// should generally be a placeholder since it's results are opaque, but stable.
mod unknown_buffer {
    use std::fmt;

    use serde::Serializer;

    pub fn serialize<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
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
