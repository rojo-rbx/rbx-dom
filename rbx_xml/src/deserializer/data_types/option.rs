use std::io::BufRead;

use rbx_dom_weak::types::CFrame;

use super::cframe_deserializer;
use crate::deserializer::{
    error::DecodeError,
    reader::{XmlData, XmlReader},
};

pub fn optional_cframe_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<Option<CFrame>, DecodeError> {
    match reader.peek() {
        Some(Ok(XmlData::ElementStart { name, .. })) if name == "CFrame" => {
            reader.expect_start_with_name("CFrame")?;
            let cf = cframe_deserializer(reader)?;
            reader.expect_end_with_name("CFrame")?;
            Ok(Some(cf))
        }
        _ => Ok(None),
    }
}
