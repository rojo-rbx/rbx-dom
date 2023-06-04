use std::io::BufRead;

use rbx_dom_weak::types::Ray;

use super::vector3_deserializer;
use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn ray_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Ray, DecodeError> {
    reader.expect_start_with_name("origin")?;
    let origin = vector3_deserializer(reader)?;
    reader.expect_end_with_name("origin")?;
    reader.expect_start_with_name("direction")?;
    let direction = vector3_deserializer(reader)?;
    reader.expect_end_with_name("direction")?;

    Ok(Ray::new(origin, direction))
}
