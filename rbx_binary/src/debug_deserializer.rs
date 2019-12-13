//! Deserializer that reads a file and creates a debug representation of it.
//! It's intended to be used to snapshot test the binary serializer without
//! suffering from same-inverse-bug problems.

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{chunk::Chunk, core::RbxReadExt, deserializer::FileHeader};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedModel {
    pub num_types: u32,
    pub num_instances: u32,
    pub chunks: Vec<DecodedChunk>,
}

impl DecodedModel {
    pub fn from_reader<R: Read>(mut reader: R) -> Self {
        let header = FileHeader::decode(&mut reader).expect("invalid file header");
        let mut chunks = Vec::new();

        loop {
            let chunk = Chunk::decode(&mut reader).expect("invalid chunk");

            match &chunk.name {
                b"META" => chunks.push(decode_meta_chunk(chunk.data.as_slice())),
                b"INST" => chunks.push(decode_inst_chunk(&chunk.data)),
                b"PROP" => chunks.push(decode_prop_chunk(&chunk.data)),
                b"PRNT" => chunks.push(decode_prnt_chunk(&chunk.data)),
                b"END\0" => {
                    chunks.push(DecodedChunk::End);
                    break;
                }
                _ => {
                    chunks.push(DecodedChunk::Unknown {
                        name: String::from_utf8_lossy(&chunk.name[..]).to_string(),
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
    let num_entries = reader.read_u32::<LittleEndian>().unwrap();
    let mut entries = Vec::with_capacity(num_entries as usize);

    for _ in 0..num_entries {
        let key = reader.read_string().unwrap();
        let value = reader.read_string().unwrap();
        entries.push((key, value));
    }

    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    DecodedChunk::Meta {
        entries,
        remaining: remaining.into(),
    }
}

fn decode_inst_chunk(data: &[u8]) -> DecodedChunk {
    DecodedChunk::Inst {
        remaining: data.into(),
    }
}

fn decode_prop_chunk(data: &[u8]) -> DecodedChunk {
    DecodedChunk::Prop {
        remaining: data.into(),
    }
}

fn decode_prnt_chunk(data: &[u8]) -> DecodedChunk {
    DecodedChunk::Prnt {
        remaining: data.into(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DecodedChunk {
    Meta {
        entries: Vec<(String, String)>,
        remaining: UnknownBuffer,
    },

    Inst {
        remaining: UnknownBuffer,
    },

    Prop {
        remaining: UnknownBuffer,
    },

    Prnt {
        remaining: UnknownBuffer,
    },

    End,

    Unknown {
        name: String,
    },
}

/// Contains data that we haven't decoded for a chunk. Using `UnknownBuffer`
/// should generally be a placeholder since it's results are opaque, but stable.
#[derive(Debug)]
pub struct UnknownBuffer {
    contents: Vec<u8>,
}

impl From<Vec<u8>> for UnknownBuffer {
    fn from(contents: Vec<u8>) -> Self {
        Self { contents }
    }
}

impl From<&[u8]> for UnknownBuffer {
    fn from(contents: &[u8]) -> Self {
        Self {
            contents: contents.to_vec(),
        }
    }
}

impl Serialize for UnknownBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&base64::display::Base64Display::with_config(
            &self.contents,
            base64::STANDARD,
        ))
    }
}

impl<'de> Deserialize<'de> for UnknownBuffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = <&str>::deserialize(deserializer)?;
        let contents = base64::decode(encoded).map_err(serde::de::Error::custom)?;

        Ok(UnknownBuffer { contents })
    }
}
