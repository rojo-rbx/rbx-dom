use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{anyhow, bail, Context};
use fs_err::File;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    /// Convert a model or place file in one format to another.
    Convert { input: PathBuf, output: PathBuf },

    /// View a binary file as an undefined text representation.
    ViewBinary { input: PathBuf },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelKind {
    Binary,
    Xml,
}

impl ModelKind {
    fn from_path(path: &Path) -> anyhow::Result<ModelKind> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rbxm") | Some("rbxl") => Ok(ModelKind::Binary),
            Some("rbxmx") | Some("rbxlx") => Ok(ModelKind::Xml),

            _ => Err(anyhow!(
                "not a Roblox model or place file: {}",
                path.display()
            )),
        }
    }
}

fn run(options: Options) -> anyhow::Result<()> {
    match options.subcommand {
        Subcommand::Convert { input, output } => convert(&input, &output)?,
        Subcommand::ViewBinary { input } => view_binary(&input)?,
    }

    Ok(())
}

fn convert(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    let input_kind = ModelKind::from_path(input_path)?;
    let output_kind = ModelKind::from_path(output_path)?;

    let input_file = BufReader::new(File::open(input_path)?);

    let dom = match input_kind {
        ModelKind::Xml => {
            let options = rbx_xml::DecodeOptions::new()
                .property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown);

            rbx_xml::from_reader(input_file, options)
                .with_context(|| format!("Failed to read {}", input_path.display()))?
        }

        ModelKind::Binary => rbx_binary::from_reader(input_file)
            .with_context(|| format!("Failed to read {}", input_path.display()))?,
    };

    let root_ids = dom.root().children();

    let output_file = BufWriter::new(File::create(output_path)?);

    match output_kind {
        ModelKind::Xml => {
            let options = rbx_xml::EncodeOptions::new()
                .property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown);

            rbx_xml::to_writer(output_file, &dom, root_ids, options)
                .with_context(|| format!("Failed to write {}", output_path.display()))?;
        }

        ModelKind::Binary => {
            rbx_binary::to_writer(output_file, &dom, root_ids)
                .with_context(|| format!("Failed to write {}", output_path.display()))?;
        }
    }

    Ok(())
}

fn view_binary(input_path: &Path) -> anyhow::Result<()> {
    let input_kind = ModelKind::from_path(input_path)?;

    if input_kind != ModelKind::Binary {
        bail!("not a binary model or place file: {}", input_path.display());
    }

    let input_file = BufReader::new(File::open(input_path)?);

    let model = rbx_binary::text_format::DecodedModel::from_reader(input_file);

    let stdout = io::stdout();
    let output = BufWriter::new(stdout.lock());
    serde_yaml::to_writer(output, &model)?;

    Ok(())
}

fn main() {
    let options = Options::from_args();

    if let Err(err) = run(options) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
