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

**あなたのタイピングをピアノ演奏に変える。**

キーを打つたびに、クラシックピアノ曲の次の音が鳴ります。<br>
メールを書いても、コードを書いても、友達とチャットしても — Chopin、Debussy、Bachの演奏が響きます。

🎵 *あなたのキーボードがコンサートホールに。* 🎵

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

[English](../README.md) · [한국어](README.ko.md) · **日本語** · [中文](README.zh.md)

</div>

---

## 🎬 デモ

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

## 📦 インストール

### 前提条件

TypianoはRustで構築されています。Rustがインストールされていない場合：

```bash
# 方法1: 公式インストーラー（推奨）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 方法2: Homebrew (macOS)
brew install rust
```

### ソースからビルド（推奨）

```bash
git clone https://github.com/khj68/typiano.git
cd typiano
cargo install --path .
```

### 手動ビルド

```bash
cargo build --release
# Binary: ./target/release/typiano
```

### Homebrew（近日対応予定）

```bash
brew tap khj68/typiano
brew install typiano
```

## 🚀 使い方

| コマンド | 説明 |
|---------|------|
| `typiano on [--sound rhodes\|piano]` | daemon を起動（ランダム曲、音源選択） |
| `typiano off` | daemon を停止 |
| `typiano play <id>` | 指定した曲に切り替え |
| `typiano list` | 利用可能な全曲を表示 |
| `typiano status` | 現在の曲、進行状況、モード＆音源 |
| `typiano random` | ランダムな曲に切り替え |
| `typiano sound <rhodes\|piano>` | 音源を変更 |
| `typiano mode <random\|repeat>` | 曲終了時の動作を設定 |
| `typiano freeplay` | フリープレイモードに切り替え |
| `typiano song` | 曲モードに戻る |
| `typiano add <file>` | JSONからカスタム曲を追加 |
| `typiano remove <id>` | ユーザーが追加した曲を削除 |

### 自分の曲を追加する

誰でもカスタム曲を追加できます：

```bash
typiano add my-song.json       # 追加
typiano remove my-song          # 削除（ユーザー追加曲のみ）
```

曲のJSONフォーマット：

```json
{
  "id": "my-song",
  "title": "My Song",
  "composer": "Me",
  "notes": ["C4", "E4", "G4", "C5", "E5", "G5", "C6"]
}
```

音名は科学的音高表記を使用します：`C2` から `C7`、フラットは `Db`、`Eb`、`Gb`、`Ab`、`Bb` です。

### MIDIからインポート

正確なメロディ抽出のための MIDI コンバーターが付属しています：

```bash
python3 tools/midi2typiano.py song.mid \
  --id my-song --title "My Song" --composer "Someone" \
  --max-notes 150
```

## 🎶 内蔵曲

実際のMIDIファイルから抽出した正確なメロディ、全29曲：

| 作曲家 | 曲目 |
|--------|------|
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

## 🎹 サウンド

Typiano は**2種類の音源**を搭載しています：

- **Rhodes**（エレクトリックピアノ）— クリーンで温かみのあるMIDIトーン（デフォルト）
- **Piano**（アコースティックピアノ）— 明るいアタック、豊かな倍音、長いサスティン

61鍵 (C2 – C7)、倍音合成によるサンプル、自然なフェードアウト付き。

音源切替：`typiano sound piano` または `typiano sound rhodes`

## 🎹 フリープレイモード

キーボードをピアノとして使えます！`typiano freeplay` で開始：

```
上オクターブ:   2    3         5    6    7              9    0
                C#4  D#4       F#4  G#4  A#4            C#5  D#5
              Q    W    E    R    T    Y    U    I    O    P
              C4   D4   E4   F4   G4   A4   B4   C5   D5   E5

下オクターブ:   S    D         G    H    J
                C#3  D#3       F#3  G#3  A#3
              Z    X    C    V    B    N    M
              C3   D3   E3   F3   G3   A3   B3
```

曲モードに戻る：`typiano song`

## ⚙️ 仕組み

```
typiano on  →  バックグラウンド daemon が起動
                ├── rdev::listen   (グローバルキー取得)
                ├── rodio          (オーディオ再生)
                └── Unix socket    (IPC サーバー)

曲モード:       キー入力  →  曲の次の音  →  サンプル再生
フリープレイ:   キー入力  →  マッピングされたピアノ音  →  サンプル再生
```

1. `typiano on` がバックグラウンドで daemon プロセスを起動します
2. daemon が `rdev` を通じてグローバルキーボードイベントを取得します
3. **曲モード**ではキーを押すたびに曲が進み、次のピアノの音が再生されます
4. 曲が終わると：**ランダムモード**は自動で次の曲へ、**リピートモード**は最初からループ
5. **フリープレイモード**ではキーがピアノの音にマッピングされ、自由演奏が可能
6. `typiano off` が Unix domain socket 経由でシャットダウンコマンドを送信します

## 🖥️ 動作環境

| プラットフォーム | オーディオバックエンド | キー取得方式 |
|------------------|------------------------|--------------|
| **macOS** | CoreAudio | CGEventTap |
| **Linux** | ALSA / PulseAudio | evdev |

> **macOS**: グローバルキー監視にはアクセシビリティ権限が必要です。<br>
> **システム設定 → プライバシーとセキュリティ → アクセシビリティ** でターミナルアプリを有効にしてください。

## 🏗️ アーキテクチャ

```
src/
├── main.rs      # CLI エントリーポイント (clap)
├── cli.rs       # サブコマンドハンドラ
├── daemon.rs    # daemon ライフサイクル (fork, PID, signal)
├── input.rs     # rdev キーリスナー (Keyイベント送信)
├── engine.rs    # 曲のステートマシン、再生/ゲームモード、キー→音符マッピング
├── audio.rs     # rodio 再生、デュアルサンプルバンク (Rhodes/Piano)
├── ipc.rs       # Unix socket サーバー/クライアント
├── songs.rs     # 曲の構造体, ローダー, バリデーター
└── config.rs    # パスと状態

tools/
├── midi2typiano.py      # MIDI → JSON 曲コンバーター
└── generate_samples.sh  # サンプル生成スクリプト (Rhodes + Piano)
```

## 🤝 コントリビュート

コントリビューション大歓迎です！以下のような方法で貢献できます：

- 🎵 **曲を追加**: MIDIファイルを `midi2typiano.py` で変換してPRを送ってください
- 🐛 **バグ報告**: Issueを作成してください
- 💡 **機能提案**: `[Feature Request]` 付きでIssueを作成してください

### 新しい曲の追加方法

1. その曲のMIDIファイルを入手する
2. 変換: `python3 tools/midi2typiano.py song.mid --id song-id --title "Title" --composer "Composer"`
3. JSONを `songs/` に配置
4. `src/songs.rs` に `include_str!` の行を追加
5. PRを送信

## 📄 ライセンス

[MIT](../LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*何かタイプしてみてください。美しい音色が聴こえます。*

</div>
