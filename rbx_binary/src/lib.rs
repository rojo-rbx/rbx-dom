/// Super early binary format (rbxm and rbxl) serializer and deserializer for
/// rbx-tree.

mod core;
mod types;
mod serializer;
mod deserializer;

pub use crate::{
    serializer::{encode, EncodeError},
    deserializer::{decode, DecodeError},
};