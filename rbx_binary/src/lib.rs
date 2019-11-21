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
mod serializer_new;
mod types;
mod types_new;

pub use crate::{
    deserializer::{decode, DecodeError},
    serializer_new::encode,
};
