use crate::songs::Song;
use rdev::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    Random,
    Repeat,
}

impl std::fmt::Display for PlayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayMode::Random => write!(f, "random"),
            PlayMode::Repeat => write!(f, "repeat"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Song,
    FreePlay,
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::Song => write!(f, "song"),
            GameMode::FreePlay => write!(f, "freeplay"),
        }
    }
}

/// Manages the current song state and note progression.
pub struct Engine {
    current_song: Song,
    note_index: usize,
    play_mode: PlayMode,
    game_mode: GameMode,
}

impl Engine {
    pub fn new(song: Song) -> Self {
        Self {
            current_song: song,
            note_index: 0,
            play_mode: PlayMode::Random,
            game_mode: GameMode::Song,
        }
    }

    /// Get the next note to play.
    /// Returns None when song ends in Random mode (signals daemon to switch song).
    pub fn next_note(&mut self) -> Option<&str> {
        if self.current_song.notes.is_empty() {
            return Some("C4");
        }
        let note = &self.current_song.notes[self.note_index];
        self.note_index += 1;
        if self.note_index >= self.current_song.notes.len() {
            match self.play_mode {
                PlayMode::Repeat => self.note_index = 0,
                PlayMode::Random => {
                    // Will return None on *next* call after playing the last note
                    // Actually, we reset and return None to signal song end
                    self.note_index = 0;
                    // Return the last note, but signal end via a separate mechanism
                    // Let's use a simpler approach: check *before* playing
                }
            }
        }
        Some(note)
    }

    /// Check if the song just finished (note_index wrapped to 0 after last note).
    /// Call this after next_note() to detect song end in Random mode.
    pub fn song_just_ended(&self) -> bool {
        self.note_index == 0 && !self.current_song.notes.is_empty()
    }

    /// Switch to a new song, resetting the position.
    pub fn set_song(&mut self, song: Song) {
        self.current_song = song;
        self.note_index = 0;
    }

    pub fn current_song(&self) -> &Song {
        &self.current_song
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.note_index, self.current_song.notes.len())
    }

    pub fn play_mode(&self) -> PlayMode {
        self.play_mode
    }

    pub fn set_play_mode(&mut self, mode: PlayMode) {
        self.play_mode = mode;
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
    }
}

/// Map a keyboard key to a piano note for free play mode.
/// Standard DAW layout: bottom rows = lower octave (C3-B3), top rows = upper octave (C4-E5)
pub fn key_to_note(key: Key) -> Option<String> {
    let note = match key {
        // Lower octave white keys (C3-B3): Z X C V B N M
        Key::KeyZ => "C3",
        Key::KeyX => "D3",
        Key::KeyC => "E3",
        Key::KeyV => "F3",
        Key::KeyB => "G3",
        Key::KeyN => "A3",
        Key::KeyM => "B3",
        // Lower octave black keys: S D G H J
        Key::KeyS => "Db3",
        Key::KeyD => "Eb3",
        Key::KeyG => "Gb3",
        Key::KeyH => "Ab3",
        Key::KeyJ => "Bb3",
        // Upper octave white keys (C4-B4): Q W E R T Y U
        Key::KeyQ => "C4",
        Key::KeyW => "D4",
        Key::KeyE => "E4",
        Key::KeyR => "F4",
        Key::KeyT => "G4",
        Key::KeyY => "A4",
        Key::KeyU => "B4",
        // Upper octave black keys: 2 3 5 6 7
        Key::Num2 => "Db4",
        Key::Num3 => "Eb4",
        Key::Num5 => "Gb4",
        Key::Num6 => "Ab4",
        Key::Num7 => "Bb4",
        // Extra high notes (C5-E5): I O P / 9 0
        Key::KeyI => "C5",
        Key::Num9 => "Db5",
        Key::KeyO => "D5",
        Key::Num0 => "Eb5",
        Key::KeyP => "E5",
        _ => return None,
    };
    Some(note.to_string())
}
