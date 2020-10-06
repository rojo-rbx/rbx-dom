mod api_dump;
mod database;
mod defaults_place;
mod plugin_injector;
mod property_patches;

use std::{error::Error, fs, path::PathBuf};

use structopt::StructOpt;

use crate::{
    api_dump::Dump, database::ReflectionDatabase, defaults_place::measure_default_properties,
    property_patches::load_property_patches,
};

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long = "json")]
    json_path: Option<PathBuf>,

    #[structopt(long = "msgpack")]
    msgpack_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let log_env = env_logger::Env::default().default_filter_or("info");

    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        // Indent following lines equal to the log level label, like `[ERROR] `
        .format_indent(Some(8))
        .init();

    let mut database = ReflectionDatabase::new();

    let property_patches = load_property_patches();
    let dump = Dump::read()?;

    database.populate_from_dump(&dump)?;
    database.populate_from_patches(&property_patches)?;

    measure_default_properties(&mut database)?;

    database.validate();

    if let Some(path) = &options.msgpack_path {
        let encoded = rmp_serde::to_vec(&database).unwrap();
        fs::write(path, encoded)?;
    }

    if let Some(path) = &options.json_path {
        let encoded = serde_json::to_string(&database).unwrap();
        fs::write(&path, encoded)?;
    }

    Ok(())
}
