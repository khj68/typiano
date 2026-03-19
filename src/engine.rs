use crate::songs::Song;

/// Manages the current song state and note progression.
pub struct Engine {
    current_song: Song,
    note_index: usize,
}

impl Engine {
    pub fn new(song: Song) -> Self {
        Self {
            current_song: song,
            note_index: 0,
        }
    }

    /// Get the next note to play, wrapping around at the end.
    pub fn next_note(&mut self) -> &str {
        if self.current_song.notes.is_empty() {
            return "C4";
        }
        let note = &self.current_song.notes[self.note_index];
        self.note_index = (self.note_index + 1) % self.current_song.notes.len();
        note
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
}
