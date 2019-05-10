//! XML format (rbxmx and rbxlx) serializer and deserializer for rbx-dom.
//!
//! Implements most types, and is driven by an up-to-date reflection database.

#![deny(missing_docs)]

mod core;
mod deserializer;
mod deserializer_core;
mod error;
mod serializer;
mod serializer_core;
mod types;

#[cfg(test)]
mod test_util;

pub use crate::{
    serializer::to_writer,
    deserializer::{from_reader, from_str},
    error::{EncodeError, DecodeError},
};