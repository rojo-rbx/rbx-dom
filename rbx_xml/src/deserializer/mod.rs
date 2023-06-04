mod conversions;
mod core;
mod data_types;
mod error;
mod reader;

use std::io::Read;

use rbx_dom_weak::WeakDom;

pub use self::core::DecodeConfig;
pub use error::DecodeError;
pub use reader::XmlReader;

pub fn decode_internal<R: Read>(reader: R, config: DecodeConfig) -> Result<WeakDom, DecodeError> {
    core::deserialize_file(XmlReader::from_reader(reader), config)
}
