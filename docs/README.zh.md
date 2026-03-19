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

**把你的打字变成钢琴演奏。**

每一次击键都会演奏一首古典钢琴曲的下一个音符。<br>
写邮件、写代码、和朋友聊天——同时聆听肖邦、德彪西或巴赫的旋律。

🎵 *你的键盘，就是一座音乐厅。* 🎵

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

[English](../README.md) · [한국어](README.ko.md) · [日本語](README.ja.md) · **中文**

</div>

---

## 🎬 演示

```
$ typiano on
🎹 typiano started! Start typing to play piano.

$ typiano status
Now playing: Chopin - Nocturne Op.9 No.2
Song ID:     chopin-nocturne-9-2
Progress:    [████████░░░░░░░░░░░░] 63/150 notes (42%)

$ typiano list
Available songs (29):

   fur-elise                      Beethoven - Fur Elise
   chopin-nocturne-9-2            Chopin - Nocturne Op.9 No.2
   river-flows-in-you             Yiruma - River Flows in You
   ...

$ typiano random
Now playing: Ravel - Bolero

$ typiano off
typiano stopped.
```

## 📦 安装

### 从源码安装（推荐）

```bash
git clone https://github.com/khj68/typiano.git
cd typiano
cargo install --path .
```

### 手动构建

```bash
cargo build --release
# Binary: ./target/release/typiano
```

### Homebrew（即将推出）

```bash
brew tap khj68/typiano
brew install typiano
```

## 🚀 使用方法

| 命令 | 说明 |
|---------|-------------|
| `typiano on` | 启动 daemon（随机歌曲） |
| `typiano off` | 停止 daemon |
| `typiano play <id>` | 切换到指定歌曲 |
| `typiano list` | 显示所有可用歌曲 |
| `typiano status` | 查看当前歌曲和进度 |
| `typiano random` | 随机切换歌曲 |
| `typiano add <file>` | 从 JSON 文件添加自定义歌曲 |
| `typiano remove <id>` | 移除用户添加的歌曲 |

### 添加自定义歌曲

任何人都可以添加自定义歌曲：

```bash
typiano add my-song.json       # 添加
typiano remove my-song          # 移除（仅限用户添加的歌曲）
```

歌曲 JSON 格式：

```json
{
  "id": "my-song",
  "title": "My Song",
  "composer": "Me",
  "notes": ["C4", "E4", "G4", "C5", "E5", "G5", "C6"]
}
```

音符使用科学音高记谱法：从 `C2` 到 `C7`，降号表示为 `Db`、`Eb`、`Gb`、`Ab`、`Bb`。

### 从 MIDI 导入

内置 MIDI 转换器，可精准提取旋律：

```bash
python3 tools/midi2typiano.py song.mid \
  --id my-song --title "My Song" --composer "Someone" \
  --max-notes 150
```

## 🎶 内置曲目

29 首乐曲，基于真实 MIDI 文件转换，旋律精准还原：

| 作曲家 | 作品 |
|----------|--------|
| **Beethoven** | Für Elise, Moonlight Sonata, Pathétique (2nd mvt) |
| **Chopin** | Nocturne Op.9 No.2, Waltz Op.64 No.2, Prelude Op.28 No.4, Etude Op.10 No.3, Ballade No.1, Fantasie-Impromptu, Raindrop Prelude |
| **Debussy** | Clair de Lune, Arabesque No.1 |
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
| **Schumann** | Träumerei |
| **Grieg** | Morning Mood (Peer Gynt) |

## 🎹 音色

Typiano 使用 **Electric Piano (Rhodes)** 采样，通过 FluidSynth 从 General MIDI SoundFont 渲染而成。

- 61 个琴键：C2 – C7
- 干净、温暖的 MIDI 音色
- 2 秒采样，自然渐弱收尾

## ⚙️ 工作原理

```
typiano on  →  后台 daemon 进程启动
                ├── rdev::listen   (全局按键捕获)
                ├── rodio          (音频播放)
                └── Unix socket    (IPC 服务器)

击键  →  歌曲的下一个音符  →  播放电钢琴采样
```

1. `typiano on` 启动一个后台 daemon 进程
2. daemon 通过 `rdev` 捕获全局键盘事件
3. 每次按键推进歌曲进度，播放下一个钢琴音符
4. 歌曲结束后自动从头循环播放
5. `typiano off` 通过 Unix domain socket 发送关闭命令

## 🖥️ 系统要求

| 平台 | 音频后端 | 按键捕获 |
|----------|--------------|-------------|
| **macOS** | CoreAudio | CGEventTap |
| **Linux** | ALSA / PulseAudio | evdev |

> **macOS**：全局按键监听需要辅助功能权限。<br>
> 前往 **系统设置 → 隐私与安全性 → 辅助功能**，启用你的终端应用。

## 🏗️ 项目结构

```
src/
├── main.rs      # CLI 入口 (clap)
├── cli.rs       # 子命令处理
├── daemon.rs    # daemon 生命周期（fork、PID、信号）
├── input.rs     # rdev 按键监听
├── engine.rs    # 歌曲状态机（当前歌曲、音符索引、循环）
├── audio.rs     # rodio 播放、采样库
├── ipc.rs       # Unix socket 服务器/客户端
├── songs.rs     # 歌曲结构体、加载器、验证器
└── config.rs    # 路径与状态

tools/
├── midi2typiano.py      # MIDI → JSON 歌曲转换器
└── generate_samples.sh  # 采样生成脚本
```

## 🤝 参与贡献

欢迎贡献！以下是一些参与方式：

- 🎵 **添加歌曲**：使用 `midi2typiano.py` 转换 MIDI 文件并提交 PR
- 🐛 **报告 Bug**：提交 issue
- 💡 **建议新功能**：提交带有 `[Feature Request]` 标签的 issue

### 添加新歌曲

1. 找到该乐曲的 MIDI 文件
2. 转换：`python3 tools/midi2typiano.py song.mid --id song-id --title "Title" --composer "Composer"`
3. 将 JSON 文件放入 `songs/` 目录
4. 在 `src/songs.rs` 中添加 `include_str!` 行
5. 提交 PR

## 📄 许可证

[MIT](../LICENSE)

---

<div align="center">

Made with 🎹 and ❤️

*敲下键盘，聆听美妙旋律。*

</div>
