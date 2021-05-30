//! Collects default property values by generated a place file every kind of
//! instance in it, then uses Roblox Studio to re-save it with default property
//! information encoded in it.

use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    fmt::{self, Write},
    fs::{self, remove_file, File},
    io::BufReader,
    process::Command,
    sync::mpsc,
    time::Duration,
};

use anyhow::Context;
use notify::{DebouncedEvent, Watcher};
use rbx_dom_weak::types::VariantType;
use rbx_dom_weak::WeakDom;
use rbx_reflection::{PropertyDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase};
use roblox_install::{Error, RobloxStudio};
//use tempfile::tempdir;

use crate::plugin_injector::{PluginInjector, StudioInfo};

/// Use Roblox Studio to populate the reflection database with default values
/// for as many properties as possible.
pub fn measure_default_properties(database: &mut ReflectionDatabase) -> anyhow::Result<()> {
    let fixture_place = generate_fixture_place(database);
    let output = roundtrip_place_through_studio(&fixture_place)?;

    database.version = output.info.version;

    log::info!("Applying defaults from place file into reflection database...");
    apply_defaults_from_fixture_place(database, &output.tree);

    Ok(())
}

fn apply_defaults_from_fixture_place(database: &mut ReflectionDatabase, tree: &WeakDom) {
    // Perform a breadth-first search to find the instance shallowest in the
    // tree of each class.

    let mut found_classes = HashSet::new();
    let mut to_visit = VecDeque::new();

    to_visit.extend(tree.root().children());

    while let Some(referent) = to_visit.pop_front() {
        let instance = tree.get_by_ref(referent).unwrap();

        to_visit.extend(instance.children());

        if found_classes.contains(&instance.class) {
            continue;
        }

        found_classes.insert(instance.class.clone());

        for (prop_name, prop_value) in &instance.properties {
            let descriptors = match find_descriptors(database, &instance.class, prop_name) {
                Some(descriptor) => descriptor,
                None => {
                    log::warn!(
                        "Found unknown property {}.{}, which is of type {:?}",
                        instance.class,
                        prop_name,
                        prop_value.ty(),
                    );
                    continue;
                }
            };

            match &descriptors.canonical.kind {
                PropertyKind::Canonical { serialization } => match serialization {
                    PropertySerialization::Serializes => {
                        if &descriptors.canonical.name != prop_name {
                            log::error!("Property {}.{} is supposed to serialize as {}, but was actually serialized as {}",
                                instance.class,
                                descriptors.canonical.name,
                                descriptors.canonical.name,
                                prop_name);
                        }
                    }

                    PropertySerialization::DoesNotSerialize => {
                        log::error!(
                            "Property {}.{} (canonical name {}) found in default place but should not serialize",
                            instance.class,
                            prop_name,
                            descriptors.canonical.name,
                        );
                    }

                    PropertySerialization::SerializesAs(serialized_name) => {
                        if serialized_name != prop_name {
                            log::error!("Property {}.{} is supposed to serialize as {}, but was actually serialized as {}",
                                instance.class,
                                descriptors.canonical.name,
                                serialized_name,
                                prop_name);
                        }
                    }

                    unknown => {
                        log::error!(
                            "Unknown property serialization {:?} on property {}.{}",
                            unknown,
                            instance.class,
                            descriptors.canonical.name
                        );
                    }
                },

                _ => panic!(
                    "find_descriptors must not return a non-canonical descriptor as canonical"
                ),
            }

            let canonical_name = Cow::Owned(descriptors.canonical.name.clone().into_owned());

            match prop_value.ty() {
                // We don't support usefully emitting these types yet.
                VariantType::Ref | VariantType::SharedString => {}

                _ => {
                    let class_descriptor = match database.classes.get_mut(instance.class.as_str()) {
                        Some(descriptor) => descriptor,
                        None => {
                            log::warn!(
                                "Class {} found in default place but not API dump",
                                instance.class
                            );
                            continue;
                        }
                    };

                    class_descriptor
                        .default_properties
                        .insert(canonical_name, prop_value.clone());
                }
            }
        }
    }
}

struct Descriptors<'a> {
    // This descriptor might be useful in the future, but is currently unused.
    #[allow(unused)]
    input: &'a PropertyDescriptor<'a>,

    canonical: &'a PropertyDescriptor<'a>,
}

fn find_descriptors<'a>(
    database: &'a ReflectionDatabase,
    class_name: &str,
    prop_name: &str,
) -> Option<Descriptors<'a>> {
    let mut input_descriptor = None;
    let mut next_class_name = Some(class_name);

    while let Some(current_class_name) = next_class_name {
        let class = database.classes.get(current_class_name).unwrap();

        if let Some(prop) = class.properties.get(prop_name) {
            if input_descriptor.is_none() {
                input_descriptor = Some(prop);
            }

            match &prop.kind {
                PropertyKind::Canonical { .. } => {
                    return Some(Descriptors {
                        input: input_descriptor.unwrap(),
                        canonical: prop,
                    });
                }
                PropertyKind::Alias { alias_for } => {
                    let aliased_prop = class.properties.get(alias_for).unwrap();

                    return Some(Descriptors {
                        input: input_descriptor.unwrap(),
                        canonical: aliased_prop,
                    });
                }
                unknown => {
                    log::warn!("Unknown property kind {:?}", unknown);
                    return None;
                }
            }
        }

        next_class_name = class.superclass.as_ref().map(|name| name.as_ref());
    }

    None
}

