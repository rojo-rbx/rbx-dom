use std::io::BufRead;

use rbx_dom_weak::types::Ray;

use super::vector3_deserializer;
use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn ray_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Ray, DecodeError> {
    Ok(Ray::new(
        reader.read_named_with("origin", vector3_deserializer)?,
        reader.read_named_with("direction", vector3_deserializer)?,
    ))
}
