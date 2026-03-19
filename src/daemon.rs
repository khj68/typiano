use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};

use crate::audio::AudioPlayer;
use crate::config;
use crate::engine::Engine;
use crate::input;
use crate::ipc;
use crate::songs;

/// Check if the daemon is currently running.
pub fn is_running() -> bool {
    let pid_path = config::pid_path();
    if !pid_path.exists() {
        return false;
    }

    if let Ok(pid_str) = std::fs::read_to_string(&pid_path) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            unsafe {
                if libc::kill(pid, 0) == 0 {
                    return true;
                }
            }
        }
    }

    // Stale PID file, clean up
    let _ = std::fs::remove_file(&pid_path);
    let _ = std::fs::remove_file(config::socket_path());
    false
}

/// Start the daemon process.
pub fn start() -> Result<()> {
    let exe = std::env::current_exe().context("cannot find executable path")?;
    let child = std::process::Command::new(exe)
        .arg("_daemon")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("failed to spawn daemon")?;

    std::fs::write(config::pid_path(), child.id().to_string())?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    Ok(())
}

/// The actual daemon entry point.
pub fn run_daemon() -> Result<()> {
    std::fs::write(config::pid_path(), std::process::id().to_string())?;

    let catalog = songs::catalog();
    let initial_song = catalog
        .choose(&mut rand::thread_rng())
        .cloned()
        .context("no songs available")?;

    let engine = Arc::new(Mutex::new(Engine::new(initial_song)));

    // Channel for note names: input thread → main thread (audio)
    let (note_tx, note_rx) = std::sync::mpsc::channel::<String>();

    // Start keyboard listener → sends note names
    let engine_for_input = Arc::clone(&engine);
    let key_rx = input::start_listener();
    std::thread::spawn(move || {
        while key_rx.recv().is_ok() {
            let note = {
                let mut eng = engine_for_input.lock().unwrap();
                eng.next_note().to_string()
            };
            let _ = note_tx.send(note);
        }
    });

    // Start IPC server in a separate thread
    let engine_for_ipc = Arc::clone(&engine);
    let shutdown_flag = Arc::new(Mutex::new(false));
    let shutdown_for_ipc = Arc::clone(&shutdown_flag);

    std::thread::spawn(move || {
        let _ = ipc::start_server(Box::new(move |cmd: &str| {
            if cmd == "shutdown" {
                *shutdown_for_ipc.lock().unwrap() = true;
                return "ok".to_string();
            }
            if cmd == "status" {
                let eng = engine_for_ipc.lock().unwrap();
                let song = eng.current_song();
                let (pos, total) = eng.progress();
                let pct = if total > 0 { pos * 100 / total } else { 0 };
                let filled = pct / 5;
                let bar: String = "█".repeat(filled) + &"░".repeat(20 - filled);
                return format!(
                    "Now playing: {} - {}\nSong ID:     {}\nProgress:    [{bar}] {pos}/{total} notes ({pct}%)",
                    song.composer, song.title, song.id
                );
            }
            if cmd == "random" {
                let catalog = songs::catalog();
                let mut eng = engine_for_ipc.lock().unwrap();
                let current_id = eng.current_song().id.clone();
                let candidates: Vec<_> = catalog
                    .into_iter()
                    .filter(|s| s.id != current_id)
                    .collect();
                if let Some(song) = candidates.choose(&mut rand::thread_rng()).cloned() {
                    let name = format!("{} - {}", song.composer, song.title);
                    eng.set_song(song);
                    return format!("ok:{name}");
                }
                return "ok".to_string();
            }
            if let Some(id) = cmd.strip_prefix("play:") {
                if let Some(song) = songs::get_song(id) {
                    let mut eng = engine_for_ipc.lock().unwrap();
                    eng.set_song(song);
                    return "ok".to_string();
                }
                return "error: song not found".to_string();
            }
            "error: unknown command".to_string()
        }));
    });

    // Audio playback on main thread (OutputStream is !Send)
    let player = AudioPlayer::new().context("failed to initialize audio")?;

    loop {
        // Check shutdown
        if *shutdown_flag.lock().unwrap() {
            break;
        }

        // Wait for a note with timeout so we can check shutdown periodically
        match note_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(note) => {
                if let Err(e) = player.play_note(&note) {
                    eprintln!("audio error: {e}");
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    let _ = std::fs::remove_file(config::pid_path());
    let _ = std::fs::remove_file(config::socket_path());
    Ok(())
}
