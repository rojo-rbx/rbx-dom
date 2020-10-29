mod api_dump;
mod database;
mod defaults_place;
mod plugin_injector;
mod property_patches;

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::api_dump::Dump;
use crate::database::ReflectionDatabase;
use crate::defaults_place::measure_default_properties;
use crate::property_patches::PropertyPatches;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long = "json")]
    json_path: Option<PathBuf>,

    #[structopt(long = "msgpack")]
    msgpack_path: Option<PathBuf>,
}

fn run(options: Options) -> anyhow::Result<()> {
    let mut database = ReflectionDatabase::new();

    let dump = Dump::read()?;
    database.populate_from_dump(&dump)?;

    let property_patches = PropertyPatches::load()?;
    property_patches.apply(&mut database.0)?;

    measure_default_properties(&mut database)?;

    database.validate();

    if let Some(path) = &options.msgpack_path {
        let encoded = rmp_serde::to_vec(&database)?;
        fs::write(path, encoded)?;
    }

    if let Some(path) = &options.json_path {
        let encoded = serde_json::to_string(&database)?;
        fs::write(&path, encoded)?;
    }

    Ok(())
}

fn main() {
    let options = Options::from_args();

    let log_env = env_logger::Env::default().default_filter_or("info");

    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        // Indent following lines equal to the log level label, like `[ERROR] `
        .format_indent(Some(8))
        .init();

    if let Err(err) = run(options) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
