//! Super early, unstable binary format (rbxm and rbxl) serializer and
//! deserializer for rbx-dom.
//!
//! Both the serializer and deserializer are functioning for limited property
//! types. `String` and `Bool` (from the `rbx_dom_weak` crate) are the only
//! supported values. Unrecognized values will be ignored when deserializing,
//! and cause a panic when serializing.

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
