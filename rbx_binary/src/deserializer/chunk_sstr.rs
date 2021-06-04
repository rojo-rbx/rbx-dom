use std::io::Read;

use rbx_dom_weak::types::SharedString;

use crate::core::RbxReadExt;

use super::error::{Error, InnerError};

pub(crate) fn deserialize(mut chunk: &[u8]) -> Result<Vec<SharedString>, Error> {
    let version = chunk.read_le_u32()?;

    if version != 0 {
        return Err(Error::from(InnerError::UnknownChunkVersion {
            chunk_name: "SSTR",
            version,
        }));
    }

    let num_entries = chunk.read_le_u32()?;
    let mut shared_strings = Vec::with_capacity(num_entries as usize);

    for _ in 0..num_entries {
        chunk.read_exact(&mut [0; 16])?; // We don't do anything with the hash.
        let data = chunk.read_binary_string()?;
        shared_strings.push(SharedString::new(data));
    }

    Ok(shared_strings)
}
