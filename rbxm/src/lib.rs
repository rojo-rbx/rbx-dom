/// Super early XML format (rbxmx and rbxlx) serializer and deserializer for
/// rbx-tree.

mod serializer;
mod deserializer;

pub use crate::{
    serializer::encode,
    deserializer::{decode, DecodeError},
};