use std::io::BufRead;

use rbx_dom_weak::types::Vector3;

use super::f32_deserializer;
use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn vector3_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Vector3, DecodeError> {
    reader.expect_start_with_name("X")?;
    let x = f32_deserializer(reader)?;
    reader.expect_end_with_name("X")?;
    reader.expect_start_with_name("Y")?;
    let y = f32_deserializer(reader)?;
    reader.expect_end_with_name("Y")?;
    reader.expect_start_with_name("Z")?;
    let z = f32_deserializer(reader)?;
    reader.expect_end_with_name("Z")?;

    Ok(Vector3::new(x, y, z))
}
