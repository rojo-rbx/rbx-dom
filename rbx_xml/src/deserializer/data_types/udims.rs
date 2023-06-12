//! Implements deserialization for `UDim` and `UDim2`
use std::io::BufRead;

use rbx_dom_weak::types::{UDim, UDim2};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

use super::{f32_deserializer, i32_deserializer};

pub fn udim_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<UDim, DecodeError> {
    Ok(UDim::new(
        reader.read_named_with("S", f32_deserializer)?,
        reader.read_named_with("O", i32_deserializer)?,
    ))
}

pub fn udim2_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<UDim2, DecodeError> {
    Ok(UDim2::new(
        UDim::new(
            reader.read_named_with("XS", f32_deserializer)?,
            reader.read_named_with("XO", i32_deserializer)?,
        ),
        UDim::new(
            reader.read_named_with("YS", f32_deserializer)?,
            reader.read_named_with("YO", i32_deserializer)?,
        ),
    ))
}
