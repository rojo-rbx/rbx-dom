mod api_dump;
mod database;
mod property_patches;

use std::{error::Error, fs, path::PathBuf};

use structopt::StructOpt;

use crate::{
    api_dump::Dump, database::ReflectionDatabase, property_patches::load_property_patches,
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

    let mut database = ReflectionDatabase::new();

    let dump = Dump::read()?;
    database.populate_from_dump(&dump)?;

    let property_patches = load_property_patches();
    database.populate_from_patches(&property_patches)?;

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
