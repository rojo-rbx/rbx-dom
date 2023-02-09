use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
    process::{Child, Command},
    time::Duration,
};

use anyhow::{bail, Context};
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use rbx_reflection::ReflectionDatabase;
use roblox_install::RobloxStudio;
use tiny_http::Response;

static PLUGIN_SOURCE: &str = include_str!("../plugin/main.lua");

pub struct PluginInjector {
    http_server: tiny_http::Server,
    roblox_studio: RobloxStudio,
    studio_process: Child,
}

impl PluginInjector {
    pub fn start(
        reflection_database: &ReflectionDatabase,
        defaults_place: &PathBuf,
    ) -> anyhow::Result<Self> {
        let http_server = tiny_http::Server::http("0.0.0.0:22073").unwrap();

        let roblox_studio =
            RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

        install_plugin(reflection_database, &roblox_studio)?;

        let studio_process = Command::new(roblox_studio.application_path())
            .arg(defaults_place)
            .spawn()?;

        Ok(PluginInjector {
            http_server,
            roblox_studio,
            studio_process,
        })
    }

    pub fn receive_version(self) -> anyhow::Result<[u32; 4]> {
        let mut request = match self.http_server.recv_timeout(Duration::from_secs(30))? {
            Some(request) => request,
            None => bail!("plugin did not send a request within 30 seconds"),
        };

        let version = serde_json::from_reader(request.as_reader())
            .context("Plugin sent an invalid studio version")?;

        request.respond(Response::empty(200))?;

        Ok(version)
    }
}

impl Drop for PluginInjector {
    fn drop(&mut self) {
        self.studio_process.kill().unwrap();
        remove_plugin(&self.roblox_studio);
    }
}

fn create_plugin() -> WeakDom {
    WeakDom::new(
        InstanceBuilder::new("Script")
            .with_name("RbxDomGenerateReflectionPlugin")
            .with_property("Source", PLUGIN_SOURCE),
    )
}

fn install_plugin(
    reflection_database: &ReflectionDatabase,
    roblox_studio: &RobloxStudio,
) -> anyhow::Result<()> {
    // Make sure the plugins folder exist before creating the plugin file.
    fs_err::create_dir_all(roblox_studio.plugins_path())
        .context("Couldn't create plugins folder")?;

    let plugin_path = roblox_studio
        .plugins_path()
        .join("RbxDomGenerateReflectionPlugin.rbxmx");
    let output = BufWriter::new(File::create(plugin_path)?);

    let plugin = create_plugin();

    rbx_xml::to_writer_default(output, &plugin, &[plugin.root_ref()], reflection_database)?;

    Ok(())
}

fn remove_plugin(roblox_studio: &RobloxStudio) {
    let plugin_path = roblox_studio
        .plugins_path()
        .join("RbxDomGenerateReflectionPlugin.rbxmx");

    fs::remove_file(plugin_path).unwrap();
}
