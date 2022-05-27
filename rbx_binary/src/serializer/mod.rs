mod error;
mod state;

use std::io::Write;

use rbx_dom_weak::{types::Ref, WeakDom};

use self::state::SerializerState;

pub use self::error::Error;

/// A configurable serializer for Roblox binary models and places.
///
/// ## Example
/// ```no_run
/// use std::fs::File;
/// use std::io::BufWriter;
///
/// use rbx_binary::Serializer;
/// use rbx_dom_weak::{InstanceBuilder, WeakDom};
///
/// let dom = WeakDom::new(InstanceBuilder::new("Folder"));
///
/// let output = BufWriter::new(File::create("PlainFolder.rbxm")?);
/// let serializer = Serializer::new();
/// serializer.serialize(output, &dom, &[dom.root_ref()])?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
// future settings:
// * reflection_database: Option<ReflectionDatabase> = default
// * recursive: bool = true
#[non_exhaustive]
pub struct Serializer {}

impl Serializer {
    /// Create a new `Serializer` with the default settings.
    pub fn new() -> Self {
        Serializer {}
    }

    /// Serialize a Roblox binary model or place into the given stream using
    /// this serializer.
    pub fn serialize<W: Write>(&self, writer: W, dom: &WeakDom, refs: &[Ref]) -> Result<(), Error> {
        profiling::scope!("rbx_binary::seserialize");

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
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}
