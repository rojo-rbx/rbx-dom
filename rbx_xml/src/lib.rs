//! Super early, unstable XML format (rbxmx and rbxlx) serializer and
//! deserializer for rbx-tree.
//!
//! The serializer is functional and can write instances with string or bool
//! values, but the deserializer currently does not finish constructing
//! instances.

mod serializer;
mod deserializer;

pub use crate::{
    serializer::{encode, EncodeError},
    deserializer::{decode, decode_str, DecodeError},
};