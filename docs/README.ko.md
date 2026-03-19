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

## 📦 설치

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

```bash
typiano on              # daemon 시작 (랜덤 곡)
typiano off             # daemon 종료
typiano play <song-id>  # 특정 곡으로 변경
typiano list            # 수록곡 전체 목록
typiano status          # 현재 곡 & 진행 상황
typiano random          # 랜덤 곡으로 변경
```

### 나만의 곡 추가하기

누구나 커스텀 곡을 추가할 수 있습니다:

```bash
# JSON 파일로 곡 추가
typiano add my-song.json

# 사용자가 추가한 곡 삭제
typiano remove my-song
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

## 🎶 수록곡

퍼블릭 도메인 클래식 피아노 20곡이 내장되어 있습니다:

| 작곡가 | 수록곡 |
|----------|--------|
| **Beethoven** | Für Elise, Moonlight Sonata, Pathétique (2nd mvt) |
| **Chopin** | Nocturne Op.9 No.2, Waltz Op.64 No.2, Prelude Op.28 No.4, Etude Op.10 No.3, Ballade No.1 |
| **Debussy** | Clair de Lune, Arabesque No.1, Rêverie |
| **Bach** | Prelude in C Major BWV 846, Two-Part Invention No.1 |
| **Mozart** | Turkish March (Rondo alla Turca) |
| **Satie** | Gymnopédie No.1, Gnossienne No.1 |
| **Liszt** | Liebesträum No.3 |
| **Schumann** | Träumerei |
| **Grieg** | Morning Mood (Peer Gynt) |
| **Schubert** | Impromptu Op.90 No.3 |

## ⚙️ 동작 원리

```
typiano on  →  백그라운드 daemon 실행
                ├── rdev::listen   (전역 키 입력 감지)
                ├── rodio          (오디오 재생)
                └── Unix socket    (IPC 서버)

키 입력  →  곡의 다음 음표  →  피아노 샘플 재생
```

1. `typiano on`은 백그라운드 daemon 프로세스를 실행합니다
2. daemon은 `rdev`를 통해 전역 키보드 이벤트를 감지합니다
3. 키를 누를 때마다 곡이 진행되며 다음 피아노 음이 재생됩니다
4. 곡이 끝나면 처음부터 다시 반복합니다
5. `typiano off`는 Unix domain socket을 통해 종료 명령을 보냅니다

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
├── input.rs     # rdev 키 리스너
├── engine.rs    # 곡 상태 머신 (현재 곡, 음표 인덱스, 반복)
├── audio.rs     # rodio 재생, 샘플 뱅크, crossfade
├── ipc.rs       # Unix socket 서버/클라이언트
├── songs.rs     # 곡 구조체, 로더, 검증기
└── config.rs    # 경로 & 상태
```

## 🤝 기여하기

기여는 언제나 환영합니다! 다음과 같은 방법으로 참여할 수 있습니다:

- 🎵 **곡 추가**: JSON 파일을 만들어 `songs/` 디렉토리에 PR을 보내주세요
- 🐛 **버그 제보**: 이슈를 열어주세요
- 💡 **기능 제안**: `[Feature Request]` 태그와 함께 이슈를 열어주세요

### 새로운 곡 추가 방법

1. [곡 형식](#나만의-곡-추가하기)에 맞춰 JSON 파일을 만듭니다
2. `songs/` 디렉토리에 넣습니다
3. `src/songs.rs`에 `include_str!` 라인을 추가합니다
4. PR을 제출합니다

## 📄 라이선스

[MIT](../LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*무언가를 타이핑하세요. 아름다운 소리가 들릴 거예요.*

</div>
