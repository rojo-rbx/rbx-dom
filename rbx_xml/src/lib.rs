//! Configurable Roblox XML place/model format (rbxmx and rbxlx) serializer and
//! deserializer.
//!
//! rbx_xml uses the [rbx_dom_weak][rbx_dom_weak] crate as its DOM.
//!
//! This crate implements most of the format and is driven by an up-to-date
//! reflection database.
//!
//! ## Deserialization
//! To decode a place or model, use a method like
//! [`from_reader_default`][from_reader_default] if you're reading from a file,
//! or [`from_str_default`][from_str_default] if you already have a string.
//! These methods also have variants like [`from_str`][from_str] that let you
//! pass in custom options.
//!
//! ```
//! use rbx_dom_weak::types::Variant;
//!
//! let model_file = r#"
//! <roblox version="4">
//!     <Item class="NumberValue" referent="RBX3B3D9D3DB43D4E6793B190B081E0A886">
//!         <Properties>
//!             <string name="Name">My NumberValue</string>
//!             <double name="Value">12345</double>
//!         </Properties>
//!     </Item>
//! </roblox>
//! "#;
//!
//! let model = rbx_xml::from_str_default(model_file)?;
//!
//! let data_model = model.root();
//! let number_value_ref = data_model.children()[0];
//! let number_value = model.get_by_ref(number_value_ref).unwrap();
//!
//! assert_eq!(
//!     number_value.properties.get("Value"),
//!     Some(&Variant::Float64(12345.0)),
//! );
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! If you're decoding from a file, you'll want to do your own I/O buffering,
//! like with [`BufReader`][BufReader]:
//!
//! ```no_run
//! use std::{
//!     io::BufReader,
//!     fs::File,
//! };
//!
//! let file = BufReader::new(File::open("place.rbxlx")?);
//! let place = rbx_xml::from_reader_default(file)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Note that the `WeakDom` instance returned by the rbx_xml decode methods will
//! have a root instance with the class name `DataModel`. This is great for
//! deserializing a place, but kind of strange for deserializing a model.
//!
//! Because models can have multiple instances at the top level, rbx_xml can't
//! just return an `WeakDom` with your single instance at the top. Instead, the
//! crate instead always creates a top-level `DataModel` instance which is
//! pretty close to free.
//!
//! ## Serialization
//! To serialize an existing `WeakDom` instance, use methods like
//! [`to_writer_default`][to_writer_default] or [`to_writer`][to_writer].
//!
//! For example, to re-save the place file we loaded above:
//!
//! ```no_run
//! use std::{
//!     io::BufWriter,
//!     fs::File,
//! };
//! use rbx_dom_weak::{WeakDom, InstanceBuilder};
//!
//! let place = WeakDom::new(InstanceBuilder::new("DataModel"));
//!
//! // A Roblox place file contains all of its top-level instances.
//! let top_level_refs = place.root().children();
//!
//! // Just like when reading a place file, we should buffer our I/O.
//! let file = BufWriter::new(File::create("place-2.rbxlx")?);
//!
//! rbx_xml::to_writer_default(file, &place, top_level_refs)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Configuration
//! rbx_xml exposes a few configuration options at the moment in the form of
//! [`DecodeOptions`][DecodeOptions] and [`EncodeOptions`][EncodeOptions].
//! For information on the configuration, see the documentation for those
//! structs.
//!
//! The non-default reader and writer functions accept these as their `options`
//! argument.
//!
//! [DecodeOptions]: struct.DecodeOptions.html
//! [EncodeOptions]: struct.EncodeOptions.html
//! [from_str]: fn.from_str.html
//! [from_reader_default]: fn.from_reader_default.html
//! [from_str_default]: fn.from_str_default.html
//! [to_writer]: fn.to_writer.html
//! [to_writer_default]: fn.to_writer_default.html
//! [rbx_dom_weak]: https://crates.io/crates/rbx_dom_weak
//! [BufReader]: https://doc.rust-lang.org/std/io/struct.BufReader.html

#![deny(missing_docs)]

mod conversion;
mod core;
mod deserializer;
mod deserializer_core;
mod error;
mod serializer;
mod serializer_core;
mod types;

#[cfg(test)]
mod test_util;
#[cfg(test)]
mod tests;

use std::io::{Read, Write};

use rbx_dom_weak::{types::Ref, WeakDom};

use crate::{deserializer::decode_internal, serializer::encode_internal};

pub use crate::{
    deserializer::{DecodeOptions, DecodePropertyBehavior},
    error::{DecodeError, EncodeError},
    serializer::{EncodeOptions, EncodePropertyBehavior},
};

/// Decodes an XML-format model or place from something that implements the
/// `std::io::Read` trait.
pub fn from_reader<R: Read>(reader: R, options: DecodeOptions) -> Result<WeakDom, DecodeError> {
    decode_internal(reader, options)
}

/// Decodes an XML-format model or place from something that implements the
/// `std::io::Read` trait using the default decoder options.
pub fn from_reader_default<R: Read>(reader: R) -> Result<WeakDom, DecodeError> {
    decode_internal(reader, DecodeOptions::default())
}

/// Decodes an XML-format model or place from a string.
pub fn from_str<S: AsRef<str>>(reader: S, options: DecodeOptions) -> Result<WeakDom, DecodeError> {
    decode_internal(reader.as_ref().as_bytes(), options)
}

/// Decodes an XML-format model or place from a string using the default decoder
/// options.
pub fn from_str_default<S: AsRef<str>>(reader: S) -> Result<WeakDom, DecodeError> {
    decode_internal(reader.as_ref().as_bytes(), DecodeOptions::default())
}

/// Serializes a subset of the given tree to an XML format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer<W: Write>(
    writer: W,
    tree: &WeakDom,
    ids: &[Ref],
    options: EncodeOptions,
) -> Result<(), EncodeError> {
    encode_internal(writer, tree, ids, options)
}

/// Serializes a subset of the given tree to an XML format model or place,
/// writing to something that implements the `std::io::Write` trait using the
/// default encoder options.
pub fn to_writer_default<W: Write>(
    writer: W,
    tree: &WeakDom,
    ids: &[Ref],
) -> Result<(), EncodeError> {
    encode_internal(writer, tree, ids, EncodeOptions::default())
}
