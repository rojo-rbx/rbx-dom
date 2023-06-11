mod conversions;
mod core;
pub(crate) mod data_types;
mod error;
mod reader;

use std::io::Read;

use rbx_dom_weak::WeakDom;

use crate::Config;

pub use error::DecodeError;
pub use reader::XmlReader;

pub(crate) fn decode_internal<R: Read>(reader: R, config: Config) -> Result<WeakDom, DecodeError> {
    core::deserialize_file(XmlReader::from_reader(reader), &config)
}
