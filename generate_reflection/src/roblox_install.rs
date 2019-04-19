//! Helps locate the user's install of Roblox Studio.
//!
//! This would make sense to turn into a crate eventually, since it's useful.

use std::{
    env,
    io,
    fs,
    path::PathBuf,
};

#[cfg(not(target_os = "windows"))]
compile_error!("Roblox cannot be located on this platform yet.");

fn locate_folder() -> Option<PathBuf> {
    let local_app_data = PathBuf::from(env::var("LOCALAPPDATA").ok()?);
    Some(local_app_data.join("Roblox"))
}

pub struct RobloxStudio {
    folder_path: PathBuf,
}

impl RobloxStudio {
    pub fn locate() -> io::Result<RobloxStudio> {
        let mut versions_folder = locate_folder().ok_or_else(||
            io::Error::new(io::ErrorKind::NotFound, "Roblox install not found")
        )?;

        versions_folder.push("Versions");

        for entry in fs::read_dir(&versions_folder)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let maybe_exe_path = path.join("RobloxStudioBeta.exe");

                if maybe_exe_path.is_file() {
                    return Ok(RobloxStudio {
                        folder_path: path,
                    })
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Roblox Studio install not found"))
    }

    pub fn exe_path(&self) -> PathBuf {
        self.folder_path.join("RobloxStudioBeta.exe")
    }

    pub fn built_in_plugins_path(&self) -> PathBuf {
        self.folder_path.join("BuiltInPlugins")
    }
}