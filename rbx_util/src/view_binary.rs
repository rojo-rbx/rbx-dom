use std::{
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use clap::Parser;
use fs_err::File;

use crate::ModelKind;

#[derive(Debug, Parser)]
pub struct ViewBinaryCommand {
    /// The file to emit the contents of.
    input: PathBuf,
}

impl ViewBinaryCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let input_kind = ModelKind::from_path(&self.input)?;

        if input_kind != ModelKind::Binary {
            anyhow::bail!("not a binary model or place file: {}", self.input.display());
        }

        let input_file = BufReader::new(File::open(&self.input)?);

        log::debug!("Decoding file into text format");
        let model = rbx_binary::text_format::DecodedModel::from_reader(input_file);

        log::debug!("Writing to stdout");
        let stdout = io::stdout();
        let output = BufWriter::new(stdout.lock());
        serde_yaml::to_writer(output, &model)?;

        Ok(())
    }
}
