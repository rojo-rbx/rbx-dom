/// Super early binary format (rbxm and rbxl) serializer and deserializer for
/// rbx-tree.

mod serializer;
mod deserializer;

pub use crate::{
    serializer::encode,
    deserializer::{decode, decode_str, DecodeError},
};