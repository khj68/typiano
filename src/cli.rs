use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::audio::SoundType;
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
    On {
        /// Sound type: rhodes or piano
        #[arg(long, default_value = "rhodes")]
        sound: String,
    },
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
    /// Change sound type (rhodes or piano)
    Sound {
        /// Sound type
        #[arg(value_parser = ["rhodes", "piano"])]
        name: String,
    },
    /// Set play mode for song endings (random or repeat)
    Mode {
        /// Play mode
        #[arg(value_parser = ["random", "repeat"])]
        name: String,
    },
    /// Enter free play mode (keyboard as piano)
    Freeplay,
    /// Return to song mode
    Song,
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
    Daemon {
        #[arg(long)]
        sound: Option<String>,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::On { sound } => {
            if daemon::is_running() {
                println!("typiano is already running.");
                return Ok(());
            }
            let sound_type: SoundType = sound
                .parse()
                .map_err(|e: String| anyhow::anyhow!(e))?;
            daemon::start(Some(sound_type)).context("failed to start daemon")?;
            println!("🎹 typiano started! Start typing to play piano. (sound: {sound_type})");
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
        Command::Sound { name } => {
            let resp = ipc::send_command(&format!("sound:{name}"))?;
            if resp == "ok" {
                println!("Sound changed to: {name}");
            } else {
                println!("{resp}");
            }
        }
        Command::Mode { name } => {
            let resp = ipc::send_command(&format!("mode:{name}"))?;
            if resp == "ok" {
                println!("Play mode set to: {name}");
            } else {
                println!("{resp}");
            }
        }
        Command::Freeplay => {
            let resp = ipc::send_command("gamemode:freeplay")?;
            if resp == "ok" {
                println!("🎹 Free play mode! Your keyboard is now a piano.");
                println!("  Lower octave: Z(C3) X(D3) C(E3) V(F3) B(G3) N(A3) M(B3)");
                println!("                S(Db3) D(Eb3) G(Gb3) H(Ab3) J(Bb3)");
                println!("  Upper octave: Q(C4) W(D4) E(E4) R(F4) T(G4) Y(A4) U(B4)");
                println!("                2(Db4) 3(Eb4) 5(Gb4) 6(Ab4) 7(Bb4)");
                println!("  High notes:   I(C5) O(D5) P(E5) | 9(Db5) 0(Eb5)");
            } else {
                println!("{resp}");
            }
        }
        Command::Song => {
            let resp = ipc::send_command("gamemode:song")?;
            if resp == "ok" {
                println!("Switched back to song mode.");
            } else {
                println!("{resp}");
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
        Command::Daemon { sound } => {
            let sound_type = sound.and_then(|s| s.parse::<SoundType>().ok());
            daemon::run_daemon(sound_type)?;
        }
    }
    Ok(())
}
