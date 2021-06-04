mod error;
mod header;
mod state;

use std::{io::Read, str};

use rbx_dom_weak::WeakDom;

use self::state::DeserializerState;

pub(crate) use self::header::FileHeader;

pub use self::error::Error;

pub(crate) fn decode<R: Read>(reader: R) -> Result<WeakDom, Error> {
    let mut deserializer = DeserializerState::new(reader)?;

    loop {
        let chunk = deserializer.next_chunk()?;

        match &chunk.name {
            b"META" => deserializer.decode_meta_chunk(&chunk.data)?,
            b"SSTR" => deserializer.decode_sstr_chunk(&chunk.data)?,
            b"INST" => deserializer.decode_inst_chunk(&chunk.data)?,
            b"PROP" => deserializer.decode_prop_chunk(&chunk.data)?,
            b"PRNT" => deserializer.decode_prnt_chunk(&chunk.data)?,
            b"END\0" => {
                deserializer.decode_end_chunk(&chunk.data)?;
                break;
            }
            _ => match str::from_utf8(&chunk.name) {
                Ok(name) => log::info!("Unknown binary chunk name {}", name),
                Err(_) => log::info!("Unknown binary chunk name {:?}", chunk.name),
            },
        }
    }

    Ok(deserializer.finish())
}
