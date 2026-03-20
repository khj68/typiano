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

**타이핑을 피아노 연주로 바꿔보세요.**

키를 누를 때마다 클래식 피아노 곡의 다음 음이 재생됩니다.<br>
이메일을 쓰든, 코드를 짜든, 친구와 채팅을 하든 — 쇼팽, 드뷔시, 바흐의 연주가 함께합니다.

🎵 *당신의 키보드가 콘서트홀이 됩니다.* 🎵

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

[English](../README.md) · **한국어** · [日本語](README.ja.md) · [中文](README.zh.md)

</div>

---

## 🎬 데모

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

$ typiano off
typiano stopped.
```

## 📦 설치

### 사전 요구사항

Typiano는 Rust로 작성되었습니다. Rust가 설치되어 있지 않다면:

```bash
# 방법 1: 공식 설치 도구 (권장)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 방법 2: Homebrew (macOS)
brew install rust
```

### 소스에서 빌드 (권장)

```bash
git clone https://github.com/khj68/typiano.git
cd typiano
cargo install --path .
```

### 수동 빌드

```bash
cargo build --release
# 바이너리: ./target/release/typiano
```

### Homebrew (준비 중)

```bash
brew tap khj68/typiano
brew install typiano
```

## 🚀 사용법

| 명령어 | 설명 |
|---------|------|
| `typiano on [--sound rhodes\|piano]` | daemon 시작 (랜덤 곡, 음원 선택) |
| `typiano off` | daemon 종료 |
| `typiano play <id>` | 특정 곡으로 변경 |
| `typiano list` | 수록곡 전체 목록 |
| `typiano status` | 현재 곡, 진행 상황, 모드 & 음원 |
| `typiano random` | 랜덤 곡으로 변경 |
| `typiano sound <rhodes\|piano>` | 음원 변경 |
| `typiano mode <random\|repeat>` | 곡 끝 동작 설정 |
| `typiano freeplay` | 건반 연주 모드 진입 |
| `typiano song` | 곡 모드로 복귀 |
| `typiano add <file>` | JSON 파일로 커스텀 곡 추가 |
| `typiano remove <id>` | 사용자가 추가한 곡 삭제 |

### 나만의 곡 추가하기

누구나 커스텀 곡을 추가할 수 있습니다:

```bash
typiano add my-song.json       # 추가
typiano remove my-song          # 삭제 (사용자가 추가한 곡만)
```

곡 JSON 형식:

```json
{
  "id": "my-song",
  "title": "My Song",
  "composer": "Me",
  "notes": ["C4", "E4", "G4", "C5", "E5", "G5", "C6"]
}
```

음표는 과학적 음높이 표기법을 사용합니다: `C2`부터 `C7`까지, 플랫 포함 (`Db`, `Eb`, `Gb`, `Ab`, `Bb`).

### MIDI에서 변환하기

정확한 멜로디 추출을 위한 MIDI 변환 도구가 포함되어 있습니다:

```bash
python3 tools/midi2typiano.py song.mid \
  --id my-song --title "My Song" --composer "Someone" \
  --max-notes 150
```

## 🎶 수록곡

실제 MIDI 파일에서 추출한 정확한 멜로디 29곡이 내장되어 있습니다:

| 작곡가 | 수록곡 |
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

## 🎹 사운드

Typiano는 **두 가지 음원**을 제공합니다:

- **Rhodes** (일렉트릭 피아노) — 깨끗하고 따뜻한 MIDI 톤 (기본값)
- **Piano** (어쿠스틱 피아노) — 밝은 어택, 풍부한 배음, 긴 서스테인

61건반 (C2 – C7), 하모닉 합성 기반 샘플에 자연스러운 페이드아웃 적용.

음원 변경: `typiano sound piano` 또는 `typiano sound rhodes`

## 🎹 건반 연주 모드

키보드를 피아노처럼 사용하세요! `typiano freeplay`로 진입:

```
상위 옥타브:   2    3         5    6    7              9    0
               C#4  D#4       F#4  G#4  A#4            C#5  D#5
             Q    W    E    R    T    Y    U    I    O    P
             C4   D4   E4   F4   G4   A4   B4   C5   D5   E5

