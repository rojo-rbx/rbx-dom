use std::io::{self, Read, Write};

use rbx_dom_weak::types::{EmbeddedContent, SharedString};

use crate::core::{RbxReadExt, RbxWriteExt};

/// A reference from a binary PROP chunk to an archive object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedReference {
    /// The first 16 bytes of the object's BLAKE3 digest.
    pub digest: [u8; 16],
    /// The object's byte length, before any container compression.
    pub size: u64,
    /// Version 2's file-local opaque identity. Version 1 has no identity.
    pub identity: Option<String>,
}

/// Supplies local archive objects while decoding a binary scene.
///
/// Implementations should bound allocations and validate object digests. The
/// binary decoder verifies the returned length and digest. No network access is performed
/// by the decoder.
pub trait EmbeddedContentResolver {
    /// Loads the referenced object, retaining shared ownership where possible.
    fn resolve(&self, reference: &EmbeddedReference) -> io::Result<SharedString>;
}

/// Receives opaque objects while encoding a scene for an archive.
pub trait EmbeddedContentSink {
    /// Stores an envelope under its BLAKE3-128 digest. Repeated calls for the
    /// same bytes must be deduplicated by the container writer.
    fn store(&self, content: &EmbeddedContent) -> io::Result<()>;
}

pub(crate) fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl EmbeddedReference {
    pub(crate) fn decode(mut bytes: &[u8]) -> io::Result<Self> {
        let version = bytes.read_u8()?;
        if version != 1 && version != 2 {
            return Err(invalid("unsupported embedded reference version"));
        }
        let mut digest = [0; 16];
        bytes.read_exact(&mut digest)?;
        let mut size = [0; 8];
        bytes.read_exact(&mut size)?;
        let size = u64::from_le_bytes(size);
        let identity = if version == 2 {
            let length = bytes.read_le_u32()? as usize;
            if length > 269 || length != bytes.len() {
                return Err(invalid("invalid opaque identity length"));
            }
            let identity = std::str::from_utf8(bytes)
                .map_err(|_| invalid("opaque identity is not UTF-8"))?
                .to_owned();
            bytes = &[];
            Some(identity)
        } else {
            None
        };
        if size == 0 || !bytes.is_empty() {
            return Err(invalid("invalid embedded reference length"));
        }
        Ok(Self {
            digest,
            size,
            identity,
        })
    }

    pub(crate) fn encode_content(
        content: &EmbeddedContent,
        mut writer: impl Write,
    ) -> io::Result<()> {
        writer.write_u8(2)?;
        writer.write_all(&content.data.hash().as_bytes()[..16])?;
        writer.write_all(&(content.data.data().len() as u64).to_le_bytes())?;
        writer.write_string(&content.identity)
    }
}

/// Metadata is a sparse per-instance column. Negative lengths skip instances;
/// zero lengths explicitly leave the inline value in use.
pub(crate) fn read_references(
    reader: &mut &[u8],
    count: usize,
) -> io::Result<Vec<Option<EmbeddedReference>>> {
    let mut references = Vec::with_capacity(count);
    while references.len() < count {
        let length = reader.read_le_u32()? as i32;
        if length < 0 {
            let skip = length.unsigned_abs() as usize;
            if skip > count - references.len() {
                return Err(invalid("embedded metadata skip exceeds instance count"));
            }
            references.resize(references.len() + skip, None);
        } else if length == 0 {
            references.push(None);
        } else {
            let length = length as usize;
            if length > reader.len() {
                return Err(invalid("truncated embedded metadata"));
            }
            let (bytes, rest) = reader.split_at(length);
            *reader = rest;
            references.push(Some(EmbeddedReference::decode(bytes)?));
        }
    }
    Ok(references)
}
