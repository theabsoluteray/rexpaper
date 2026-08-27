<p align="center">
  <img src="assets/logo.png" alt="RexPaper Logo" width="120" />
</p>

<h1 align="center">RexPaper</h1>

<p align="center">
  <strong>A fast, native wallpaper manager for Windows and Hyprland</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue?style=flat-square" alt="Version" />
  <img src="https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/license-GPL--3.0-green?style=flat-square" alt="License" />
  <img src="https://img.shields.io/badge/platform-windows%20%7C%20hyprland-lightgrey?style=flat-square" alt="Platform" />
</p>

<p align="center">
  <a href="#-features">Features</a> &bull;
  <a href="#-tech-stack">Tech Stack</a> &bull;
  <a href="#-roadmap">Roadmap</a> &bull;
  <a href="#-getting-started">Getting Started</a> &bull;
  <a href="#-license">License</a>
</p>

---

## Features

- **Static Wallpapers** — Manage and browse your image wallpaper collection
- **Live Wallpapers** — Support for animated/live wallpapers
- **Dark & Light Themes** — Full theming system with smooth animated transitions
- **Native Folder Picker** — OS-native directory selection via `rfd`
- **Startup Integration** — Run on system startup toggle
- **Fullscreen Detection** — Pause live wallpapers when a fullscreen app is running
- **Git Repository Support** — Clone wallpaper packs directly from Git repositories
- **Category System** — Organize wallpapers by categories

---

## Tech Stack

| Layer | Technology |
|---|---|
| **Language** | <img src="https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust" alt="Rust" /> Rust (Edition 2024) |
| **UI Framework** | <img src="https://img.shields.io/badge/Slint-1.17-purple?style=flat-square" alt="Slint" /> Slint — Declarative native GUI |
| **Async Runtime** | <img src="https://img.shields.io/badge/Tokio-1.40-darkred?style=flat-square" alt="Tokio" /> Tokio — Multi-threaded async runtime |
| **Image Processing** | <img src="https://img.shields.io/badge/image--rs-0.25-yellow?style=flat-square" alt="image-rs" /> image — Decoding & thumbnails |
| **Git Integration** | <img src="https://img.shields.io/badge/libgit2-0.20-red?style=flat-square" alt="libgit2" /> git2 — Repository cloning |
| **Serialization** | <img src="https://img.shields.io/badge/Serde-1.0-blue?style=flat-square" alt="Serde" /> Serde + serde_json |
| **File Dialog** | <img src="https://img.shields.io/badge/rfd-0.15-green?style=flat-square" alt="rfd" /> rfd — Native file dialogs |
| **Windows API** | <img src="https://img.shields.io/badge/windows--rs-0.61-blue?style=flat-square&logo=windows" alt="Windows" /> windows — DWM, GDI, Win32 |
| **Build System** | <img src="https://img.shields.io/badge/Cargo-stable-blue?style=flat-square&logo=rust" alt="Cargo" /> Cargo with LTO optimizations |

---

## Roadmap

### UI

- [x] Sidebar navigation with animated active states
- [x] Dark / Light theme system with 17+ semantic color tokens
- [x] Custom toggle switch component (replacing built-in Switch)
- [x] Static wallpapers page scaffold
- [x] Live wallpapers page scaffold
- [x] Settings page (Appearance, General, Wallpaper Storage)
- [ ] Responsive / resizable layout
- [ ] Wallpaper preview modal
- [ ] Drag-and-drop support

### Core

- [ ] Scan wallpaper directories recursively (`walkdir`)
- [ ] Generate and cache wallpaper thumbnails (`image`)
- [ ] Set desktop wallpaper at OS level (Win32 / Hyprland)
- [ ] Persistent settings via JSON config (`serde` / `dirs`)
- [ ] Async wallpaper loading (`tokio`)
- [ ] Category management (create, rename, delete)
- [ ] Git repository cloning for wallpaper packs (`git2`)
- [ ] Auto-start on system boot
- [ ] Pause on fullscreen application detection

### Polish

- [ ] Installation packages (MSI / installer)
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Unit & integration tests
- [ ] User documentation & wiki
- [ ] Localization / i18n support

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- CMake and a C compiler (required by `libgit2` / `git2-rs`)
- On Windows: Windows 10 SDK or later

### Build & Run

```bash
# Clone the repository
git clone https://github.com/theabsoluteray/rexpaper.git
cd rexpaper

# Build in release mode
cargo build --release

# Run
cargo run --release
```

### Project Structure

```
rexpaper/
├── assets/            # App icons and images
├── src/
│   └── main.rs        # Application entry point
├── ui/                # Slint UI definitions
│   ├── main.slint     # Root window layout
│   ├── store.slint    # Global state & theming
│   ├── navbar.slint   # Sidebar navigation
│   ├── static.slint   # Static wallpapers page
│   ├── live.slint     # Live wallpapers page
│   ├── settings.slint # Settings page
│   └── area.slint     # Reusable area component
├── build.rs           # Slint build script
└── Cargo.toml         # Dependencies & manifest
```

---

## License

This project is licensed under the **GNU General Public License v3.0** — see the [LICENSE](LICENSE) file for details.

<p align="center">
  Made with 🦕 by <a href="https://github.com/theabsoluteray">theabsoluteray</a>
</p>
