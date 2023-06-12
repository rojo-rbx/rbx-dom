use std::io::BufRead;

use rbx_dom_weak::types::UniqueId;

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn unique_id_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<UniqueId, DecodeError> {
    reader
        .eat_text()?
        .parse()
        .map_err(|err| reader.error(format!("could not read UniqueId because {err}")))
}
