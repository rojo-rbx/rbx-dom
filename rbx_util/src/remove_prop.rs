use std::{
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::Context as _;
use clap::Parser;
use fs_err::File;

use crate::ModelKind;

#[derive(Debug, Parser)]
pub struct RemovePropCommand {
    /// The file to remove the property from.
    input: PathBuf,
    #[clap(long, short)]
    /// The place to write the stripped file to.
    output: PathBuf,
    /// The class name to remove the property from.
    class_name: String,
    /// The property to remove from the provided class.
    prop_name: String,
}

impl RemovePropCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let input_kind = ModelKind::from_path(&self.input)?;
        let output_kind = ModelKind::from_path(&self.output)?;

        let input_file = BufReader::new(File::open(&self.input)?);

        log::debug!("Reading from {input_kind:?} file {}", self.input.display());
        let mut dom = match input_kind {
            ModelKind::Xml => {
                let options = rbx_xml::DecodeOptions::new()
                    .property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown);

                rbx_xml::from_reader(input_file, options)
                    .with_context(|| format!("Failed to read {}", self.input.display()))?
            }

            ModelKind::Binary => rbx_binary::from_reader(input_file)
                .with_context(|| format!("Failed to read {}", self.input.display()))?,
        };

        let mut queue = vec![dom.root_ref()];
        while let Some(referent) = queue.pop() {
            let inst = dom.get_by_ref_mut(referent).unwrap();
            if inst.class == self.class_name {
                log::trace!("Removed property {}.{}", inst.name, self.prop_name);
                inst.properties.remove(&self.prop_name);
            }
            queue.extend_from_slice(inst.children());
        }

        let output_file = BufWriter::new(File::create(&self.output)?);

        let root_ids = dom.root().children();
        match output_kind {
            ModelKind::Xml => {
                let options = rbx_xml::EncodeOptions::new()
                    .property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown);

                rbx_xml::to_writer(output_file, &dom, root_ids, options)
                    .with_context(|| format!("Failed to write {}", self.output.display()))?;
            }

            ModelKind::Binary => {
                rbx_binary::to_writer(output_file, &dom, root_ids)
                    .with_context(|| format!("Failed to write {}", self.output.display()))?;
            }
        }
        log::info!(
            "Wrote stripped {output_kind:?} file to {}",
            self.output.display()
        );

        Ok(())
    }
}
