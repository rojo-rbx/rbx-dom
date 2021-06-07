mod error;
mod state;

use std::io::Write;

use rbx_dom_weak::{types::Ref, WeakDom};

use self::state::SerializerState;

pub use self::error::Error;

/// Serializes instances from an `WeakDom` into a writer in Roblox's binary
/// model format.
pub fn encode<W: Write>(dom: &WeakDom, refs: &[Ref], writer: W) -> Result<(), Error> {
    let mut serializer = SerializerState::new(dom, writer);

    serializer.add_instances(refs)?;
    serializer.generate_referents();
    serializer.write_header()?;
    serializer.serialize_metadata()?;
    serializer.serialize_shared_strings()?;
    serializer.serialize_instances()?;
    serializer.serialize_properties()?;
    serializer.serialize_parents()?;
    serializer.serialize_end()?;

    Ok(())
}
