mod error;
mod state;

use std::io::Write;

use rbx_dom_weak::{types::Ref, WeakDom};
use rbx_reflection::ReflectionDatabase;

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
///
/// ## Configuration
///
/// A custom [`ReflectionDatabase`][ReflectionDatabase] can be specified via
/// [`reflection_database`][reflection_database].
///
/// [ReflectionDatabase]: rbx_reflection::ReflectionDatabase
/// [reflection_database]: Serializer#method.reflection_database
//
// future settings:
// * recursive: bool = true
#[non_exhaustive]
pub struct Serializer<'db> {
    database: &'db ReflectionDatabase<'db>,
}

impl<'db> Serializer<'db> {
    /// Create a new `Serializer` with the default settings.
    pub fn new() -> Self {
        Serializer {
            database: rbx_reflection_database::get(),
        }
    }

    /// Sets what reflection database for the serializer to use.
    #[inline]
    pub fn reflection_database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        Self { database }
    }

    /// Serialize a Roblox binary model or place into the given stream using
    /// this serializer.
    pub fn serialize<W: Write>(&self, writer: W, dom: &WeakDom, refs: &[Ref]) -> Result<(), Error> {
        profiling::scope!("rbx_binary::seserialize");

        let mut serializer = SerializerState::new(self, dom, writer);

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

impl<'db> Default for Serializer<'db> {
    fn default() -> Self {
        Self::new()
    }
}
