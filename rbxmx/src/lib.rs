/// Super early XML format (rbxmx and rbxlx) serializer for rbx-tree.

extern crate rbx_tree;
extern crate xml;

mod serializer;
mod deserializer;

pub use crate::{
    serializer::encode,
    deserializer::{decode, decode_str, DecodeError},
};