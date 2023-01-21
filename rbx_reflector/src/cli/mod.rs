mod defaults_place;
mod dump;
mod generate;

use clap::Parser;

use self::{
    defaults_place::DefaultsPlaceSubcommand, dump::DumpSubcommand, generate::GenerateSubcommand,
};

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
    DefaultsPlace(DefaultsPlaceSubcommand),
    ComputeDefaults,
    Generate(GenerateSubcommand),
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Subcommand::Dump(sub) => sub.run(),
            Subcommand::Patch => todo!(),
            Subcommand::DefaultsPlace(sub) => sub.run(),
            Subcommand::ComputeDefaults => todo!(),
            Subcommand::Generate(sub) => sub.run(),
        }
    }
}
