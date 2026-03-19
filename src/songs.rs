use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub composer: String,
    pub notes: Vec<String>,
}

const VALID_NOTES: &[&str] = &[
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
    "C#", "D#", "F#", "G#", "A#",
];

/// Validate a song's structure and notes.
pub fn validate(song: &Song) -> Result<()> {
    if song.id.is_empty() {
        anyhow::bail!("song id cannot be empty");
    }
    if song.id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
        anyhow::bail!("song id can only contain alphanumeric characters, hyphens, and underscores");
    }
    if song.title.is_empty() {
        anyhow::bail!("song title cannot be empty");
    }
    if song.composer.is_empty() {
        anyhow::bail!("song composer cannot be empty");
    }
    if song.notes.is_empty() {
        anyhow::bail!("song must have at least one note");
    }
    for (i, note) in song.notes.iter().enumerate() {
        if !is_valid_note(note) {
            anyhow::bail!(
                "invalid note '{}' at position {} (expected e.g. C4, Bb3, Gb5)",
                note,
                i + 1
            );
        }
    }
    Ok(())
}

fn is_valid_note(note: &str) -> bool {
    // Must end with a digit (octave 0-8)
    let Some(last) = note.chars().last() else {
        return false;
    };
    if !last.is_ascii_digit() {
        return false;
    }
    let name = &note[..note.len() - 1];
    VALID_NOTES.iter().any(|v| *v == name)
}

/// Return the full catalog: built-in songs + user-added songs.
pub fn catalog() -> Vec<Song> {
    let mut songs = builtin_songs();

    // Load user songs from ~/.typiano/songs/
    let user_dir = config::user_songs_dir();
    if let Ok(entries) = std::fs::read_dir(&user_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(data) = std::fs::read_to_string(&path) {
                    if let Ok(song) = serde_json::from_str::<Song>(&data) {
                        // Skip if ID conflicts with existing
                        if !songs.iter().any(|s| s.id == song.id) {
                            songs.push(song);
                        }
                    }
                }
            }
        }
    }

    songs
}

/// Get a song by ID from the full catalog.
pub fn get_song(id: &str) -> Option<Song> {
    catalog().into_iter().find(|s| s.id == id)
}

/// Add a user song from a JSON file path.
pub fn add_song(path: &Path) -> Result<Song> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read file: {}", path.display()))?;

    let song: Song = serde_json::from_str(&data).context(
        "invalid JSON format. Expected: {\"id\": \"...\", \"title\": \"...\", \"composer\": \"...\", \"notes\": [\"C4\", ...]}",
    )?;

    validate(&song)?;

    // Check for duplicate ID
    let existing = catalog();
    if existing.iter().any(|s| s.id == song.id) {
        anyhow::bail!(
            "a song with id '{}' already exists. Use a different id or remove it first.",
            song.id
        );
    }

    // Save to user songs directory
    let dest = config::user_songs_dir().join(format!("{}.json", song.id));
    let pretty = serde_json::to_string_pretty(&song)?;
    std::fs::write(&dest, pretty)?;

    Ok(song)
}

/// Remove a user-added song by ID. Returns error if it's a built-in song.
pub fn remove_song(id: &str) -> Result<()> {
    let builtin = builtin_songs();
    if builtin.iter().any(|s| s.id == id) {
        anyhow::bail!("'{}' is a built-in song and cannot be removed.", id);
    }

    let path = config::user_songs_dir().join(format!("{id}.json"));
    if !path.exists() {
        anyhow::bail!("no user song found with id '{id}'.");
    }

    std::fs::remove_file(&path)?;
    Ok(())
}

fn builtin_songs() -> Vec<Song> {
    let mut songs = Vec::new();
    let song_files: &[&str] = &[
        include_str!("../songs/fur-elise.json"),
        include_str!("../songs/chopin-nocturne-9-2.json"),
        include_str!("../songs/moonlight-sonata.json"),
        include_str!("../songs/clair-de-lune.json"),
        include_str!("../songs/gymnopedie-1.json"),
        include_str!("../songs/bach-prelude-c.json"),
        include_str!("../songs/turkish-march.json"),
        include_str!("../songs/chopin-waltz-64-2.json"),
        include_str!("../songs/debussy-arabesque-1.json"),
        include_str!("../songs/chopin-prelude-4.json"),
        include_str!("../songs/beethoven-pathetique-2.json"),
        include_str!("../songs/bach-invention-1.json"),
        include_str!("../songs/satie-gnossienne-1.json"),
        include_str!("../songs/chopin-etude-10-3.json"),
        include_str!("../songs/debussy-reverie.json"),
        include_str!("../songs/liszt-liebestraum-3.json"),
        include_str!("../songs/schumann-traumerei.json"),
        include_str!("../songs/grieg-morning-mood.json"),
        include_str!("../songs/schubert-impromptu-90-3.json"),
        include_str!("../songs/chopin-ballade-1.json"),
        include_str!("../songs/canon-in-d.json"),
        include_str!("../songs/river-flows-in-you.json"),
        include_str!("../songs/comptine.json"),
        include_str!("../songs/spring-waltz.json"),
        include_str!("../songs/la-campanella.json"),
        include_str!("../songs/fantasie-impromptu.json"),
        include_str!("../songs/waltz-of-flowers.json"),
        include_str!("../songs/eine-kleine.json"),
        include_str!("../songs/swan-lake.json"),
        include_str!("../songs/flight-of-bumblebee.json"),
        include_str!("../songs/bolero.json"),
        include_str!("../songs/clair-de-lune-suite.json"),
        include_str!("../songs/chopin-raindrop.json"),
        include_str!("../songs/beethoven-5th.json"),
        include_str!("../songs/debussy-doctor-gradus.json"),
    ];
    for data in song_files {
        if let Ok(song) = serde_json::from_str::<Song>(data) {
            songs.push(song);
        }
    }
    songs
}
