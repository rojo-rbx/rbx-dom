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
//! rbx_xml exposes no useful configuration yet, but there are methods that
//! accept [`DecodeOptions`][DecodeOptions] and
//! [`EncodeOptions`][EncodeOptions] that will be useful when it does.
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

// TODO redo documentation
#![deny(missing_docs)]

mod conversion;
mod deserializer;
mod error;
mod property_descriptor;
mod serializer;

#[cfg(test)]
mod test_util;

use std::io::{BufReader, Read, Write};

use rbx_dom_weak::{types::Ref, WeakDom};
use rbx_reflection::ReflectionDatabase;

use serializer::encode_internal;
pub use serializer::{EncodeError, EncodeOptions, EncodePropertyBehavior};

use deserializer::decode_internal;
pub use deserializer::{DecodeError, DecodeOptions, DecodePropertyBehavior};

/// Decodes an XML-format model or place from something that implements the
/// `std::io::Read` trait.
pub fn from_reader<R: Read>(reader: R, options: DecodeOptions) -> Result<WeakDom, DecodeError> {
    decode_internal(BufReader::new(reader), options)
}

/// Decodes an XML-format model or place from something that implements the
/// `std::io::Read` trait using the default decoder options.
pub fn from_reader_default<R: Read>(reader: R) -> Result<WeakDom, DecodeError> {
    decode_internal(
        reader,
        DecodeOptions::new().database(rbx_reflection_database::get()),
    )
}

/// Decodes an XML-format model or place from a string.
pub fn from_str<S: AsRef<str>>(reader: S, options: DecodeOptions) -> Result<WeakDom, DecodeError> {
    decode_internal(reader.as_ref().as_bytes(), options)
}

/// Decodes an XML-format model or place from a string using the default decoder
/// options.
pub fn from_str_default<S: AsRef<str>>(reader: S) -> Result<WeakDom, DecodeError> {
    decode_internal(
        reader.as_ref().as_bytes(),
        DecodeOptions::new().database(rbx_reflection_database::get()),
    )
}

/// Serializes a subset of the given tree to an XML format model or place,
/// writing to something that implements the `std::io::Write` trait.
pub fn to_writer<W: Write>(
    writer: &mut W,
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
    encode_internal(
        writer,
        tree,
        ids,
        EncodeOptions::new().database(rbx_reflection_database::get()),
    )
}

#[derive(Debug, Clone, Default)]
/// An indicator of what behavior to use when reading and writing properties.
pub enum PropertyBehavior {
    /// Ignore properties that aren't known by the database.
    ///
    /// This is the default behavior.
    #[default]
    IgnoreUnknown,

    /// Read and write unrecognized properties.
    ///
    /// With this option set, properties that are newer than the provided
    /// reflection database will show up. It may be problematic to depend on
    /// these properties, since future revisions to a reflection database
    /// may alter how they're read or written.
    AcceptUnknown,

    /// Returns an error if any properties are found that aren't known to
    /// the reflection database.
    ErrorOnUnknown,
}

/// Represents a configuration for deserializing and serializing an XML file.
/// Specifically, this allows the user to control whether a reflection database
/// is used, and if so what the behavior is when unknown properties are encountered.
///
/// The `'db` lifetime of this struct refers to the lifetime of the reflection
/// database, as it only holds a reference to it.
#[derive(Debug, Clone, Default)]
pub struct Config<'db> {
    database: Option<&'db ReflectionDatabase<'db>>,
    property_behavior: PropertyBehavior,
    migrate_properties: bool,
    check_class_names: bool,
}

impl<'db> Config<'db> {
    /// Creates a new `Config` with the default options. This means
    /// no database is used and unknown properties, classes, and datatypes are
    /// ignored during (de)serialization.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Config` with the given database. By default, this means
    /// this means that unknown properties will be read and written but
    /// properties may have their names and values translated to new ones
    /// according to the rules in the reflection database.
    pub fn with_database(database: &'db ReflectionDatabase<'db>) -> Self {
        Self {
            database: Some(database),
            property_behavior: PropertyBehavior::AcceptUnknown,
            migrate_properties: true,
            check_class_names: false,
        }
    }

    /// Sets a database to use during (de)serialization.
    pub fn database(mut self, database: &'db ReflectionDatabase<'db>) -> Self {
        self.database = Some(database);
        self
    }

    /// Sets whether property names and values are migrated during
    /// (de)serialization.
    pub fn migrate_properties(mut self, migrate: bool) -> Self {
        self.migrate_properties = migrate;
        self
    }

    /// Sets the behavior to use when (de)serializing properties. For more
    /// information, see `PropertyBehavior.
    pub fn property_behavior(mut self, behavior: PropertyBehavior) -> Self {
        self.property_behavior = behavior;
        self
    }

    /// Whether to ignore uknown properties
    pub(crate) fn ignore_unknown(&self) -> bool {
        matches!(self.property_behavior, PropertyBehavior::IgnoreUnknown)
    }

    pub(crate) fn error_on_unknown(&self) -> bool {
        matches!(self.property_behavior, PropertyBehavior::ErrorOnUnknown)
    }
}
