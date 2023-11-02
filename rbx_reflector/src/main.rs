mod api_dump;
mod cli;
mod defaults;
mod patches;

use clap::Parser;

use crate::cli::Args;

fn main() {
    let args = Args::parse();

    let log_env = env_logger::Env::default().default_filter_or("info");

    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        .init();

    if let Err(err) = args.run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
