#![recursion_limit="128"]

mod api_dump;
mod property_patches;
mod database;
mod emitter_lua;
mod emitter_rust;
mod reflection_types;
mod roblox_install;
mod run_in_roblox;

use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    mem,
    path::PathBuf,
};

use serde_derive::Deserialize;
use rbx_dom_weak::{RbxTree, RbxValue, RbxInstanceProperties};

use crate::{
    run_in_roblox::{inject_plugin_main, run_in_roblox},
    api_dump::{Dump, DumpClassMember},
    database::ReflectionDatabase,
    property_patches::get_property_patches,
    reflection_types::{RbxInstanceClass, RbxInstanceTags},
};

static PLUGIN_MAIN: &str = include_str!("../plugin/main.lua");

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum PluginMessage {
    Version {
        version: [u32; 4],
    },

    #[serde(rename_all = "camelCase")]
    DefaultProperties {
        class_name: String,
        properties: HashMap<Cow<'static, str>, RbxValue>,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (dump_source, dump) = Dump::read_with_source()?;
    let property_patches = get_property_patches();

    let mut classes = HashMap::new();

    for dump_class in &dump.classes {
        let superclass = if dump_class.superclass == "<<<ROOT>>>" {
            None
        } else {
            Some(Cow::Owned(dump_class.superclass.clone()))
        };

        let tags = RbxInstanceTags::from_dump_tags(&dump_class.tags);

        let mut properties = HashMap::new();

        for member in &dump_class.members {
            match member {
                DumpClassMember::Property(property) => {
                    properties.insert(Cow::Owned(property.name.clone()), property.into());
                }
                _ => {}
            }
        }

        let class = RbxInstanceClass {
            name: Cow::Owned(dump_class.name.clone()),
            superclass,
            tags,
            properties,
            default_properties: HashMap::new(),

            is_canonical: true,
            canonical_name: None,
            serialized_name: None,
        };

        classes.insert(dump_class.name.clone(), class);
    }

    let plugin = {
        let mut plugin = RbxTree::new(RbxInstanceProperties {
            name: String::from("generate_reflection plugin"),
            class_name: String::from("Folder"),
            properties: Default::default(),
        });

        let root_id = plugin.get_root_id();

        let mut main_properties = HashMap::new();
        main_properties.insert(String::from("Source"), RbxValue::String {
            value: PLUGIN_MAIN.to_owned(),
        });

        let main = RbxInstanceProperties {
            name: String::from("Main"),
            class_name: String::from("ModuleScript"),
            properties: main_properties,
        };

        plugin.insert_instance(main, root_id);

        inject_plugin_main(&mut plugin);
        inject_api_dump(&mut plugin, dump_source);

        plugin
    };

    let messages = run_in_roblox(&plugin);

    let mut studio_version = [0, 0, 0, 0];

    for message in &messages {
        let deserialized = serde_json::from_slice(&message)
            .expect("Couldn't deserialize message");

        match deserialized {
            PluginMessage::Version { version } => {
                studio_version = version;
            }
            PluginMessage::DefaultProperties { class_name, properties } => {
                if let Some(class) = classes.get_mut(&class_name) {
                    mem::replace(&mut class.default_properties, properties);
                }
            }
        }
    }

    let database = ReflectionDatabase {
        dump,
        studio_version,

        classes,
    };

    let rust_output_dir = {
        let mut rust_output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        rust_output_dir.pop();
        rust_output_dir.push("rbx_reflection/src/reflection_database");
        rust_output_dir
    };

    let lua_output_dir = {
        let mut lua_output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lua_output_dir.pop();
        lua_output_dir.push("rbx_dom_lua/src/ReflectionDatabase");
        lua_output_dir
    };

    fs::create_dir_all(&rust_output_dir)?;
    fs::create_dir_all(&lua_output_dir)?;

    {
        let classes_path = rust_output_dir.join("classes.rs");
        let mut classes_file = BufWriter::new(File::create(classes_path)?);
        emitter_rust::emit_classes(&mut classes_file, &database)?;
        classes_file.flush()?;
    }

    {
        let enums_path = rust_output_dir.join("enums.rs");
        let mut enums_file = BufWriter::new(File::create(enums_path)?);
        emitter_rust::emit_enums(&mut enums_file, &database)?;
        enums_file.flush()?;
    }

    {
        let version_path = rust_output_dir.join("version.rs");
        let mut version_file = BufWriter::new(File::create(version_path)?);
        emitter_rust::emit_version(&mut version_file, &database)?;
        version_file.flush()?;
    }

    {
        let classes_path = lua_output_dir.join("classes.lua");
        let mut classes_file = BufWriter::new(File::create(classes_path)?);
        emitter_lua::emit_classes(&mut classes_file, &database)?;
        classes_file.flush()?;
    }

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