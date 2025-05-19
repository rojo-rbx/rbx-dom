mod error;
mod file;
mod header;
mod intern;
mod state;

use std::str;

use rbx_dom_weak::WeakDom;
use rbx_reflection::ReflectionDatabase;

use self::state::DeserializerState;

pub(crate) use self::header::FileHeader;

pub use self::file::DecompressedFile;

pub use self::error::Error;
pub use self::intern::*;
pub use self::state::DecodeOptions;

/// A configurable deserializer for Roblox binary models and places.
///
/// ## Example
/// ```no_run
/// use std::fs::File;
/// use std::io::BufReader;
///
/// use rbx_binary::{DecodeOptions, DecompressedFile, Deserializer};
///
/// let input = BufReader::new(File::open("File.rbxm")?);
///
/// let file = DecompressedFile::from_reader(input)?;
/// let deserializer = Deserializer::new();
/// let dom = deserializer.deserialize(&file, DecodeOptions::read_unknown())?;
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
pub struct Deserializer<'de, 'db> {
    database: &'de ReflectionDatabase<'db>,
}

impl<'de, 'dom, 'db: 'dom, 'file> Deserializer<'de, 'db> {
    /// Create a new `Deserializer` with the default settings.
    pub fn new() -> Self {
        Self {
            database: rbx_reflection_database::get(),
        }
    }

    /// Sets what reflection database for the deserializer to use.
    #[inline]
    pub fn reflection_database(self, database: &'de ReflectionDatabase<'db>) -> Self {
        Self { database }
    }

    /// Deserialize a Roblox binary model or place from the given stream using
    /// this deserializer.
    pub fn deserialize<S: StringIntern<'file, 'dom>>(
        &self,
        file: &'file DecompressedFile,
        options: DecodeOptions<S>,
    ) -> Result<WeakDom<'dom>, Error> {
        profiling::scope!("rbx_binary::deserialize");

        let mut deserializer = DeserializerState::new(self, &file.header, options)?;

        for chunk in &file.chunks {
            match &chunk.name {
                b"META" => deserializer.decode_meta_chunk(&chunk.data)?,
                b"SSTR" => deserializer.decode_sstr_chunk(&chunk.data)?,
                b"INST" => deserializer.decode_inst_chunk(&chunk.data)?,
                b"PROP" => deserializer.decode_prop_chunk(&chunk.data)?,
                b"PRNT" => deserializer.decode_prnt_chunk(&chunk.data)?,
                b"END\0" => deserializer.decode_end_chunk(&chunk.data)?,
                _ => match str::from_utf8(&chunk.name) {
                    Ok(name) => log::info!("Unknown binary chunk name {}", name),
                    Err(_) => log::info!("Unknown binary chunk name {:?}", chunk.name),
                },
            }
        }

        Ok(deserializer.finish())
    }
}

impl Default for Deserializer<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}
