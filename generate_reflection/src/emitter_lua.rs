use std::{
    io,
    path::Path,
};

use crate::database::ReflectionDatabase;

pub fn emit(_database: &ReflectionDatabase, _output_dir: &Path) -> io::Result<()> {
    Ok(())
}