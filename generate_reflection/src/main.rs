mod api_dump;
mod database;
mod property_patches;

use std::{error::Error, fs, io, path::PathBuf};

use crate::{
    api_dump::Dump, database::ReflectionDatabase, property_patches::load_property_patches,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut database = ReflectionDatabase::new();

    let dump = Dump::read()?;
    database.populate_from_dump(&dump)?;

    let property_patches = load_property_patches();
    database.populate_from_patches(&property_patches)?;

    database.validate();

    emit_database(&database)?;

    Ok(())
}

fn emit_database(database: &ReflectionDatabase) -> io::Result<()> {
    let rust_output = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("rbx_reflection_database/database.msgpack");
        path
    };

    let encoded = rmp_serde::to_vec(database).unwrap();
    fs::write(&rust_output, encoded)?;

    let lua_output = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("rbx_dom_lua/src/database-json.txt");
        path
    };

    let encoded = serde_json::to_string(database).unwrap();
    fs::write(&lua_output, encoded)?;

    Ok(())
}
