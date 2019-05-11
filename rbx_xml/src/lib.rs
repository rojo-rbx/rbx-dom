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

use std::io::{Read, Write};

use rbx_dom_weak::{RbxTree, RbxId};

use crate::{
    deserializer::decode_internal,
    serializer::encode_internal,
};

pub use crate::{
    error::{EncodeError, DecodeError},
    deserializer::DecodeOptions,
    serializer::EncodeOptions,
};

/// Decodes an XML-format model or place from anything that implements the
/// `std::io::Read` trait.
pub fn from_reader<R: Read>(reader: R, options: DecodeOptions) -> Result<RbxTree, DecodeError> {
    decode_internal(reader, options)
}

/// Decodes an XML-format model or place from anything that implements the
/// `std::io::Read` trait.
pub fn from_reader_default<R: Read>(reader: R) -> Result<RbxTree, DecodeError> {
    decode_internal(reader, DecodeOptions::default())
}

/// Decodes an XML-format model or place from a string.
pub fn from_str<S: AsRef<str>>(reader: S, options: DecodeOptions) -> Result<RbxTree, DecodeError> {
    decode_internal(reader.as_ref().as_bytes(), options)
}

/// Decodes an XML-format model or place from a string.
pub fn from_str_default<S: AsRef<str>>(reader: S) -> Result<RbxTree, DecodeError> {
    decode_internal(reader.as_ref().as_bytes(), DecodeOptions::default())
}

/// Serializes a subset of the given tree to an XML format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer<W: Write>(writer: W, tree: &RbxTree, ids: &[RbxId], options: EncodeOptions) -> Result<(), EncodeError> {
    encode_internal(writer, tree, ids, options)
}

/// Serializes a subset of the given tree to an XML format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer_default<W: Write>(writer: W, tree: &RbxTree, ids: &[RbxId]) -> Result<(), EncodeError> {
    encode_internal(writer, tree, ids, EncodeOptions::default())
}