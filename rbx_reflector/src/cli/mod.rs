mod defaults_place;
mod dump;
mod generate;
mod values;

use clap::Parser;

use self::{
    defaults_place::DefaultsPlaceSubcommand, dump::DumpSubcommand, generate::GenerateSubcommand,
    values::ValuesSubcommand,
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
    DefaultsPlace(DefaultsPlaceSubcommand),
    Generate(GenerateSubcommand),
    Values(ValuesSubcommand),
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Subcommand::Dump(sub) => sub.run(),
            Subcommand::DefaultsPlace(sub) => sub.run().map(|_| ()),
            Subcommand::Generate(sub) => sub.run(),
            Subcommand::Values(sub) => sub.run(),
        }
    }
}
