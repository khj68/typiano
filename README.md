<div align="center">

```
    ┌─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┐
    │ │ ││ │ │ │ ││ ││ │ │ │ ││ │ │ │ ││ ││ │ │
    │ │█││█│ │ │█││█││█│ │ │█││█│ │ │█││█││█│ │
    │ │█││█│ │ │█││█││█│ │ │█││█│ │ │█││█││█│ │
    │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
    │  │  │  │  │  │  │  │  │  │  │  │  │  │  │
    └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
```

# 🎹 Typiano

**Turn your typing into a piano performance.**

Every keystroke plays the next note from a classical piano piece.<br>
Type an email, write code, chat with friends — and hear Chopin, Debussy, or Bach play along.

🎵 *Your keyboard is now a concert hall.* 🎵

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

[English](README.md) · [한국어](docs/README.ko.md) · [日本語](docs/README.ja.md) · [中文](docs/README.zh.md)

</div>

---

## 🎬 Demo

```
$ typiano on
🎹 typiano started! Start typing to play piano. (sound: rhodes)

$ typiano status
Now playing: Chopin - Nocturne Op.9 No.2
Song ID:     chopin-nocturne-9-2
Progress:    [████████░░░░░░░░░░░░] 63/150 notes (42%)
Sound:       rhodes
Play mode:   random
Game mode:   song

$ typiano sound piano
Sound changed to: piano

$ typiano freeplay
🎹 Free play mode! Your keyboard is now a piano.
  White keys: A(A3) S(B3) D(C4) F(D4) G(E4) H(F4) J(G4) K(A4) L(B4) ;(C5) '(D5)
  Black keys: W(Bb3) R(Db4) T(Eb4) Y(Gb4) U(Ab4) I(Bb4) P(Db5)

$ typiano off
typiano stopped.
```

## 📦 Installation

### Prerequisites

Typiano is built with Rust. If you don't have Rust installed:

```bash
# Option 1: Official installer (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Option 2: Homebrew (macOS)
brew install rust
```

### From source (recommended)

```bash
git clone https://github.com/khj68/typiano.git
cd typiano
cargo install --path .
```

### Build manually

```bash
cargo build --release
# Binary: ./target/release/typiano
```

### Homebrew (coming soon)

```bash
brew tap khj68/typiano
brew install typiano
```

## 🚀 Usage

| Command | Description |
|---------|-------------|
| `typiano on [--sound rhodes\|piano]` | Start daemon (random song, optional sound) |
| `typiano off` | Stop daemon |
| `typiano play <id>` | Switch to a specific song |
| `typiano list` | Show all available songs |
| `typiano status` | Current song, progress, mode & sound |
| `typiano random` | Switch to a random song |
| `typiano sound <rhodes\|piano>` | Change sound type |
| `typiano mode <random\|repeat>` | Set play mode for song endings |
| `typiano freeplay` | Enter free play mode (keyboard as piano) |
| `typiano song` | Return to song mode |
| `typiano add <file>` | Add a custom song from JSON |
| `typiano remove <id>` | Remove a user-added song |

### Add your own songs

Anyone can add custom songs:

```bash
typiano add my-song.json       # Add
typiano remove my-song          # Remove (user-added only)
```

Song JSON format:

```json
{
  "id": "my-song",
  "title": "My Song",
  "composer": "Me",
  "notes": ["C4", "E4", "G4", "C5", "E5", "G5", "C6"]
}
```

Notes use scientific pitch notation: `C2` to `C7`, with flats (`Db`, `Eb`, `Gb`, `Ab`, `Bb`).

### Import from MIDI

A built-in MIDI converter is included for accurate melody extraction:

```bash
python3 tools/midi2typiano.py song.mid \
  --id my-song --title "My Song" --composer "Someone" \
  --max-notes 150
```

## 🎶 Built-in Songs

29 pieces, all sourced from real MIDI files for accurate melodies:

