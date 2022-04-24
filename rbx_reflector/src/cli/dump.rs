use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{bail, Context};
use clap::Parser;
use fs_err::File;

use crate::api_dump::{database_from_dump, Dump};

/// Generate a reflection database from the system's Roblox Studio installation
/// and write it to disk.
#[derive(Debug, Parser)]
pub struct DumpSubcommand {
    /// Where to output the reflection database. The output format is inferred
    /// from the file path and supports JSON (.json) and MessagePack (.msgpack).
    pub output: Vec<PathBuf>,
}

impl DumpSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let dump = Dump::read().context("Could not read API dump from Roblox Studio")?;
        let database = database_from_dump(&dump);

        for path in &self.output {
            let extension = path.extension().with_context(|| {
                format!("Could not infer output type for path {}", path.display())
            })?;

            let mut file = BufWriter::new(File::create(path)?);

            match extension.to_str() {
                Some("json") => {
                    serde_json::to_writer_pretty(&mut file, &database)
                        .context("Could not serialize reflection database as JSON")?;
                }
                Some("msgpack") => {
                    let buf = rmp_serde::to_vec(&database)
                        .context("Could not serialize reflection database as MessagePack")?;

                    file.write_all(&buf)?;
                }
                _ => bail!(
                    "Unknown output format for path {} -- \
                    Supported formats are JSON (.json) and MessagePack (.msgpack)",
                    path.display()
                ),
            }

            file.flush()?;
        }

        Ok(())
    }
}