struct StudioOutput {
    info: StudioInfo,
    tree: WeakDom,
}

/// Generate a new fixture place from the given reflection database, open it in
/// Studio, coax Studio to re-save it, and read back the resulting place.
fn roundtrip_place_through_studio(place_contents: &str) -> anyhow::Result<StudioOutput> {
    // TODO: Find out where Roblox Studio keeps its settings so we can set the
    // auto-save directory to a temp directory
    let output_dir = dirs::document_dir()
        .ok_or(Error::DocumentsDirectoryNotFound)?
        .join("ROBLOX")
        .join("AutoSaves");

    let autosave_path = output_dir.join("GenerateReflectionFixturePlace_AutoRecovery_0.rbxl");
    let place_path = output_dir.join("GenerateReflectionFixturePlace.rbxlx");

    log::info!("Generating place at {}", place_path.display());
    fs::write(&place_path, place_contents)?;

    let studio_install = RobloxStudio::locate()?;
    let injector = PluginInjector::start(&studio_install);

    log::info!("Starting Roblox Studio...");

    let mut studio_process = Command::new(studio_install.application_path())
        .arg(place_path.display().to_string())
        .spawn()?;

    let info = injector.receive_info();

    let (tx, rx) = mpsc::channel();

    let mut place_watcher = notify::watcher(tx.clone(), Duration::from_millis(300))?;
    place_watcher.watch(&place_path, notify::RecursiveMode::NonRecursive)?;

    let mut autosave_watcher = notify::watcher(tx, Duration::from_millis(300))?;
    autosave_watcher.watch(&output_dir, notify::RecursiveMode::NonRecursive)?;

    log::info!("Waiting for the place to be saved...");
    println!("Please save the open place. Studio will automatically save it after one minute.");

    let file_path = loop {
        match rx.recv_timeout(Duration::from_secs(61))? {
            DebouncedEvent::Write(path) if path == place_path => break path,
            DebouncedEvent::Create(path) if path == autosave_path => break path,
            _ => continue,
        }
    };

    log::info!("Place saved, killing Studio");

    // TODO: We don't have to talk to the plugin again if we can set the settings from here
    injector.finish();

    studio_process.kill()?;

    log::info!("Reading back place file...");

    let file = BufReader::new(File::open(&file_path)?);

    let tree = match rbx_binary::from_reader_default(file) {
        Ok(tree) => tree,
        Err(err) => {
            let _ = fs::copy(autosave_path, "defaults-place.rbxlx");
            return Err(err).context(
                "failed to decode defaults place; it has been copied to defaults-place.rbxlx",
            );
        }
    };

    // TODO: Delete these two lines when we set the auto-save directory
    remove_file(autosave_path)?;
    remove_file(place_path)?;

    Ok(StudioOutput { info, tree })
}

/// Create a place file that contains a copy of every Roblox class and no
/// properties defined.
///
/// When this place is re-saved by Roblox Studio, it'll contain default values
/// for every property.
fn generate_fixture_place(database: &ReflectionDatabase) -> String {
    log::info!("Generating place with every instance...");

    let mut output = String::new();

    writeln!(&mut output, "<roblox version=\"4\">").unwrap();

    for descriptor in database.classes.values() {
        let mut instance = FixtureInstance::named(&descriptor.name);

        match &*descriptor.name {
            // These types can't be put into place files by default.
            "DebuggerWatch" | "DebuggerBreakpoint" | "AdvancedDragger" | "Dragger"
            | "ScriptDebugger" | "PackageLink" => continue,

            // These types have specific parenting restrictions handled
            // elsewhere.
            "Terrain"
            | "Attachment"
            | "Animator"
            | "StarterPlayerScripts"
            | "StarterCharacterScripts"
            | "Bone"
            | "BaseWrap"
            | "WrapLayer"
            | "WrapTarget" => continue,

            // WorldModel is not yet enabled.
            "WorldModel" => continue,

            "StarterPlayer" => {
                instance.add_child(FixtureInstance::named("StarterPlayerScripts"));
                instance.add_child(FixtureInstance::named("StarterCharacterScripts"));
            }
            "Workspace" => {
                instance.add_child(FixtureInstance::named("Terrain"));
            }
            "Part" => {
                instance.add_child(FixtureInstance::named("Attachment"));
                instance.add_child(FixtureInstance::named("Bone"));
            }
            "Humanoid" => {
                instance.add_child(FixtureInstance::named("Animator"));
            }
            "MeshPart" => {
                // Without this special case, Studio will fail to open the
                // resulting file, complaining about "BaseWrap".
                instance.add_child(FixtureInstance::named("BaseWrap"));
                instance.add_child(FixtureInstance::named("WrapLayer"));
                instance.add_child(FixtureInstance::named("WrapTarget"));
            }
            _ => {}
        }

        write!(output, "{}", instance).unwrap();
    }

    writeln!(&mut output, "</roblox>").unwrap();
    output
}

struct FixtureInstance<'a> {
    name: &'a str,
    children: Vec<FixtureInstance<'a>>,
}

impl<'a> FixtureInstance<'a> {
    fn named(name: &'a str) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: FixtureInstance<'a>) {
        self.children.push(child);
    }
}

impl fmt::Display for FixtureInstance<'_> {
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
