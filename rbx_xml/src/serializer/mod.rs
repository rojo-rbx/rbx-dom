mod conversions;
mod core;
mod data_types;
mod error;
mod writer;

use std::io::Write;

use rbx_dom_weak::{types::Ref, WeakDom};

use crate::Config;

pub use error::EncodeError;
pub use writer::XmlWriter;

pub fn encode_internal<W: Write>(
    output: &mut W,
    dom: &WeakDom,
    refs: &[Ref],
    config: Config,
) -> Result<(), EncodeError> {
    core::serialize_refs(output, dom, refs, &config)
}
