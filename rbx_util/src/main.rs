mod convert;
mod remove_prop;
mod view_binary;

use std::process;
use std::{path::Path, str::FromStr};

use clap::Parser;

use convert::ConvertCommand;
use remove_prop::RemovePropCommand;
use view_binary::ViewBinaryCommand;

#[derive(Debug, Parser)]
#[clap(name = "rbx_util", about)]
struct Options {
    #[clap(flatten)]
    global: GlobalOptions,
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl Options {
    fn run(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::ViewBinary(command) => command.run(),
            Subcommand::Convert(command) => command.run(),
            Subcommand::RemoveProp(command) => command.run(),
        }
    }
}

#[derive(Debug, Parser)]
enum Subcommand {
    /// Displays a binary file in a text format.
    ViewBinary(ViewBinaryCommand),
    /// Convert between the XML and binary formats for places and models.
    Convert(ConvertCommand),
    /// Removes a specific property from a specific class within a Roblox file.
    RemoveProp(RemovePropCommand),
}

#[derive(Debug, Parser, Clone, Copy)]
struct GlobalOptions {
    /// Sets verbosity level. Can be specified multiple times.
    #[clap(long, short, global(true), action = clap::ArgAction::Count)]
    verbosity: u8,
    /// Set color behavior. Valid values are auto, always, and never.
    #[clap(long, global(true), default_value = "auto")]
    color: ColorChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelKind {
    Binary,
    Xml,
}

impl ModelKind {
    pub fn from_path(path: &Path) -> anyhow::Result<ModelKind> {
        log::trace!("Resolving type of file for path {}", path.display());
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rbxm") | Some("rbxl") => Ok(ModelKind::Binary),
            Some("rbxmx") | Some("rbxlx") => Ok(ModelKind::Xml),

            _ => anyhow::bail!("not a Roblox model or place file: {}", path.display()),
        }
    }
}

fn main() {
    let options = Options::parse();

    let log_filter = match options.global.verbosity {
        0 => "info",
        1 => "info,rbx_binary=debug,rbx_xml=debug,rbx_util=debug",
        2 => "debug,rbx_binary=trace,rbx_xml=trace,rbx_util=trace",
        _ => "trace",
    };

    let log_env = env_logger::Env::default().default_filter_or(log_filter);
    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        .format_indent(Some(8))
        .write_style(options.global.color.into())
        .init();

    if let Err(err) = options.run() {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl FromStr for ColorChoice {
    type Err = anyhow::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source {
            "auto" => Ok(ColorChoice::Auto),
            "always" => Ok(ColorChoice::Always),
            "never" => Ok(ColorChoice::Never),
            _ => anyhow::bail!(
                "Invalid color choice '{source}'. Valid values are: auto, always, never"
            ),
        }
    }
}

impl From<ColorChoice> for clap::ColorChoice {
    fn from(value: ColorChoice) -> Self {
        match value {
            ColorChoice::Auto => clap::ColorChoice::Auto,
            ColorChoice::Always => clap::ColorChoice::Always,
            ColorChoice::Never => clap::ColorChoice::Never,
        }
    }
}

impl From<ColorChoice> for env_logger::WriteStyle {
    fn from(value: ColorChoice) -> Self {
        match value {
            ColorChoice::Auto => env_logger::WriteStyle::Auto,
            ColorChoice::Always => env_logger::WriteStyle::Always,
            ColorChoice::Never => env_logger::WriteStyle::Never,
        }
    }
}
