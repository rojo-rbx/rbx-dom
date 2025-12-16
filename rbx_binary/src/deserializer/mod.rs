mod error;
mod header;
mod state;

use std::{io::Read, str};

use rbx_dom_weak::WeakDom;
use rbx_reflection::ReflectionDatabase;

use self::state::DeserializerState;

#[cfg(any(test, feature = "unstable_text_format"))]
pub(crate) use self::header::FileHeader;

pub use self::error::Error;

/// A configurable deserializer for Roblox binary models and places.
///
/// ## Example
/// ```no_run
/// use std::fs::File;
/// use std::io::BufReader;
///
/// use rbx_binary::Deserializer;
///
/// let input = BufReader::new(File::open("File.rbxm")?);
///
/// let deserializer = Deserializer::new();
/// let dom = deserializer.deserialize(input)?;
///
/// // rbx_binary always returns a DOM with a DataModel at the top level.
/// // To get to the instances from our file, we need to go one level deeper.
///
/// println!("Root instances in file:");
/// for &referent in dom.root().children() {
///     let instance = dom.get_by_ref(referent).unwrap();
///     println!("- {}", instance.name);
/// }
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
/// [reflection_database]: Deserializer#method.reflection_database
pub struct Deserializer<'db> {
    database: &'db ReflectionDatabase<'db>,
}

impl<'db> Deserializer<'db> {
    /// Create a new `Deserializer` with the default settings.
    pub fn new() -> Self {
        Self {
            database: rbx_reflection_database::get().unwrap(),
        }
    }

    /// Sets what reflection database for the deserializer to use.
    #[inline]
    pub fn reflection_database(self, database: &'db ReflectionDatabase<'db>) -> Self {
        Self { database }
    }

    /// Deserialize a Roblox binary model or place from the given stream using
    /// this deserializer.
    pub fn deserialize<R: Read>(&self, reader: R) -> Result<WeakDom, Error> {
        profiling::scope!("rbx_binary::deserialize");

        let d_meta = DeserializerState::new(self, reader)?;
        let d_sstr = d_meta.decode_optional()?;
        let d_inst = d_sstr.decode_optional()?;
        let d_prop = d_inst.decode_many()?;
        let d_prnt = d_prop.decode_many()?;
        let d_end = d_prnt.decode_optional()?;

        Ok(d_end.finish())
    }
}

impl Default for Deserializer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
