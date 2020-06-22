//! Implementation of Roblox's binary model (rbxm) and place (rbxl) file
//! formats.
//!
//! rbx_binary has limited property support. See [the rbx-dom
//! homepage](https://github.com/Roblox/rbx-dom#readme) for details on what
//! support rbx_binary and its sibling crates have.

#![deny(missing_docs)]

mod chunk;
mod core;
mod deserializer;
mod serializer;
mod types;

#[cfg(any(test, feature = "unstable_text_format"))]
mod text_deserializer;

#[cfg(test)]
mod tests;

use std::io::{Read, Write};

use rbx_dom_weak::{types::Ref, WeakDom};

use crate::{deserializer::decode, serializer::encode};

/// An unstable textual format that can be used to debug binary models.
#[cfg(feature = "unstable_text_format")]
pub mod text_format {
    pub use crate::text_deserializer::*;
}

pub use crate::{deserializer::Error as DecodeError, serializer::Error as EncodeError};

/// Decodes an binary format model or place from something that implements the
/// `std::io::Read` trait.
pub fn from_reader_default<R: Read>(reader: R) -> Result<WeakDom, DecodeError> {
    decode(reader)
}

/// Serializes a subset of the given DOM to a binary format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer_default<W: Write>(
    writer: W,
    dom: &WeakDom,
    refs: &[Ref],
) -> Result<(), EncodeError> {
    encode(dom, refs, writer)
}
