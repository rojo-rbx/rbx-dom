//! Implementation of Roblox's binary model (rbxm) and place (rbxl) file
//! formats.
//!
//! rbx_binary has limited property support. See [the rbx-dom
//! homepage](https://github.com/rojo-rbx/rbx-dom#readme) for details on what
//! support rbx_binary and its sibling crates have.

#![deny(missing_docs)]

mod chunk;
mod core;
mod deserializer;
mod serializer;
mod types;

#[cfg(test)]
mod text_deserializer;

#[cfg(test)]
mod tests;

pub use {
    deserializer::{decode_compat as decode, Error as DecodeError},
    serializer::{encode, Error as EncodeError},
};
