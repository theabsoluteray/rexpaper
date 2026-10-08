<p align="center">
  <img src="assets/logo.png" alt="RexPaper Logo" width="128" height="128" />
</p>

<h1 align="center">RexPaper</h1>

<p align="center">
  <strong>A high-performance wallpaper manager for Windows with hardware-accelerated live video wallpapers.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.3-blue?style=flat-square" alt="Version" />
  <img src="https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square&logo=rust" alt="Rust Edition 2024" />
  <img src="https://img.shields.io/badge/tauri-v2-24C8DB?style=flat-square&logo=tauri" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/license-GPL--3.0-green?style=flat-square" alt="License GPL-3.0" />
  <img src="https://img.shields.io/badge/platform-windows%2010%20%7C%2011%20(x64)-0078D6?style=flat-square&logo=windows" alt="Platform Windows" />
  <img src="https://img.shields.io/badge/mpv-Direct3D11%20%2F%20GPU-darkblue?style=flat-square" alt="Direct3D11" />
</p>

<p align="center">
  <a href="#overview">Overview</a> &bull;
  <a href="#features">Features</a> &bull;
  <a href="#supported-formats">Supported Formats</a> &bull;
  <a href="#tech-stack">Tech Stack</a> &bull;
  <a href="#getting-started">Getting Started</a> &bull;
  <a href="#project-structure">Project Structure</a> &bull;
  <a href="#roadmap">Roadmap</a> &bull;
  <a href="#license">License</a>
</p>

---

## Overview

**RexPaper** is a fast, lightweight wallpaper manager engineered specifically for Windows 10 and 11. The backend is pure **Rust (Edition 2024)** — Win32 desktop window manipulation, thumbnail pipeline, tray, and settings — while the UI is a minimal **Preact + Tailwind** app running in a **Tauri v2** WebView.

It manages thousands of ultra-high-resolution 4K/8K static wallpapers or renders seamless 60+ FPS live video wallpapers behind your desktop icons, with minimal resource usage and zero UI stutter.

> **Status:** the Slint UI has been removed; the Tauri UI is being built up page by page. See `docs/specs/tauri-v2-migration.md`.

---

## Features

### High-Performance Static Wallpaper Gallery
- **Recursive Directory Scanning**: Scans custom wallpaper directories recursively and indexes entire collections in milliseconds.
- **Dynamic Category Organization**: Automatically groups wallpapers into clean categories based on subfolder structure (e.g., `Wallpapers/Anime/pic.jpg` &rarr; `Anime`).
- **Real-Time Instant Search**: Live text-filtering across titles and categories as you type.
- **Native Win32 Wallpaper Application**: Applies static wallpapers directly via `SystemParametersInfoW(SPI_SETDESKWALLPAPER)` with instant desktop refresh.

### Hardware-Accelerated Live Video Wallpapers
- **GPU-Accelerated Video Engine**: Direct3D11 / GPU-powered rendering (`--vo=gpu`, `--hwdec=auto-safe`) delivering butter-smooth 60+ FPS playback with near-zero CPU consumption.
- **WorkerW Desktop Canvas Injection**: Injects video playback seamlessly behind Windows desktop icons (`SHELLDLL_DefView`) using `Progman` shell message `0x052C` and `WorkerW` reparenting.
- **Seamless Two-Way Transitions**: Live &rarr; static terminates `mpv` and hides `WorkerW`; static &rarr; live reveals `WorkerW` and attaches playback to `--wid=<hwnd>`.
- **Live Wallpaper Controls**: Start, pause, resume, and stop desktop live wallpapers at any time.

### Multi-Core Background Precomputation & Disk Caching
- **Parallel Multi-Core Resizing**: Uses Rayon (`rayon::par_iter()`) across all CPU threads to precompute thumbnails on background threads.
- **GPU-Accelerated Frame Extraction**: Captures video frames at 0.5s via `mpv` (`--vo=image`, `--hwdec=auto-safe`) for instantaneous video browsing.
- **Persistent Disk Caching**: Caches 384&times;216 px thumbnails in `%LOCALAPPDATA%/rexpaper/cache` using 64-bit cryptographic hashing for sub-millisecond lookup.
- **Crash-Proof Fault Tolerance**: All decoding and thumbnail generation is wrapped in `std::panic::catch_unwind` with dark placeholder fallbacks for corrupt files.

### Deep Windows System Integration
- **System Tray Integration**: Background system tray with context menu (Open, Static, Live, Settings, Quit).
- **Per-Monitor V2 HiDPI Awareness**: Razor-sharp rendering at every display scaling factor.
- **Silent Windows Autostart**: Registry integration (`HKCU\...\CurrentVersion\Run`) with `--autostart` / `--minimized` flags.

### Configurable Settings & Persistence
- **Persistent JSON Configuration**: Settings saved via Serde (shared with the v1 install).
- **Native Folder Pickers**: Windows folder pickers via `rfd` for static and live wallpaper library locations.
- **Configurable Toggles**: Run on startup, pause on fullscreen, mute live wallpapers.

---

## Supported Formats

| Media Type | Supported Formats / Extensions |
|---|---|
| **Static Images** | `.png`, `.jpg`, `.jpeg`, `.webp`, `.bmp`, `.gif`, `.avif`, `.tiff`, `.tif` |
| **Live Videos** | `.mp4`, `.webm`, `.mkv`, `.mov`, `.avi`, `.flv`, `.m4v`, `.gif`, `.mpg`, `.mpeg`, `.wmv` |

