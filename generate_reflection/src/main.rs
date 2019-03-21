#![recursion_limit="128"]

mod api_dump;
mod database;
mod emitter_lua;
mod emitter_rust;
mod roblox_install;
mod run_in_roblox;
mod reflection_types;

use std::{
    collections::HashMap,
    path::PathBuf,
    error::Error,
};

use serde_derive::Deserialize;
use rbx_dom_weak::{RbxTree, RbxValue, RbxInstanceProperties};

use crate::{
    run_in_roblox::{inject_plugin_main, run_in_roblox},
    api_dump::Dump,
    database::ReflectionDatabase,
};

static PLUGIN_MODEL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/plugin.rbxmx"));

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum PluginMessage {
    Version {
        version: [u32; 4],
    },

    #[serde(rename_all = "camelCase")]
    DefaultProperties {
        class_name: String,
        properties: HashMap<String, RbxValue>,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (dump_source, dump) = Dump::read_with_source()?;

    let plugin = {
        let mut plugin = RbxTree::new(RbxInstanceProperties {
            name: String::from("generate_rbx_reflection plugin"),
            class_name: String::from("Folder"),
            properties: Default::default(),
        });

        let root_id = plugin.get_root_id();

        rbx_xml::decode(&mut plugin, root_id, PLUGIN_MODEL)
            .expect("Couldn't deserialize built-in plugin");

        inject_plugin_main(&mut plugin);
        inject_api_dump(&mut plugin, dump_source);

        plugin
    };

    let messages = run_in_roblox(&plugin);

    let mut default_properties = HashMap::new();
    let mut studio_version = [0, 0, 0, 0];

    for message in &messages {
        if let Ok(str) = std::str::from_utf8(message) {
            println!("{}", str);
        }

        let deserialized = serde_json::from_slice(&message)
            .expect("Couldn't deserialize message");

        match deserialized {
            PluginMessage::Version { version } => {
                studio_version = version;
            }
            PluginMessage::DefaultProperties { class_name, properties } => {
                default_properties.insert(class_name, properties);
            }
        }
    }

    let database = ReflectionDatabase {
        dump,
        default_properties,
        studio_version,
    };

    let rust_output_dir = {
        let mut rust_output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rust_output_dir.pop();
        rust_output_dir.push("rbx_reflection");
        rust_output_dir.push("src");
        rust_output_dir
    };

    let lua_output_dir = {
        let mut lua_output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lua_output_dir.pop();
        lua_output_dir.push("rbx_dom_lua");
        lua_output_dir.push("src");
        lua_output_dir.push("database");
        lua_output_dir
    };

    emitter_rust::emit(&database, &rust_output_dir)?;
    emitter_lua::emit(&database, &lua_output_dir)?;

    Ok(())
}

fn inject_api_dump(plugin: &mut RbxTree, source: String) {
    let root_id = plugin.get_root_id();

    let dump_node = RbxInstanceProperties {
        class_name: String::from("StringValue"),
        name: String::from("ApiDump"),
        properties: {
            let mut properties = HashMap::new();

            properties.insert(
                String::from("Value"),
                RbxValue::String { value: source },
            );

            properties
        },
    };

    plugin.insert_instance(dump_node, root_id);
}