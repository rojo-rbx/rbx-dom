use std::{
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use fs_err::File;

use crate::ModelKind;

#[derive(Debug, Parser)]
pub struct ConvertCommand {
    /// A path to the file to convert.
    input_path: PathBuf,
    /// A path to the desired output for the conversion. The output format is
    /// deteremined by the file extension of this path.
    output_path: PathBuf,
}

impl ConvertCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let input_kind = ModelKind::from_path(&self.input_path)?;
        let output_kind = ModelKind::from_path(&self.output_path)?;

        let input_file = BufReader::new(File::open(&self.input_path)?);

        log::debug!("Reading file into WeakDom");
        let dom = match input_kind {
            ModelKind::Xml => {
                let options = rbx_xml::DecodeOptions::read_unknown();

                rbx_xml::from_reader(input_file, options)
                    .with_context(|| format!("Failed to read {}", self.input_path.display()))?
            }

            ModelKind::Binary => rbx_binary::from_reader(input_file)
                .with_context(|| format!("Failed to read {}", self.input_path.display()))?,
        };

        let root_ids = dom.root().children();

        let output_file = BufWriter::new(File::create(&self.output_path)?);

        log::debug!("Writing into new file at {}", self.output_path.display());
        match output_kind {
            ModelKind::Xml => {
                let options = rbx_xml::EncodeOptions::new()
                    .property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown);

                rbx_xml::to_writer(output_file, &dom, root_ids, options)
                    .with_context(|| format!("Failed to write {}", self.output_path.display()))?;
            }

            ModelKind::Binary => {
                rbx_binary::to_writer(output_file, &dom, root_ids)
                    .with_context(|| format!("Failed to write {}", self.output_path.display()))?;
            }
        }

        Ok(())
    }
}
