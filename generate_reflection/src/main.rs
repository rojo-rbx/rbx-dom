// Our use of quote! hits the recursion limit, oops.
#![recursion_limit = "128"]

mod api_dump;
mod database;
mod emitter_lua;
mod emitter_rust;
mod property_patches;
mod reflection_types;
mod run_in_roblox;

use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    mem,
    path::PathBuf,
    str,
};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree, RbxValue};
use serde_derive::Deserialize;

use crate::{
    api_dump::{Dump, DumpClassMember},
    database::ReflectionDatabase,
    property_patches::load_property_patches,
    reflection_types::{
        RbxClassDescriptor, RbxInstanceTags, RbxPropertyDescriptor, RbxPropertyScriptability,
        RbxPropertyTags, RbxPropertyTypeDescriptor,
    },
    run_in_roblox::{inject_plugin_main, run_in_roblox},
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
    },
}

#[allow(
    clippy::useless_let_if_seq, // https://github.com/rust-lang/rust-clippy/issues/3769
    clippy::cyclomatic_complexity, // TODO
)]
fn main() -> Result<(), Box<dyn Error>> {
    let dump = Dump::read()?;

    let mut classes: HashMap<Cow<'static, str>, RbxClassDescriptor> = HashMap::new();

    for dump_class in &dump.classes {
        let superclass = if dump_class.superclass == "<<<ROOT>>>" {
            None
        } else {
            Some(Cow::Owned(dump_class.superclass.clone()))
        };

        let tags = RbxInstanceTags::from_dump_tags(&dump_class.tags);

        let mut properties = HashMap::new();

        for member in &dump_class.members {
            if let DumpClassMember::Property(dump_property) = member {
                let tags = RbxPropertyTags::from_dump_tags(&dump_property.tags);

                let scriptability = if tags.contains(RbxPropertyTags::NOT_SCRIPTABLE) {
                    RbxPropertyScriptability::None
                } else if tags.contains(RbxPropertyTags::READ_ONLY) {
                    RbxPropertyScriptability::Read
                } else {
                    RbxPropertyScriptability::ReadWrite
                };

                let serializes = !dump_property.tags.iter().any(|v| v == "ReadOnly")
                    && dump_property.serialization.can_save;

                let property = RbxPropertyDescriptor {
                    name: Cow::Owned(dump_property.name.clone()),
                    value_type: RbxPropertyTypeDescriptor::from(&dump_property.value_type),
                    tags,

                    is_canonical: true,
                    canonical_name: None,
                    serialized_name: None,
                    scriptability,
                    serializes,
                };

                properties.insert(Cow::Owned(dump_property.name.clone()), property);
            }
        }

        let class = RbxClassDescriptor {
            name: Cow::Owned(dump_class.name.clone()),
            superclass,
            tags,
            properties,
            default_properties: HashMap::new(),
        };

        classes.insert(Cow::Owned(dump_class.name.clone()), class);
    }

    let property_patches = load_property_patches();

    for (class_name, class_changes) in &property_patches.change {
        let class = classes
            .get_mut(class_name.as_str())
            .unwrap_or_else(|| panic!("Class {} defined in patch file wasn't present", class_name));

        for (property_name, property_change) in class_changes {
            let existing_property = class
                .properties
                .get_mut(property_name.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "Property {}.{} did not exist in dump",
                        class_name, property_name
                    )
                });

            println!("{}.{} changed", class_name, property_name);

            existing_property.is_canonical = property_change.canonical_name.is_none();

            if let Some(canonical_name) = &property_change.canonical_name {
                existing_property.canonical_name = Some(canonical_name.clone());
                existing_property.serializes = false;
            }

            if let Some(serialized_name) = &property_change.serialized_name {
                existing_property.serialized_name = Some(serialized_name.clone());
                existing_property.serializes = true;
            }
        }
    }

    for (class_name, class_adds) in &property_patches.add {
        let class = classes
            .get_mut(class_name.as_str())
            .unwrap_or_else(|| panic!("Class {} defined in patch file wasn't present", class_name));

        for (property_name, property_add) in class_adds {
            if class.properties.contains_key(property_name.as_str()) {
                panic!(
                    "Property {}.{} marked for addition in patch was already present",
                    class_name, property_name
                );
            }

            println!("{}.{} added", class_name, property_name);

            let name = Cow::Owned(property_name.clone());
            let value_type = property_add.property_type.clone();
            let is_canonical = property_add.canonical_name.is_none();
            let canonical_name = property_add.canonical_name.clone();
            let serialized_name = property_add.serialized_name.clone();
            let scriptability = property_add.scriptability;
            let serializes = property_add.serializes;

            let property = RbxPropertyDescriptor {
                name,
                value_type,
                is_canonical,
                canonical_name,
                serialized_name,
                scriptability,
                serializes,

                tags: RbxPropertyTags::empty(),
            };

            class
                .properties
                .insert(Cow::Owned(property_name.clone()), property);
        }
    }

    for (class_name, class) in &classes {
        for (property_name, property) in &class.properties {
            if let Some(canonical_name) = &property.canonical_name {
                let canonical_property = match class.properties.get(canonical_name) {
                    Some(value) => value,
                    None => panic!(
                        "Property {}.{} refers to canonical property ({}) that does not exist.",
                        class_name, property_name, canonical_name
                    ),
                };

                if !canonical_property.is_canonical {
                    panic!("Property {}.{} is marked as the canonical form of {}, but is not canonical!",
                        class_name, canonical_name, property_name);
                }
            }

            if let Some(serialized_name) = &property.serialized_name {
                let _serialized_property = match class.properties.get(serialized_name) {
                    Some(value) => value,
                    None => panic!(
                        "Property {}.{} refers to serialized property ({}) that does not exist.",
                        class_name, property_name, serialized_name
                    ),
                };
            }

            if property.is_canonical {
                let mut probably_mistake = false;

                if property_name.chars().next().unwrap().is_lowercase() {
                    probably_mistake = true;
                }

                if property_name.ends_with("_xml") {
                    probably_mistake = true;
                }

                if probably_mistake {
                    println!(
                        "Property {}.{} doesn't look canonical",
                        class_name, property_name
                    );
                }
            }
        }
    }

    let mut database = ReflectionDatabase {
        dump,
        studio_version: [0, 0, 0, 0],
        classes,
    };

    let plugin = {
        let mut plugin = RbxTree::new(RbxInstanceProperties {
            name: String::from("generate_reflection plugin"),
            class_name: String::from("Folder"),
            properties: Default::default(),
        });

        let root_id = plugin.get_root_id();

        let mut main_properties = HashMap::new();
        main_properties.insert(
            String::from("Source"),
            RbxValue::String {
                value: PLUGIN_MAIN.to_owned(),
            },
        );

        let main = RbxInstanceProperties {
            name: String::from("Main"),
            class_name: String::from("ModuleScript"),
            properties: main_properties,
        };

        plugin.insert_instance(main, root_id);

        inject_plugin_main(&mut plugin);
        inject_reflection_classes(&mut plugin, &database);
        inject_dependencies(&mut plugin);

        plugin
    };

    let messages = run_in_roblox(&plugin);

    for message in &messages {
        let deserialized = match serde_json::from_slice(&message) {
            Ok(v) => v,
            Err(e) => {
                panic!(
                    "Couldn't deserialize message: {}\n{}",
                    e,
                    str::from_utf8(message).unwrap()
                );
            }
        };

        match deserialized {
            PluginMessage::Version { version } => {
                database.studio_version = version;
            }
            PluginMessage::DefaultProperties {
                class_name,
                properties,
            } => {
                if let Some(class) = database.classes.get_mut(class_name.as_str()) {
                    mem::replace(&mut class.default_properties, properties);
                }
            }
        }
    }

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