하위 옥타브:   S    D         G    H    J
               C#3  D#3       F#3  G#3  A#3
             Z    X    C    V    B    N    M
             C3   D3   E3   F3   G3   A3   B3
```

곡 모드로 복귀: `typiano song`

## ⚙️ 동작 원리

```
typiano on  →  백그라운드 daemon 실행
                ├── rdev::listen   (전역 키 입력 감지)
                ├── rodio          (오디오 재생)
                └── Unix socket    (IPC 서버)

곡 모드:        키 입력  →  곡의 다음 음표  →  샘플 재생
건반 모드:      키 입력  →  매핑된 피아노 음  →  샘플 재생
```

1. `typiano on`은 백그라운드 daemon 프로세스를 실행합니다
2. daemon은 `rdev`를 통해 전역 키보드 이벤트를 감지합니다
3. **곡 모드**에서 키를 누를 때마다 곡이 진행되며 다음 피아노 음이 재생됩니다
4. 곡이 끝나면: **랜덤 모드**는 자동으로 다음 곡으로, **반복 모드**는 처음부터 반복
5. **건반 모드**에서는 키보드 자판이 피아노 건반에 매핑되어 자유 연주 가능
6. `typiano off`는 Unix domain socket을 통해 종료 명령을 보냅니다

## 🖥️ 시스템 요구사항

| 플랫폼 | 오디오 백엔드 | 키 입력 감지 |
|----------|--------------|-------------|
| **macOS** | CoreAudio | CGEventTap |
| **Linux** | ALSA / PulseAudio | evdev |

> **macOS**: 전역 키 입력 감지를 위해 손쉬운 사용 권한이 필요합니다.<br>
> **시스템 설정 → 개인정보 보호 및 보안 → 손쉬운 사용**에서 터미널 앱을 활성화하세요.

## 🏗️ 아키텍처

```
src/
├── main.rs      # CLI 진입점 (clap)
├── cli.rs       # 서브커맨드 핸들러
├── daemon.rs    # daemon 생명주기 (fork, PID, signal)
├── input.rs     # rdev 키 리스너 (Key 이벤트 전송)
├── engine.rs    # 곡 상태 머신, 재생/게임 모드, 키→음표 매핑
├── audio.rs     # rodio 재생, 듀얼 샘플 뱅크 (Rhodes/Piano)
├── ipc.rs       # Unix socket 서버/클라이언트
├── songs.rs     # 곡 구조체, 로더, 검증기
└── config.rs    # 경로 & 상태

tools/
├── midi2typiano.py      # MIDI → JSON 곡 변환기
└── generate_samples.sh  # 샘플 생성 스크립트 (Rhodes + Piano)
```

## 🤝 기여하기

기여는 언제나 환영합니다! 다음과 같은 방법으로 참여할 수 있습니다:

- 🎵 **곡 추가**: MIDI 파일을 `midi2typiano.py`로 변환하여 PR을 보내주세요
- 🐛 **버그 제보**: 이슈를 열어주세요
- 💡 **기능 제안**: `[Feature Request]` 태그와 함께 이슈를 열어주세요

### 새로운 곡 추가 방법

1. 해당 곡의 MIDI 파일을 구합니다
2. 변환: `python3 tools/midi2typiano.py song.mid --id song-id --title "Title" --composer "Composer"`
3. JSON 파일을 `songs/` 디렉토리에 넣습니다
4. `src/songs.rs`에 `include_str!` 라인을 추가합니다
5. PR을 제출합니다

## 📄 라이선스

[MIT](../LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*무언가를 타이핑하세요. 아름다운 소리가 들릴 거예요.*

</div>
