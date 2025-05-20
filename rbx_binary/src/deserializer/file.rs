use std::io::Read;

use rbx_dom_weak::WeakDom;

use super::{error::InnerError, header::FileHeader};
use crate::chunk::Chunks;
use crate::deserializer::{Deserializer, Error};

/// File header and decompressed chunks.  Call deserialize to
/// deserialize the file.
pub struct DecompressedFile {
    pub(crate) header: FileHeader,
    pub(crate) chunks: Chunks,
}

impl DecompressedFile {
    /// Perform the chunk decompression step without deserializing the file.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, Error> {
        let header = FileHeader::decode(&mut reader)?;
        let chunks = Chunks::decode(reader).map_err(InnerError::from)?;
        Ok(DecompressedFile { header, chunks })
    }
    /// Perform the deserialization step.
    pub fn deserialize(&self) -> Result<WeakDom, Error> {
        Deserializer::new().deserialize(self)
    }
}
