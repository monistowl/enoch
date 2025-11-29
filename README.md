# Enoch

> A modern, terminal-based engine for Enochian Chess (4-player variant).

![enoch-kitty.gif](screenshots/enoch-kitty.gif)

**[📖 Read the Full Documentation](https://monistowl.github.io/enoch/)**

## 🌟 Overview

**Enoch** is a feature-rich engine and Terminal User Interface (TUI) for **Enochian Chess**, a complex four-army variant derived from the Golden Dawn tradition. Built in Rust, it enforces strict rules, supports historical setups (Tablets), and offers a responsive, interactive experience in your terminal.

Unlike standard chess, Enochian Chess features:
- **Four Armies**: Blue, Red, Black, and Yellow battling in teams of two.
- **Unique Pieces**: Queens that leap (Alibaba), Bishops restricted to diagonal networks.
- **Thrones**: Capture thrones to control allied armies.
- **Divination Mode**: Use dice rolls to constrain legal moves.

## ✨ Features

- **Interactive TUI**: Play directly in your terminal with mouse-free controls.
- **Dual Modes**:
  - **Board Mode**: Navigate and move pieces with arrow keys.
  - **Command Mode**: Enter moves using strict notation (e.g., `blue: e2-e4`).
- **Responsive Design**: Adapts layout for compact or wide terminal windows (min 80x24).
- **Rule Enforcement**: Validates move legality, turns, promotion zones, and frozen armies.
- **Save/Load**: Persist your games to JSON.
- **Divination Support**: Integrated dice rolling mechanics for divination play.

## 📦 Installation

### Pre-built Binaries
Download the latest release from the [Releases Page](https://github.com/monistowl/enoch/releases).

```bash
tar -xzvf enoch-linux-x86_64.tar.gz
./enoch
```

### From Source (Rust)
Ensure you have [Rust](https://www.rust-lang.org/) (1.80+) installed.

```bash
git clone https://github.com/monistowl/enoch.git
cd enoch
cargo install --path .
enoch
```

## 🕹️ Controls

### Board Mode (Interactive)
*Active by default on start.*
- **Arrow Keys**: Move cursor.
- **Enter**: Select piece / Confirm move.
- **Esc**: Cancel selection.
- **Tab** or **/**: Switch to Command Mode.

### Command Mode (Text)
- **Type command**: `/help`, `/new`, `/save game.json`.
- **Manual Move**: `color: from-to` (e.g., `red: h7-h6`).
- **Tab** or **Esc**: Switch back to Board Mode.

## 🛠️ Configuration

**Rendering Modes:**
- `enoch`: Default rendering (best for Kitty, Alacritty).
- `enoch --halfblocks`: Use if you experience flickering or vertical spacing issues (e.g., iTerm2).

## 📖 Documentation

Complete rules, strategy guides, and historical context are available at the [Official Site](https://monistowl.github.io/enoch/).

## 🤝 Contributing

Contributions are welcome! Please check `AGENTS.md` for our development workflow using `bd` for issue tracking.

## 📄 License

MIT License.