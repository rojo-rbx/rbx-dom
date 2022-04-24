mod api_dump;
mod cli;

use clap::Parser;

use crate::cli::Args;

fn main() {
    let args = Args::parse();
    println!("{args:?}");

    if let Err(err) = args.run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
