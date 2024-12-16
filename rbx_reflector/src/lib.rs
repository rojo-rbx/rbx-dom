use std::path::PathBuf;

use cli::defaults_place::StudioInfo;

mod api_dump;
pub mod cli;
mod defaults;
mod patches;

pub fn defaults_place(api_dump: PathBuf, output: PathBuf) -> anyhow::Result<StudioInfo> {
    cli::defaults_place::DefaultsPlaceSubcommand { api_dump, output }.run()
}

pub fn dump(output: PathBuf) -> anyhow::Result<()> {
    cli::dump::DumpSubcommand { output }.run()
}

pub fn generate(
    patches: Option<PathBuf>,
    output: Vec<PathBuf>,
    no_pretty: bool,
    human_readable: bool,
) -> anyhow::Result<()> {
    cli::generate::GenerateSubcommand {
        patches,
        output,
        no_pretty,
        human_readable,
    }
    .run()
}

pub fn values(output: PathBuf) -> anyhow::Result<()> {
    cli::values::ValuesSubcommand { output }.run()
}
