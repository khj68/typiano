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
🎹 typiano started! Start typing to play piano.

$ typiano status
Now playing: Chopin - Nocturne Op.9 No.2
Song ID:     chopin-nocturne-9-2
Progress:    [████████░░░░░░░░░░░░] 42/122 notes (34%)

$ typiano list
Available songs (20):

   fur-elise                      Beethoven - Fur Elise
   chopin-nocturne-9-2            Chopin - Nocturne Op.9 No.2
   moonlight-sonata               Beethoven - Moonlight Sonata
   ...
```

## 📦 Installation

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

```bash
typiano on              # Start daemon (random song)
typiano off             # Stop daemon
typiano play <song-id>  # Switch to a specific song
typiano list            # Show all available songs
typiano status          # Current song & progress
typiano random          # Switch to a random song
```

### Add your own songs

Anyone can add custom songs:

```bash
# Add a song from JSON file
typiano add my-song.json

# Remove a user-added song
typiano remove my-song
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

## 🎶 Built-in Songs

35 classical & popular piano pieces:

| Composer | Pieces |
|----------|--------|
| **Beethoven** | Für Elise, Moonlight Sonata, Pathétique (2nd mvt), Symphony No.5 |
| **Chopin** | Nocturne Op.9 No.2, Waltz Op.64 No.2, Prelude Op.28 No.4, Etude Op.10 No.3, Ballade No.1, Fantasie-Impromptu, Raindrop Prelude |
| **Debussy** | Clair de Lune, Arabesque No.1, Rêverie, Suite bergamasque Prelude, Doctor Gradus |
| **Bach** | Prelude in C Major BWV 846, Two-Part Invention No.1 |
| **Mozart** | Turkish March, Eine kleine Nachtmusik |
| **Liszt** | Liebesträum No.3, La Campanella |
| **Tchaikovsky** | Swan Lake Theme, Waltz of the Flowers |
| **Satie** | Gymnopédie No.1, Gnossienne No.1 |
| **Ravel** | Boléro |
| **Pachelbel** | Canon in D |
| **Rimsky-Korsakov** | Flight of the Bumblebee |
| **Yiruma** | River Flows in You |
| **Tiersen** | Comptine d'un autre été (Amélie) |
| **Senneville** | Spring Waltz (Mariage d'Amour) |
| **Schumann** | Träumerei |
| **Grieg** | Morning Mood (Peer Gynt) |
| **Schubert** | Impromptu Op.90 No.3 |

## ⚙️ How It Works

```
typiano on  →  background daemon spawns
                ├── rdev::listen   (global key capture)
                ├── rodio          (audio playback)
                └── Unix socket    (IPC server)

keystroke  →  next note from song  →  piano sample plays
```

1. `typiano on` spawns a background daemon process
2. The daemon captures global keyboard events via `rdev`
3. Each keypress advances the song and plays the next piano note
4. When the song ends, it loops from the beginning
5. `typiano off` sends a shutdown command via Unix domain socket

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
├── input.rs     # rdev key listener
├── engine.rs    # Song state machine (current song, note index, looping)
├── audio.rs     # rodio playback, sample bank, crossfade
├── ipc.rs       # Unix socket server/client
├── songs.rs     # Song struct, loader, validator
└── config.rs    # Paths & state
```

## 🤝 Contributing

Contributions are welcome! Here are some ways to help:

- 🎵 **Add songs**: Create a JSON file and submit a PR to `songs/`
- 🐛 **Report bugs**: Open an issue
- 💡 **Suggest features**: Open an issue with `[Feature Request]`

### Adding a new song

1. Create a JSON file following the [song format](#add-your-own-songs)
2. Place it in `songs/`
3. Add the `include_str!` line in `src/songs.rs`
4. Submit a PR

## 📄 License

[MIT](LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*Type something. Hear something beautiful.*

</div>
