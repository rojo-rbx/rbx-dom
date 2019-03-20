use std::{
    process::Command,
    path::PathBuf,
    env,
};

fn main() {
    let mut output_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    output_path.push("plugin.rbxmx");

    let status = Command::new("rojo")
        .args(&["build", "plugin", "-o"])
        .arg(format!("{}", output_path.display()))
        .status()
        .expect("Couldn't execute Rojo");

    assert!(status.success());
}