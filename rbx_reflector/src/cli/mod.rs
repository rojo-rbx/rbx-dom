mod dump;

use clap::Parser;

use self::dump::DumpSubcommand;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    Dump(DumpSubcommand),
    Patch,
    DefaultsPlace,
    ComputeDefaults,
    Generate,
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Subcommand::Dump(sub) => sub.run(),
            Subcommand::Patch => todo!(),
            Subcommand::DefaultsPlace => todo!(),
            Subcommand::ComputeDefaults => todo!(),
            Subcommand::Generate => todo!(),
        }
    }
}