fn create_module(name: &str, source: String) -> RbxInstanceProperties {
    let mut properties = HashMap::new();

    properties.insert(String::from("Source"), RbxValue::String { value: source });

    RbxInstanceProperties {
        class_name: String::from("ModuleScript"),
        name: String::from(name),
        properties,
    }
}

fn inject_reflection_classes(plugin: &mut RbxTree, database: &ReflectionDatabase) {
    let root_id = plugin.get_root_id();

    let mut classes_buffer = Vec::new();
    emitter_lua::emit_classes(&mut classes_buffer, database)
        .expect("Couldn't emit Lua class database");

    let classes_source =
        String::from_utf8(classes_buffer).expect("Lua class database wasn't valid UTF-8");

    let module = create_module("ReflectionClasses", classes_source);
    plugin.insert_instance(module, root_id);
}

/// Injects in the pieces of rbx_dom_lua that we need to generate the dump.
///
/// Notably, this reduces the code duplication for serializing/deserializing
/// values through JSON. We manually track dependencies right now to avoid
/// needing to depend on Rojo to build the plugin.
fn inject_dependencies(plugin: &mut RbxTree) {
    static BASE64: &str = include_str!("../../rbx_dom_lua/src/base64.lua");
    static ENCODED_VALUE: &str = include_str!("../../rbx_dom_lua/src/EncodedValue.lua");

    let root_id = plugin.get_root_id();

    plugin.insert_instance(create_module("base64", String::from(BASE64)), root_id);
    plugin.insert_instance(
        create_module("EncodedValue", String::from(ENCODED_VALUE)),
        root_id,
    );
}
