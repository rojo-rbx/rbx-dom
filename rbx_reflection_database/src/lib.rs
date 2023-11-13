use rbx_reflection::ReflectionDatabase;

use std::{env, fs, path::PathBuf};

static ENCODED_DATABASE: &[u8] = include_bytes!("../database.msgpack");

/// The name of an environmental variable that may be used to specify
/// the location of a reflection database to use. The expected format of
/// a file at this point is MessagePack.
pub const OVERRIDE_PATH_VAR: &str = "RBX_DATABASE";

/// The name of the directory used for the local location for a reflection
/// database. The directory will be placed inside the current user's
/// local data folder on MacOS and Windows and inside
/// the home directory on Linux.
pub const LOCAL_DIR_NAME: &str = ".rbxreflection";

lazy_static::lazy_static! {
    static ref BUNDLED_DATABASE: ReflectionDatabase<'static> = {
        log::debug!("Loading bundled reflection database");
        rmp_serde::decode::from_slice(ENCODED_DATABASE).unwrap_or_else(|e| panic!("could not decode reflection database because: {}", e))
    };

    static ref LOCAL_DATABASE: Option<ReflectionDatabase<'static>> = {
        let location = get_local_location()?;
        if let Ok(file) = fs::read(&location) {
            log::debug!("Loading local reflection database from {}", location.display());
            Some(
                rmp_serde::decode::from_slice(&file).unwrap_or_else(|e| {
                    panic!("could not decode reflection database because: {}", e)
                }),
            )
        } else {
            None
        }
    };
}

/// Returns a populated [`ReflectionDatabase`]. This will attempt to load one locally and
/// if one can't be found, it will return one that is bundled with this crate.
///
/// ## Panics
///
/// Panics if a locally stored [`ReflectionDatabase`] is not valid MessagePack.
pub fn get() -> &'static ReflectionDatabase<'static> {
    get_local().unwrap_or(&BUNDLED_DATABASE)
}

/// Returns a reflection database from the file system, if one can be found.
/// This is loaded from a location set by the `RBX_DATABASE` environmental
/// variable if it's set. Otherwise, the default location is checked.
///
/// The default location varies depending upon OS:
///
/// | OS      | Location                                                            |
/// |:--------|:--------------------------------------------------------------------|
/// | Windows | `%localappdata%/.rbxreflection/database.msgpack`                    |
/// | MacOS   | `$HOME/Library/Application Support/.rbxreflection/database.msgpack` |
/// | Linux   | `$HOME/.rbxreflection/database.msgpack`                             |
///
/// The file at the above location (or the one pointed to by `RBX_DATABASE`)
/// must be valid MessagePack.
///
/// ## Panics
///
/// Panics if the file specified by `RBX_DATABASE` or in the default location
/// exists but is invalid MessagePack.
pub fn get_local() -> Option<&'static ReflectionDatabase<'static>> {
    LOCAL_DATABASE.as_ref()
}

/// Returns the locally bundled [`ReflectionDatabase`]. This database may or may
/// not be up to date, but it will always exist.
pub fn get_bundled() -> &'static ReflectionDatabase<'static> {
    &BUNDLED_DATABASE
}

/// Fetches the location a [`ReflectionDatabase`] is expected to be loaded from.
/// This may return [`None`] if the local data directory cannot be found.
pub fn get_local_location() -> Option<PathBuf> {
    if let Ok(location) = env::var(OVERRIDE_PATH_VAR) {
        log::debug!("Using enviromental variable {OVERRIDE_PATH_VAR} to fetch reflection database");
        Some(PathBuf::from(location))
    } else {
        // Due to concerns about the local data directory existing
        // on Linux, we use the home directory instead.
        #[cfg(target_os = "linux")]
        let mut home = dirs::home_dir()?;
        #[cfg(not(target_os = "linux"))]
        let mut home = dirs::data_local_dir()?;

        home.push(LOCAL_DIR_NAME);
        home.push("database.msgpack");
        Some(home)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bundled() {
        let _database = get_bundled();
    }

    #[test]
    fn env_var() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("empty.msgpack");

        env::set_var(OVERRIDE_PATH_VAR, &test_path);
        let empty_db = get();
        println!("{:?}", empty_db.version);
        assert!(empty_db.version == [0, 0, 0, 0]);
    }
}
