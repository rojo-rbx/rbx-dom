//! Super early, unstable XML format (rbxmx and rbxlx) serializer and
//! deserializer for rbx-dom.
//!
//! The serializer is functional and can write instances with string or bool
//! values, but the deserializer currently does not finish constructing
//! instances.

#[macro_use]
mod macros;

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
    serializer::{encode, EncodeError},
    deserializer::{decode, decode_str},
    error::DecodeError,
};