---

## Tech Stack

| Component | Technology | Description |
|---|---|---|
| **Language** | <img src="https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust" alt="Rust" /> Rust (Edition 2024) | High-performance, memory-safe systems programming |
| **App Shell** | <img src="https://img.shields.io/badge/Tauri-v2-24C8DB?style=flat-square&logo=tauri" alt="Tauri" /> Tauri v2 | WebView shell, typed commands, events, NSIS bundling |
| **UI** | <img src="https://img.shields.io/badge/Preact-+Vite-673AB7?style=flat-square" alt="Preact" /> Preact + Vite + TypeScript | Minimal SPA with Preact Signals state |
| **Styling** | <img src="https://img.shields.io/badge/Tailwind_CSS-v4-38B2AC?style=flat-square" alt="Tailwind" /> Tailwind CSS v4 | Flat, hairline-bordered design language |
| **Core Library** | <img src="https://img.shields.io/badge/rexpaper--core-workspace-orange?style=flat-square" alt="rexpaper-core" /> `crates/rexpaper-core` | UI-free backend: scanner, settings, wallpaper, tray, thumbnails |
| **Video Engine** | <img src="https://img.shields.io/badge/mpv-libmpv-darkblue?style=flat-square" alt="mpv" /> libmpv & mpv (D3D11) | Hardware-accelerated playback and frame extraction |
| **Win32 OS APIs** | <img src="https://img.shields.io/badge/windows--rs-0.61-blue?style=flat-square&logo=windows" alt="windows-rs" /> windows-rs (0.61) | `WorkerW` injection, `Shell_NotifyIconW`, HiDPI |
| **Dialogs** | <img src="https://img.shields.io/badge/rfd-0.15-green?style=flat-square" alt="rfd" /> rfd (0.15) | Native Windows folder pickers |
| **Packaging** | <img src="https://img.shields.io/badge/NSIS-via%20Tauri-orange?style=flat-square" alt="NSIS" /> NSIS (Tauri bundler) | Windows installer |

---

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2024 / 1.85+)
- [Node.js](https://nodejs.org/) 20+
- Windows 10 SDK or later
- mpv runtime (`mpv/`) and import library (`mpv-lib/mpv.lib`, or set `MPV_LIB_DIR`) — required to link and run

### Build and Run

```powershell
# Clone the repository
git clone https://github.com/theabsoluteray/rexpaper.git
cd rexpaper

# Install frontend dependencies (first time)
npm install

# Development mode (Vite dev server + Tauri window)
npm run tauri dev

# Production build (NSIS installer + portable bundle)
npm run tauri build
```

`cargo build` on its own builds the workspace (core + app), but the app needs the Vite dev server or a built `frontend/dist` to render — use `npm run tauri dev`.

---

## Project Structure

```
rexpaper/
├── assets/
│   ├── icon.ico             # Application icon (tray + Tauri)
│   └── logo.png             # RexPaper brand logo
├── crates/
│   └── rexpaper-core/       # UI-free Rust backend library
│       ├── build.rs         # mpv link-search + runtime DLL staging
│       └── src/
│           ├── scanner.rs       # Recursive filesystem wallpaper scanner
│           ├── thumbnail.rs     # Multi-core thumbnail precompute & disk cache
│           ├── settings.rs      # JSON settings + autostart registry
│           ├── static_wallpaper.rs  # Win32 wallpaper application
│           ├── live_wallpaper.rs    # Live wallpaper coordinator
│           ├── mpv_player.rs        # libmpv bindings
│           └── platform/           # WorkerW injection + system tray
├── src-tauri/               # Tauri v2 app (commands, events, config)
│   ├── src/main.rs          # Entry point, command registration
│   ├── src/commands.rs      # Typed command layer over rexpaper-core
│   └── tauri.conf.json      # Window, dev server, NSIS bundle config
├── frontend/                # Preact + Vite + TypeScript + Tailwind
│   └── src/                 # App, pages, typed API mirror
├── mpv/                     # mpv runtime binaries & DLLs (local)
├── mpv-lib/                 # mpv.lib import library (local)
├── docs/specs/              # Design specs
└── LICENSE                  # GNU General Public License v3.0
```

---

## Roadmap

- [x] **Core extraction** &mdash; UI-free `rexpaper-core` workspace crate.
- [x] **Tauri v2 + Preact scaffold** &mdash; typed commands, minimal settings/scan UI.
- [ ] **Static gallery page** &mdash; grid, categories, search, lazy thumbnails, apply.
- [ ] **Live gallery page** &mdash; video thumbnails and playback controls.
- [ ] **Settings page** &mdash; directory pickers, toggles, tray + autostart wiring.
- [ ] **NSIS packaging + release CI** &mdash; `tauri build` installers.
- [ ] **Multi-Monitor Wallpaper Assignment** &mdash; Assign independent wallpapers per display monitor.
- [ ] **Granular Live Audio Volume Slider** &mdash; In-app slider to adjust live wallpaper volume levels.

---

## License

This project is licensed under the **GNU General Public License v3.0** &mdash; see the [LICENSE](LICENSE) file for details.

<p align="center">
  Made by <a href="https://github.com/theabsoluteray">theabsoluteray</a>
</p>
