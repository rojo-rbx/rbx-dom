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
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::{self, Write as _},
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    path::PathBuf,
    process::Command,
    str,
    sync::mpsc,
    thread,
    time::Duration,
};

use notify::{DebouncedEvent, Watcher};
use rbx_dom_weak::{RbxInstanceProperties, RbxTree, RbxValue, RbxValueType};
use roblox_install::RobloxStudio;
use serde_derive::Deserialize;
use tempfile::tempdir;

use crate::{
    api_dump::Dump,
    database::ReflectionDatabase,
    property_patches::load_property_patches,
    reflection_types::RbxPropertyScriptability,
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
    PatchDescriptors {
        class_name: String,
        descriptors: HashMap<Cow<'static, str>, DescriptorPatch>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DescriptorPatch {
    default_value: Option<RbxValue>,
    scriptability: Option<RbxPropertyScriptability>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut database = ReflectionDatabase::new();

    let dump = Dump::read()?;
    database.populate_from_dump(&dump)?;

    let tree = roundtrip_place_through_studio(&database)?;
    collect_defaults_from_place(&mut database, &tree);

    let property_patches = load_property_patches();
    database.populate_from_patches(&property_patches)?;

    // let plugin = create_plugin(&database);
    // let messages = run_in_roblox(&plugin);
    // process_plugin_messages(&mut database, &messages);

    database.validate();

    emit_source(&database, &dump)?;

    Ok(())
}

fn collect_defaults_from_place(database: &mut ReflectionDatabase, tree: &RbxTree) {
    // Perform a breadth-first search to find the instance shallowest in the
    // tree of each class.

    let mut found_classes = HashSet::new();
    let mut to_visit = VecDeque::new();

    let root_instance = tree.get_instance(tree.get_root_id()).unwrap();
    to_visit.extend(root_instance.get_children_ids());

    while let Some(id) = to_visit.pop_front() {
        let instance = tree.get_instance(id).unwrap();

        to_visit.extend(instance.get_children_ids());

        if found_classes.contains(&instance.class_name) {
            continue;
        }

        found_classes.insert(instance.class_name.clone());

        let descriptor = match database.classes.get_mut(instance.class_name.as_str()) {
            Some(descriptor) => descriptor,
            None => {
                log::warn!(
                    "Class {} found in default place but not API dump!",
                    instance.class_name
                );
                continue;
            }
        };

        for (prop_name, prop_value) in &instance.properties {
            match prop_value.get_type() {
                // We don't support emitting SharedString values yet.
                RbxValueType::SharedString => {}

                _ => {
                    descriptor
                        .default_properties
                        .insert(Cow::Owned(prop_name.clone()), prop_value.clone());
                }
            }
        }
    }
}

fn roundtrip_place_through_studio(
    database: &ReflectionDatabase,
) -> Result<RbxTree, Box<dyn Error>> {
    let fixture = generate_fixture_place(&database);

    let output_dir = tempdir()?;
    let output_path = output_dir.path().join("roundtrip.rbxlx");
    fs::write(&output_path, fixture)?;

    println!("Generating place at {}", output_path.display());

    let studio_install = RobloxStudio::locate()?;

    let mut studio_process = Command::new(studio_install.application_path())
        .arg(output_path.display().to_string())
        .spawn()?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::watcher(tx, Duration::from_millis(300))?;
    watcher.watch(&output_path, notify::RecursiveMode::NonRecursive)?;

    println!("Waiting to resave place...");

    loop {
        match rx.recv()? {
            DebouncedEvent::Write(_) => break,
            _ => {}
        }
    }

    println!("Got it!");

    studio_process.kill()?;

    let mut file = BufReader::new(File::open(output_path)?);

    let decode_options = rbx_xml::DecodeOptions::new()
        .property_behavior(rbx_xml::DecodePropertyBehavior::NoReflection);
    let tree = rbx_xml::from_reader(&mut file, decode_options)?;

    Ok(tree)
}

fn generate_fixture_place(database: &ReflectionDatabase) -> String {
    struct Instance<'a> {
        name: &'a str,
        children: Vec<Instance<'a>>,
    }

    impl<'a> Instance<'a> {
        fn named(name: &'a str) -> Self {
            Self {
                name,
                children: Vec::new(),
            }
        }
    }

    impl fmt::Display for Instance<'_> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            writeln!(
                formatter,
                "<Item class=\"{}\" reference=\"{}\">",
                &self.name, &self.name
            )?;

            for child in &self.children {
                write!(formatter, "{}", child)?;
            }

            writeln!(formatter, "</Item>")?;

            Ok(())
        }
    }

    let mut output = String::new();

    writeln!(&mut output, "<roblox version=\"4\">").unwrap();

    for descriptor in database.classes.values() {
        let mut instance = Instance::named(&descriptor.name);

        match &*descriptor.name {
            // These types can't be put into place files by default.
            "DebuggerWatch" | "DebuggerBreakpoint" | "AdvancedDragger" | "Dragger"
            | "ScriptDebugger" | "PackageLink" => continue,

            // rbx_xml does not currently support Ray values.
            // https://github.com/rojo-rbx/rbx-dom/issues/87
            "RayValue" => continue,

            // rbx_xml does not currently support Faces values.
            // https://github.com/rojo-rbx/rbx-dom/issues/88
            "Handles" => continue,

            // rbx_xml does not currently support Axes values.
            // https://github.com/rojo-rbx/rbx-dom/issues/89
            "ArcHandles" => continue,

            // These types have specific parenting restrictions handled
            // elsewhere.
            "Terrain"
            | "Attachment"
            | "Animator"
            | "StarterPlayerScripts"
            | "StarterCharacterScripts" => continue,

            // WorldModel is not yet enabled.
            "WorldModel" => continue,

            "StarterPlayer" => {
                instance
                    .children
                    .push(Instance::named("StarterPlayerScripts"));
                instance
                    .children
                    .push(Instance::named("StarterCharacterScripts"));
            }
            "Workspace" => {
                instance.children.push(Instance::named("Terrain"));
            }
            "Part" => {
                instance.children.push(Instance::named("Attachment"));
            }
            "Humanoid" => {
                instance.children.push(Instance::named("Animator"));
            }
            _ => {}
        }

        write!(output, "{}", instance).unwrap();
    }

    writeln!(&mut output, "</roblox>").unwrap();
    output
}

fn emit_source(database: &ReflectionDatabase, dump: &Dump) -> io::Result<()> {
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
        emitter_rust::emit_enums(&mut enums_file, &dump)?;
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

fn process_plugin_messages(database: &mut ReflectionDatabase, messages: &[Vec<u8>]) {
    for message in messages {
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
            PluginMessage::PatchDescriptors {
                class_name,
                descriptors,
            } => {
                if let Some(class) = database.classes.get_mut(class_name.as_str()) {
                    for (property_name, patch) in descriptors {
                        if let Some(default_value) = patch.default_value {
                            class
                                .default_properties
                                .insert(property_name.clone(), default_value.clone());
                        }

                        if let Some(descriptor) = class.properties.get_mut(&property_name) {
                            if let Some(scriptability) = patch.scriptability {
                                descriptor.scriptability = scriptability;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn create_plugin(database: &ReflectionDatabase) -> RbxTree {
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
