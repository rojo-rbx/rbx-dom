use std::{
    fmt::{self, Write},
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc,
    time::Duration,
};

use anyhow::{bail, Context};
use clap::Parser;
#[cfg(target_os = "windows")]
use innerput::{Innerput, Key, Keyboard};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use roblox_install::RobloxStudio;
use serde::Deserialize;
use tempfile::tempdir;
use tiny_http::Response;

use crate::api_dump::Dump;

static PLUGIN_SOURCE: &str = include_str!("../../plugin.lua");

/// Generate a place file with all classes and their default properties.
#[derive(Debug, Parser)]
pub struct DefaultsPlaceSubcommand {
    /// The path of an API dump that came from the dump command.
    #[clap(long)]
    pub api_dump: PathBuf,
    /// Where to output the place. The extension must be .rbxlx
    pub output: PathBuf,
}

impl DefaultsPlaceSubcommand {
    pub fn run(&self) -> anyhow::Result<StudioInfo> {
        if self.output.extension().unwrap_or_default() != "rbxlx" {
            bail!("The output path must have a .rbxlx extension")
        }

        let contents = fs::read_to_string(&self.api_dump).context("Could not read API dump")?;
        let dump = serde_json::from_str(&contents).context("Invalid API dump")?;

        // Studio leaves a .lock file behind because the defaults place crashes on close. This uses a temporary
        // directory to ignore the lock file.
        let temp_dir = tempdir()?;
        let temp_place_path = temp_dir.path().join("defaults-place.rbxlx");

        generate_place_with_all_classes(&temp_place_path, &dump)?;
        let studio_info = save_place_in_studio(&temp_place_path)?;

        fs::copy(temp_place_path, &self.output)?;

        Ok(studio_info)
    }
}

fn save_place_in_studio(path: &PathBuf) -> anyhow::Result<StudioInfo> {
    let studio_install =
        RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

    let plugin_injector = PluginInjector::start(&studio_install)?;

    log::info!("Starting Roblox Studio...");

    let mut studio_process = Command::new(studio_install.application_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(path)
        .spawn()?;

    log::info!("Waiting for Roblox Studio to re-save place...");

    let studio_info = plugin_injector.wait_for_response()?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    #[cfg(target_os = "windows")]
    {
        let result = Innerput::new().send_chord(&[Key::Control, Key::Char('s')], &studio_process);

        if let Err(err) = result {
            log::error!("{err}");

            println!("Failed to send key chord to Roblox Studio. Please save the opened place manually (ctrl+s).")
        }
    }

    #[cfg(target_os = "macos")]
    {
        let process_id = studio_process.id();
        let script = format!(
            r#"
tell application "System Events"
    set robloxStudio to the first process whose unix id is {process_id}

    -- The generated defaults place can cause Roblox Studio to create modal
    -- dialogs that prevent command + S from saving the place. To work around
    -- this, we need to repeatedly press escape then command + S until the Roblox
    -- Studio process only has one window.

    -- This could be hazardous - for example, escape may not close every modal,
    -- or Roblox Studio could one day gain more windows. So, we'll also cap the 
    -- number of times this loop can execute to 100.
    set attemptCount to 0
    repeat until count of windows of robloxStudio is 1 or attemptCount >= 100
        set frontmost of robloxStudio to true

        -- We should avoid making any keystrokes when Roblox Studio is not the
        -- frontmost application. We just set it frontmost, but who knows what
        -- the user is doing and how it could interact with AppleScript here...
        if frontmost of robloxStudio then
            key code 53
            keystroke "s" using command down
        end if

        set attemptCount to attemptCount + 1
    end repeat
end tell
"#
        );

        Command::new("osascript")
            .args(["-e", script.as_str()])
            .output()?;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    println!("Please save the opened place in Roblox Studio (ctrl+s).");

    loop {
        let event = rx.recv()??;
        if event.kind.is_create() || event.kind.is_modify() {
            break;
        }
    }

    log::info!("Place saved, killing Studio...");

    studio_process.kill()?;

    Ok(studio_info)
}

fn generate_place_with_all_classes(path: &PathBuf, dump: &Dump) -> anyhow::Result<()> {
    let mut place_contents = String::new();

    writeln!(place_contents, "<roblox version=\"4\">").unwrap();

    for class in &dump.classes {
        let mut instance = Instance::new(&class.name);

        match &*class.name {
            // These classes can't be put into place files by default.
            "DebuggerWatch" | "DebuggerBreakpoint" | "AdvancedDragger" | "Dragger"
            | "ScriptDebugger" | "PackageLink" | "Ad" | "AdPortal" | "AdGui"
            | "InternalSyncItem" | "AuroraScript" => continue,

            // Settings singletons cannot be put into a DataModel. This changed
            // in release 653 and 657.
            "DebugSettings" | "GameSettings" | "LuaSettings" | "NetworkSettings"
            | "PhysicsSettings" | "RenderSettings" | "Studio" | "TaskScheduler"
            | "UserGameSettings" => continue,

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
            | "WrapTarget"
            | "WrapDeformer" => continue,

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
                instance.add_child(Instance::new("WrapDeformer"));
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

pub struct PluginInjector<'a> {
    http_server: tiny_http::Server,
    studio_install: &'a RobloxStudio,
}

#[derive(Debug, Deserialize)]
pub struct StudioInfo {
    pub version: [u32; 4],
}

impl<'a> PluginInjector<'a> {
    pub fn start(studio_install: &'a RobloxStudio) -> anyhow::Result<Self> {
        let http_server = tiny_http::Server::http("0.0.0.0:22073").unwrap();

        log::info!("Installing Studio plugin");

        let plugin_path = studio_install
            .plugins_path()
            .join("RbxDomDefaultsPlacePlugin.lua");

        fs::write(plugin_path, PLUGIN_SOURCE)?;

        Ok(Self {
            http_server,
            studio_install,
        })
    }

    pub fn wait_for_response(self) -> anyhow::Result<StudioInfo> {
        let mut request = match self.http_server.recv_timeout(Duration::from_secs(30))? {
            Some(request) => request,
            None => bail!("Plugin did not send a response within 30 seconds"),
        };

        let studio_info: StudioInfo = serde_json::from_reader(request.as_reader())?;
        request.respond(Response::empty(200))?;

        Ok(studio_info)
    }
}

impl Drop for PluginInjector<'_> {
    fn drop(&mut self) {
        log::info!("Uninstalling Studio plugin");

        let plugin_path = self
            .studio_install
            .plugins_path()
            .join("RbxDomDefaultsPlacePlugin.lua");

        if let Err(err) = fs::remove_file(plugin_path) {
            log::error!("Could not remove plugin: {err}");
        }
    }
}
