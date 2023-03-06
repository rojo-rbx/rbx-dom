use std::path::PathBuf;
use std::process::Command;

use anyhow::{bail, Context};
use clap::Parser;
use roblox_install::RobloxStudio;

/// Generate an API dump from the system's Roblox Studio installation.
#[derive(Debug, Parser)]
pub struct DumpSubcommand {
    /// Where to output the API dump. The extension must be JSON (.json)
    pub output: PathBuf,
}

impl DumpSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        if self.output.extension().unwrap_or_default() != "json" {
            bail!("The output path must have a .json extension")
        }

        let studio_install =
            RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

        Command::new(studio_install.application_path())
            .arg("-FullAPI")
            .arg(&self.output)
            .status()?;

        Ok(())
    }
}
