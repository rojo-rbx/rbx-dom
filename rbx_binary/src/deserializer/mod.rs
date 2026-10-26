mod diagnostic_sink;
mod error;
mod header;
mod state;

use std::{io::Read, str};

use rbx_dom_weak::WeakDom;
use rbx_reflection::ReflectionDatabase;

use self::diagnostic_sink::{CollectDiagnostics, DiagnosticSink, DiscardDiagnostics};
use self::state::DeserializerState;
use crate::DecodeReport;

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
    ///
    /// This entry does not retain diagnostic payloads. Use
    /// [`Self::deserialize_with_diagnostics`] when source-loss evidence is needed.
    pub fn deserialize<R: Read>(&self, reader: R) -> Result<WeakDom, Error> {
        self.deserialize_with_sink::<R, DiscardDiagnostics>(reader)
    }

    /// Decodes once, returning the DOM together with structured evidence of
    /// source information that permissive decoding omitted or replaced.
    ///
    /// This accepts the same inputs as [`Self::deserialize`]. Repeated losses
    /// are retained even when their log messages are suppressed. Errors that
    /// prevent decoding still return `Err`, rather than a partial report.
    ///
    /// See [`DecodeDiagnostic`](crate::DecodeDiagnostic) for the recorded loss
    /// conditions and [`DecodeReport::diagnostics`] for their ordering.
    pub fn deserialize_with_diagnostics<R: Read>(&self, reader: R) -> Result<DecodeReport, Error> {
        self.deserialize_with_sink::<R, CollectDiagnostics>(reader)
    }

    fn deserialize_with_sink<R: Read, S: DiagnosticSink>(
        &self,
        reader: R,
    ) -> Result<S::Output, Error> {
        profiling::scope!("rbx_binary::deserialize");

        let mut deserializer = DeserializerState::<R, S>::new(self, reader)?;

        loop {
            let chunk = deserializer.next_chunk()?;

            match &chunk.name {
                b"META" => deserializer.decode_meta_chunk(&chunk.data)?,
                b"SSTR" => deserializer.decode_sstr_chunk(&chunk.data)?,
                b"INST" => deserializer.decode_inst_chunk(&chunk.data)?,
                b"PROP" => deserializer.decode_prop_chunk(&chunk.data)?,
                b"PRNT" => deserializer.decode_prnt_chunk(&chunk.data)?,
                b"END\0" => {
                    deserializer.decode_end_chunk(&chunk.data)?;
                    break;
                }
                _ => {
                    match str::from_utf8(&chunk.name) {
                        Ok(name) => log::info!("Unknown binary chunk name {name}"),
                        Err(_) => log::info!("Unknown binary chunk name {:?}", chunk.name),
                    }
                    deserializer.ignore_unknown_chunk(chunk);
                }
            }
        }

        Ok(deserializer.finish())
    }
}

impl Default for Deserializer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
