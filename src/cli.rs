use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::daemon;
use crate::ipc;
use crate::songs;

#[derive(Parser)]
#[command(name = "typiano", about = "Turn your typing into a piano performance 🎹")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the typiano daemon
    On,
    /// Stop the typiano daemon
    Off,
    /// Switch to a specific song
    Play {
        /// Song ID (e.g., "chopin-nocturne-9-2")
        id: String,
    },
    /// List available songs
    List,
    /// Show current status
    Status,
    /// Switch to a random song
    Random,
    /// Add a song from a JSON file
    Add {
        /// Path to the song JSON file
        file: String,
    },
    /// Remove a user-added song
    Remove {
        /// Song ID to remove
        id: String,
    },
    /// Internal: run as daemon (hidden)
    #[command(hide = true)]
    #[command(name = "_daemon")]
    Daemon,
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::On => {
            if daemon::is_running() {
                println!("typiano is already running.");
                return Ok(());
            }
            daemon::start().context("failed to start daemon")?;
            println!("🎹 typiano started! Start typing to play piano.");
        }
        Command::Off => {
            if !daemon::is_running() {
                println!("typiano is not running.");
                return Ok(());
            }
            ipc::send_command("shutdown")?;
            println!("typiano stopped.");
        }
        Command::Play { id } => {
            let catalog = songs::catalog();
            if !catalog.iter().any(|s| s.id == id) {
                anyhow::bail!("unknown song: {id}. Use 'typiano list' to see available songs.");
            }
            ipc::send_command(&format!("play:{id}"))?;
            let song = catalog.iter().find(|s| s.id == id).unwrap();
            println!("Now playing: {} - {}", song.composer, song.title);
        }
        Command::List => {
            let catalog = songs::catalog();
            let user_dir = crate::config::user_songs_dir();
            println!("Available songs ({}):\n", catalog.len());
            for song in &catalog {
                let marker = if user_dir.join(format!("{}.json", song.id)).exists() {
                    "*"
                } else {
                    " "
                };
                println!(" {} {:<30} {} - {}", marker, song.id, song.composer, song.title);
            }
            println!("\n * = user-added song");
            println!("   Add songs: typiano add <file.json>");
        }
        Command::Status => {
            if !daemon::is_running() {
                println!("typiano is not running.");
                return Ok(());
            }
            let resp = ipc::send_command("status")?;
            println!("{resp}");
        }
        Command::Random => {
            let resp = ipc::send_command("random")?;
            if let Some(name) = resp.strip_prefix("ok:") {
                println!("Now playing: {name}");
            } else {
                println!("Switched to a random song.");
            }
        }
        Command::Add { file } => {
            let path = std::path::Path::new(&file);
            let song = songs::add_song(path)?;
            println!(
                "Added: {} - {} ({} notes)",
                song.composer,
                song.title,
                song.notes.len()
            );
            println!("Song ID: {}", song.id);
            println!("\nUse 'typiano play {}' to play it.", song.id);
        }
        Command::Remove { id } => {
            songs::remove_song(&id)?;
            println!("Removed song: {id}");
        }
        Command::Daemon => {
            daemon::run_daemon()?;
        }
    }
    Ok(())
}
