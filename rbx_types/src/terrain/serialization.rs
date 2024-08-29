use crate::{Chunk, SmoothGrid};

use serde::de::{self, Deserialize, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};

struct SmoothGridVisitor;
struct ChunkVisitor;

fn base64_or_unexpected<E>(str: &str) -> Result<Vec<u8>, E>
where
    E: de::Error,
{
    match base64::decode(str) {
        Ok(res) => Ok(res),
        Err(_) => Err(de::Error::custom("invalid base64 encountered")),
    }
}

impl<'de> Visitor<'de> for SmoothGridVisitor {
    type Value = SmoothGrid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct SmoothGrid")
    }

    fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded_bytes = base64_or_unexpected(str)?;

        match SmoothGrid::decode(&decoded_bytes[..]) {
            Ok(grid) => Ok(grid),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Str(str), &self)),
        }
    }
}

impl Serialize for SmoothGrid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded_world = base64::encode(self.encode());
        serializer.serialize_str(&encoded_world)
    }
}

impl<'de> Deserialize<'de> for SmoothGrid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SmoothGridVisitor)
    }
}

impl<'de> Visitor<'de> for ChunkVisitor {
    type Value = Chunk;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Chunk")
    }

    fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded_bytes = base64_or_unexpected(str)?;

        match Chunk::decode(&decoded_bytes[..]) {
            Ok(grid) => Ok(grid),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Str(str), &self)),
        }
    }
}

impl Serialize for Chunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded_chunk = base64::encode(self.encode());
        serializer.serialize_str(&encoded_chunk)
    }
}

impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ChunkVisitor)
    }
}
