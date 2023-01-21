use std::{
    fmt::{self, Write},
    fs::{self},
    path::PathBuf,
    process::Command,
    sync::mpsc,
};

use anyhow::{bail, Context};
use clap::Parser;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use roblox_install::RobloxStudio;

use crate::api_dump::Dump;

/// Generate a place file with all classes and their default properties.
#[derive(Debug, Parser)]
pub struct DefaultsPlaceSubcommand {
    /// The path of an API dump that came from the dump command.
    pub api_dump: PathBuf,
    /// Where to output the place. The extension must be .rbxlx
    pub output: PathBuf,
}

impl DefaultsPlaceSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        if self.output.extension().unwrap_or_default() != "rbxlx" {
            bail!("The output path must have a .rbxlx extension")
        }

        let contents = fs::read_to_string(&self.api_dump).context("Could not read API dump")?;
        let dump = serde_json::from_str(&contents).context("Invalid API dump")?;

        generate_place_with_all_classes(&self.output, &dump)?;
        save_place_in_studio(&self.output)?;

        Ok(())
    }
}

fn save_place_in_studio(path: &PathBuf) -> anyhow::Result<()> {
    let studio_install =
        RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

    println!("Starting Roblox Studio...");

    let mut studio_process = Command::new(studio_install.application_path())
        .arg(&path)
        .spawn()?;

    println!("Please save the opened place in Roblox Studio (ctrl+s).");

    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    loop {
        if rx.recv()??.kind.is_create() {
            break;
        }
    }

    println!("Place saved, killing Studio...");

    // TODO: This will cause studio to leave behind a .rbxlx.lock file.
    studio_process.kill()?;

    Ok(())
}

fn generate_place_with_all_classes(path: &PathBuf, dump: &Dump) -> anyhow::Result<()> {
    let mut place_contents = String::new();

    writeln!(place_contents, "<roblox version=\"4\">").unwrap();

    for class in &dump.classes {
        let mut instance = Instance::new(&class.name);

        match &*class.name {
            // These classes can't be put into place files by default.
            "DebuggerWatch" | "DebuggerBreakpoint" | "AdvancedDragger" | "Dragger"
            | "ScriptDebugger" | "PackageLink" | "Ad" | "AdPortal" | "AdGui" => continue,

            // This class will cause studio to crash on close.
            "VoiceSource" => continue,

            // These classes have specific parenting restrictions handled elsewhere.
            "Terrain"
            | "Attachment"
            | "Animator"
            | "StarterPlayerScripts"
            | "StarterCharacterScripts"
            | "Bone"
            | "BaseWrap"
            | "WrapLayer"
            | "WrapTarget" => continue,

            "StarterPlayer" => {
                instance.add_child(Instance::new("StarterPlayerScripts"));
                instance.add_child(Instance::new("StarterCharacterScripts"));
            }
            "Workspace" => {
                instance.add_child(Instance::new("Terrain"));
            }
            "Part" => {
                instance.add_child(Instance::new("Attachment"));
                instance.add_child(Instance::new("Bone"));
            }
            "Humanoid" => {
                instance.add_child(Instance::new("Animator"));
            }
            "MeshPart" => {
                // Without this special case, Studio will fail to open the resulting file, complaining about "BaseWrap".
                instance.add_child(Instance::new("BaseWrap"));
                instance.add_child(Instance::new("WrapLayer"));
                instance.add_child(Instance::new("WrapTarget"));
            }

            _ => {}
        }

        write!(place_contents, "{}", instance).unwrap();
    }

    writeln!(place_contents, "</roblox>").unwrap();

    fs::write(path, place_contents)?;

    Ok(())
}

struct Instance<'a> {
    class_name: &'a str,
    children: Vec<Instance<'a>>,
}

impl<'a> Instance<'a> {
    fn new(class_name: &'a str) -> Self {
        Self {
            class_name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Instance<'a>) {
        self.children.push(child);
    }
}

impl fmt::Display for Instance<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "<Item class=\"{}\" referent=\"{}\">",
            self.class_name, self.class_name
        )?;

        for child in &self.children {
            write!(f, "{}", child)?;
        }

        writeln!(f, "</Item>")?;

        Ok(())
    }
}
