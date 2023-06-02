mod conversions;
mod core;
mod data_types;
mod error;
mod reader;

use std::io::Read;

pub use self::core::DecodeConfig;
pub use error::DecodeError;
use rbx_dom_weak::WeakDom;
pub use reader::XmlReader;

pub fn decode_internal<R: Read>(reader: R, option: DecodeConfig) -> Result<WeakDom, DecodeError> {
    core::deserialize_file(XmlReader::from_reader(reader), option)
}
