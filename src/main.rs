mod audio;
mod cli;
mod config;
mod daemon;
mod engine;
mod input;
mod ipc;
mod songs;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
