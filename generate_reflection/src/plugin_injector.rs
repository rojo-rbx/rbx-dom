//! Defines an interface to inject a plugin into a local Roblox Studio install
//! that can communicate with a temporary HTTP server.

use std::fs::{remove_file, File};
use std::io::BufWriter;
use std::time::Duration;

use rbx_dom_weak::{InstanceBuilder, WeakDom};
use roblox_install::RobloxStudio;
use serde::Deserialize;
use tiny_http::Response;

static PLUGIN_SOURCE: &str = include_str!("../plugin/main.lua");

#[derive(Debug, Deserialize)]
pub struct StudioInfo {
    pub version: [u32; 4],
}

pub struct PluginInjector<'a> {
    http_server: tiny_http::Server,
    roblox_studio: &'a RobloxStudio,
}

impl<'a> PluginInjector<'a> {
    pub fn start(roblox_studio: &'a RobloxStudio) -> Self {
        log::info!("Starting HTTP server to receive Studio metadata");
        let http_server = tiny_http::Server::http("0.0.0.0:22073").unwrap();

        log::info!("Installing Studio Plugin");
        install_plugin(roblox_studio);

        PluginInjector {
            http_server,
            roblox_studio,
        }
    }

    pub fn receive_info(self) -> StudioInfo {
        log::info!("Waiting to hear back from Studio plugin...");
        let mut request = self
            .http_server
            .recv_timeout(Duration::from_secs(30))
            .expect("error receiving HTTP request")
            .expect("plugin did not send a request within 30 seconds");

        let studio_info: StudioInfo = serde_json::from_reader(request.as_reader()).unwrap();
        request.respond(Response::empty(200)).unwrap();

        studio_info
    }
}

impl<'a> Drop for PluginInjector<'a> {
    fn drop(&mut self) {
        log::info!("Uninstalling Studio Plugin");
        remove_plugin(self.roblox_studio);
    }
}

fn install_plugin(roblox_studio: &RobloxStudio) {
    let plugin = create_plugin();

    let plugin_path = roblox_studio
        .plugins_path()
        .join("RbxDomGenerateReflectionPlugin.rbxmx");

    // trying to write to plugin_path fails if plugins_path() doesn't already exist
    fs_err::create_dir_all(roblox_studio.plugins_path()).expect("Couldn't create plugins path");

    let output = BufWriter::new(File::create(plugin_path).unwrap());
    rbx_xml::to_writer_default(output, &plugin, &[plugin.root_ref()]).unwrap();
}

fn remove_plugin(roblox_studio: &RobloxStudio) {
    let plugin_path = roblox_studio
        .plugins_path()
        .join("RbxDomGenerateReflectionPlugin.rbxmx");

    remove_file(plugin_path).unwrap();
}

fn create_plugin() -> WeakDom {
    WeakDom::new(
        InstanceBuilder::new("Script")
            .with_name("RbxDomGenerateReflectionPlugin")
            .with_property("Source", PLUGIN_SOURCE),
    )
}