| Composer | Pieces |
|----------|--------|
| **Beethoven** | Fur Elise, Moonlight Sonata, Pathetique (2nd mvt) |
| **Chopin** | Nocturne Op.9 No.2, Waltz Op.64 No.2, Prelude Op.28 No.4, Etude Op.10 No.3, Ballade No.1, Fantasie-Impromptu, Raindrop Prelude |
| **Debussy** | Clair de Lune, Arabesque No.1 |
| **Bach** | Prelude in C Major BWV 846, Two-Part Invention No.1 |
| **Mozart** | Turkish March, Eine kleine Nachtmusik |
| **Liszt** | Liebestraum No.3, La Campanella |
| **Tchaikovsky** | Swan Lake Theme, Waltz of the Flowers |
| **Satie** | Gymnopedie No.1, Gnossienne No.1 |
| **Ravel** | Bolero |
| **Pachelbel** | Canon in D |
| **Rimsky-Korsakov** | Flight of the Bumblebee |
| **Yiruma** | River Flows in You |
| **Tiersen** | Comptine d'un autre ete (Amelie) |
| **Schumann** | Traumerei |
| **Grieg** | Morning Mood (Peer Gynt) |

## 🎹 Sound

Typiano comes with **two sound types**:

- **Rhodes** (Electric Piano) — Clean, warm MIDI tone (default)
- **Piano** (Acoustic Piano) — Brighter attack, richer overtones, longer sustain

Both use synthesized samples: 61 keys (C2 – C7) with harmonic overtones and natural fade-out.

Switch anytime: `typiano sound piano` or `typiano sound rhodes`

## 🎹 Free Play Mode

Use your keyboard as a piano! Enter with `typiano freeplay`:

```
Upper octave:   2    3         5    6    7              9    0
                C#4  D#4       F#4  G#4  A#4            C#5  D#5
              Q    W    E    R    T    Y    U    I    O    P
              C4   D4   E4   F4   G4   A4   B4   C5   D5   E5

Lower octave:   S    D         G    H    J
                C#3  D#3       F#3  G#3  A#3
              Z    X    C    V    B    N    M
              C3   D3   E3   F3   G3   A3   B3
```

Return to song mode: `typiano song`

## ⚙️ How It Works

```
typiano on  →  background daemon spawns
                ├── rdev::listen   (global key capture)
                ├── rodio          (audio playback)
                └── Unix socket    (IPC server)

Song mode:      keystroke  →  next note from song  →  sample plays
Free play mode: keystroke  →  mapped piano note    →  sample plays
```

1. `typiano on` spawns a background daemon process
2. The daemon captures global keyboard events via `rdev`
3. In **song mode**, each keypress advances the song and plays the next note
4. When the song ends: **random mode** auto-switches to a new song, **repeat mode** loops
5. In **free play mode**, keys are mapped to piano notes for live performance
6. `typiano off` sends a shutdown command via Unix domain socket

## 🖥️ Requirements

| Platform | Audio Backend | Key Capture |
|----------|--------------|-------------|
| **macOS** | CoreAudio | CGEventTap |
| **Linux** | ALSA / PulseAudio | evdev |

> **macOS**: Accessibility permission is required for global key listening.<br>
> Go to **System Settings → Privacy & Security → Accessibility** and enable your terminal app.

## 🏗️ Architecture

```
src/
├── main.rs      # CLI entrypoint (clap)
├── cli.rs       # Subcommand handlers
├── daemon.rs    # Daemon lifecycle (fork, PID, signal)
├── input.rs     # rdev key listener (sends Key events)
├── engine.rs    # Song state machine, play/game modes, key-to-note mapping
├── audio.rs     # rodio playback, dual sample banks (Rhodes/Piano)
├── ipc.rs       # Unix socket server/client
├── songs.rs     # Song struct, loader, validator
└── config.rs    # Paths & state

tools/
├── midi2typiano.py      # MIDI → JSON song converter
└── generate_samples.sh  # Sample generation script (Rhodes + Piano)
```

## 🤝 Contributing

Contributions are welcome! Here are some ways to help:

- 🎵 **Add songs**: Convert a MIDI file with `midi2typiano.py` and submit a PR
- 🐛 **Report bugs**: Open an issue
- 💡 **Suggest features**: Open an issue with `[Feature Request]`

### Adding a new song

1. Find a MIDI file of the piece
2. Convert: `python3 tools/midi2typiano.py song.mid --id song-id --title "Title" --composer "Composer"`
3. Place the JSON in `songs/`
4. Add the `include_str!` line in `src/songs.rs`
5. Submit a PR

## 📄 License

[MIT](LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*Type something. Hear something beautiful.*

</div>
