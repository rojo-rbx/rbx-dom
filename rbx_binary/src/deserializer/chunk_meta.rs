use std::collections::HashMap;

use crate::core::RbxReadExt;

use super::error::Error;

pub(crate) fn deserialize(mut chunk: &[u8]) -> Result<HashMap<String, String>, Error> {
    let len = chunk.read_le_u32()?;
    let mut metadata = HashMap::with_capacity(len as usize);

    for _ in 0..len {
        let key = chunk.read_string()?;
        let value = chunk.read_string()?;

        metadata.insert(key, value);
    }

    Ok(metadata)
}
