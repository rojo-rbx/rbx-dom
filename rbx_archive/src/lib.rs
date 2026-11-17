//! Read and write version-1 JSONL Roblox place archives (`.rbx`).
//!
//! Archives contain a binary scene and content-addressed objects. The scene is
//! decoded through `rbx_binary`, including embedded strings and opaque Content.
//! No HTTP requests are made; remote asset references stay references.
//!
//! ```no_run
//! use std::{fs::File, io::{BufReader, BufWriter}};
//! let dom = rbx_archive::from_reader(BufReader::new(File::open("place.rbx")?))?;
//! rbx_archive::to_writer(
//!     BufWriter::new(File::create("copy.rbx")?), &dom, dom.root().children(),
//! )?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![deny(missing_docs)]

mod envelope;
mod manifest;
mod reader;
mod writer;

pub use envelope::ContentEnvelope;
pub use manifest::{ContentRecord, Manifest};
pub use reader::Reader;
pub use writer::Writer;

use rbx_dom_weak::{types::Ref, WeakDom};
use std::io::{Read, Seek, Write};

/// Errors produced by container validation or scene serialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A malformed or unsupported archive feature.
    #[error("invalid archive: {0}")]
    Invalid(&'static str),
    /// A configured resource limit was exceeded.
    #[error("archive limit exceeded: {0}")]
    Limit(&'static str),
    /// Underlying I/O failure.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// ZIP container failure.
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    /// Invalid JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Binary scene decode failure.
    #[error(transparent)]
    Decode(#[from] rbx_binary::DecodeError),
    /// Binary scene encode failure.
    #[error(transparent)]
    Encode(#[from] rbx_binary::EncodeError),
}

/// Result of an archive operation.
pub type Result<T> = std::result::Result<T, Error>;

/// Container and embedded-object budgets. These do not bound the complete DOM
/// allocation or replace the binary decoder's own input validation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Limits {
    /// Total input archive bytes (default 512 MiB).
    pub max_archive_bytes: u64,
    /// Aggregate physical object bytes (default 512 MiB).
    pub max_total_object_bytes: u64,
    /// One scene or embedded object (default 256 MiB).
    pub max_object_bytes: u64,
    /// Manifest bytes (default 16 MiB).
    pub max_manifest_bytes: u64,
    /// One JSONL record (default 1 MiB).
    pub max_record_bytes: u64,
    /// Physical ZIP entries and logical content records (default 65,534).
    pub max_entries: usize,
    /// Aggregate resolved property bytes, counting repeated references (512 MiB).
    pub max_restored_bytes: u64,
    /// JSON bytes inside an opaque envelope (default 1 MiB).
    pub max_runtime_metadata_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 512 * 1024 * 1024,
            max_total_object_bytes: 512 * 1024 * 1024,
            max_object_bytes: 256 * 1024 * 1024,
            max_manifest_bytes: 16 * 1024 * 1024,
            max_record_bytes: 1024 * 1024,
            max_entries: 65_534,
            max_restored_bytes: 512 * 1024 * 1024,
            max_runtime_metadata_bytes: 1024 * 1024,
        }
    }
}

/// Decodes an archive into a DataModel-rooted DOM using default limits.
pub fn from_reader<R: Read + Seek>(input: R) -> Result<WeakDom> {
    Reader::new(input, Limits::default())?.deserialize()
}

/// Writes a place archive containing the selected instances and descendants.
/// Strings remain inline; embedded Content is stored in deduplicated objects.
/// This serializes a DOM, so unattached objects and manifest metadata from an
/// input archive are not retained. Use [`Reader::copy_to`] for container copying.
pub fn to_writer<W: Write + Seek>(output: W, dom: &WeakDom, refs: &[Ref]) -> Result<()> {
    Writer::default().serialize(output, dom, refs)
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..32].to_owned()
}

pub(crate) fn invalid_io(
    error: impl Into<Box<dyn std::error::Error + Send + Sync>>,
) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, error)
}
