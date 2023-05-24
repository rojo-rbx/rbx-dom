use anyhow::{bail, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    version: String,
}

pub fn get_studio_version() -> anyhow::Result<[u32; 4]> {
    let response: Response =
        reqwest::blocking::get("https://clientsettings.roblox.com/v2/client-version/WindowsStudio")
            .context("Failed to retrieve Studio version")?
            .json()?;

    let result = response
        .version
        .split('.')
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>()
        .try_into();

    if let Ok(version) = result {
        Ok(version)
    } else {
        bail!("Invalid Studio version: {}", response.version)
    }
}
