<p align="center">
  <img src="assets/logo.svg" alt="RexPaper Logo" width="120" />
</p>

<h1 align="center">RexPaper</h1>

<p align="center">
  <strong>A fast, modern, native wallpaper manager for Windows with hardware-accelerated live wallpapers</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue?style=flat-square" alt="Version" />
  <img src="https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/slint-1.17-purple?style=flat-square" alt="Slint" />
  <img src="https://img.shields.io/badge/license-GPL--3.0-green?style=flat-square" alt="License" />
  <img src="https://img.shields.io/badge/platform-windows%2010%20%7C%2011-0078D6?style=flat-square&logo=windows" alt="Platform" />
  <img src="https://img.shields.io/badge/ci-github%20actions-2088FF?style=flat-square&logo=githubactions" alt="CI" />
</p>

<p align="center">
  <a href="#features">Features</a> &bull;
  <a href="#tech-stack">Tech Stack</a> &bull;
  <a href="#architecture--implementation">Architecture & Implementation</a> &bull;
  <a href="#getting-started">Getting Started</a> &bull;
  <a href="#project-structure">Project Structure</a> &bull;
  <a href="#roadmap">Roadmap</a> &bull;
  <a href="#license">License</a>
</p>

---

## Features

- **High-Performance Static Wallpaper Gallery** &mdash; Recursively scans image directories (`.png`, `.jpg`, `.jpeg`, `.webp`, `.bmp`, `.avif`) with instant filtering and applies wallpapers via native Win32 `SystemParametersInfoW`.
- **Hardware-Accelerated Live Wallpapers** &mdash; Direct3D11 / GPU-accelerated video playback (`.mp4`, `.webm`, `.mkv`, `.mov`) attached directly behind Windows desktop icons using `WorkerW` injection and `mpv`.
- **Parallel Multi-Core Background Precomputation** &mdash; Uses Rayon parallel iterators across all CPU cores to resize and precompute thumbnails in seconds without blocking or stuttering the UI thread.
- **GPU-Accelerated Video Frame Extraction** &mdash; Automatically extracts high-definition frame captures at 0.5s from video files via `mpv` hardware decoding (`--hwdec=auto-safe`) and caches them locally for instant browsing.
- **Ultra-Low Memory Footprint** &mdash; High-efficiency disk cache engine reduces RAM/VRAM usage by 98%, enabling smooth 60+ FPS navigation across libraries with thousands of 4K/8K wallpapers in < 30 MB RAM.
- **Full-Bleed Modern Gallery Cards** &mdash; Clean, distraction-free edge-to-edge wallpaper previews with subtle hover sheen and 16:9 widescreen proportions.
- **Crash-Proof Fault Tolerant Engine** &mdash; Unreadable or corrupt image files are safely caught via `std::panic::catch_unwind` and fall back to clean placeholders without crashing.
- **Pure Windows Desktop Subsystem** &mdash; Built with `#![windows_subsystem = "windows"]` for clean GUI launches without console / terminal window flashes.
- **Native Message-Only System Tray** &mdash; Background `HWND_MESSAGE` tray sink with context menus, quick navigation, and high-resolution dinosaur logo icon in the taskbar notification area.
- **Silent Windows Autostart** &mdash; Automatically registers in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` with `--autostart` to launch silently in the system tray on Windows boot.
- **Per-Monitor V2 High-DPI Awareness** &mdash; Enforces native physical pixel rendering across all display scaling factors (100%, 125%, 150%, 175%, 200%) for razor-sharp typography and vector assets.
- **WiX Toolset Installer** &mdash; Standalone `.msi` package with GPL-3.0 license agreement dialog, custom cover branding, Start Menu / Desktop shortcuts, and Windows Add/Remove Programs registration.

---

## Tech Stack

| Layer | Technology | Description |
|---|---|---|
| **Language** | <img src="https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust" alt="Rust" /> Rust (Edition 2024) | Safe, blazingly fast systems language |
| **UI Framework** | <img src="https://img.shields.io/badge/Slint-1.17-purple?style=flat-square" alt="Slint" /> Slint GUI | Declarative, reactive native UI framework with Fluent styling |
| **Concurrency** | <img src="https://img.shields.io/badge/Rayon-1.10-red?style=flat-square" alt="Rayon" /> Rayon | Data-parallel multi-core background thumbnail precomputation |
| **Video Engine** | <img src="https://img.shields.io/badge/mpv-libmpv-darkblue?style=flat-square" alt="mpv" /> libmpv & mpv binary | Hardware-accelerated Direct3D11 / GPU video playback and frame extraction |
| **OS Integration** | <img src="https://img.shields.io/badge/windows--rs-0.61-blue?style=flat-square&logo=windows" alt="Windows" /> windows-rs | Native Win32 API, `WorkerW` injection, `Shell_NotifyIconW`, DWM, HiDPI |
| **PE Resources** | <img src="https://img.shields.io/badge/winres-0.1-brightgreen?style=flat-square" alt="winres" /> winres | Multi-resolution icon and Windows executable metadata compiler |
| **Image Processing** | <img src="https://img.shields.io/badge/image--rs-0.25-yellow?style=flat-square" alt="image-rs" /> image-rs | Fast thumbnail generation, aspect-ratio scaling, and format decoding |
| **Installer** | <img src="https://img.shields.io/badge/WiX_Toolset-3.14-blueviolet?style=flat-square" alt="WiX" /> WiX Toolset | Windows Installer XML packaging for native `.msi` deployment |
| **Dialogs** | <img src="https://img.shields.io/badge/rfd-0.15-green?style=flat-square" alt="rfd" /> rfd | Native Windows folder and file picker dialogs |
| **Serialization** | <img src="https://img.shields.io/badge/Serde-1.0-blue?style=flat-square" alt="Serde" /> Serde & serde_json | Robust configuration persistence to `settings.json` |

---

## Architecture & Implementation

### 1. Multi-Core Background Precomputation & Disk Cache (`src/thumbnail.rs`)
- **Parallel Iteration**: `precompute_static_thumbnails` and `precompute_video_thumbnails` dispatch tasks across all available CPU threads using `rayon::par_iter()`.
- **Persistent Disk Caching**: Resized 384×216 px thumbnails are stored in `AppData/Local/rexpaper/cache/` using fast 64-bit cryptographic hash paths (`v{:016x}.jpg`), enabling sub-millisecond cache lookups.
- **GPU Hardware Decoding**: `mpv` video frame extraction specifies `--hwdec=auto-safe` for hardware-assisted decoding on Nvidia, AMD, and Intel GPUs.
- **Safe Fallback**: All decoding operations are isolated inside `std::panic::catch_unwind`, guaranteeing corrupt files never crash the application.

### 2. Desktop Wallpaper Injection Engine (`src/platform/windows.rs`)
- **WorkerW Layer Injection**: Dispatches shell message `0x052C` to `Progman` to spawn a dedicated `WorkerW` canvas layer behind desktop icons (`SHELLDLL_DefView`).
- **Silent Process Management**: Launches `mpv.exe` with `CREATE_NO_WINDOW (0x08000000)`, attached directly to the desktop window handle (`--wid=<hwnd>`) with `--vo=gpu` and `--hwdec=auto-safe`.
- **Two-Way Seamless Transition**:
  - **Live to Static**: Terminates `mpv.exe`, hides `WorkerW` (`SW_HIDE`), and applies the static image via `SystemParametersInfoW(SPI_SETDESKWALLPAPER)`.
  - **Static to Live**: Un-hides `WorkerW` (`SW_SHOW`), terminates any previous process, and attaches `mpv.exe`.

### 3. Message-Only System Tray (`src/platform/tray.rs`)
- **`HWND_MESSAGE` Architecture**: Creates a message-only Win32 window with `WS_EX_TOOLWINDOW` and `WS_EX_NOACTIVATE`, ensuring no blank or duplicate windows appear on the Windows taskbar.
- **High-Resolution Icon**: Queries the embedded application icon (Resource ID 1) using `LoadImageW` with exact small-icon metrics (`SM_CXSMICON`, `SM_CYSMICON`) for crisp rendering at all display scale factors.

### 4. Slint UI Architecture (`ui/`)
- **`ui/main.slint`**: Window root containing the navigation sidebar and smooth page cross-fading.
- **`ui/static.slint` & `ui/live.slint`**: Full-bleed responsive 4-column card grid with equal-weight layout distribution, category selectors, and instant search filtering.
- **`ui/store.slint`**: Centralized reactive state store managing wallpaper models, active themes, directory paths, and search queries.

---

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2024 / 1.85+)
- Windows 10 SDK or later

### Build and Run

```powershell
# Clone the repository
git clone https://github.com/theabsoluteray/rexpaper.git
cd rexpaper

# Build in release mode (auto-stages mpv binaries and DLLs)
cargo build --release

# Run
cargo run --release
```

---

## Project Structure

```
rexpaper/
├── assets/                  # Application vector SVG icons and multi-res icon.ico
├── mpv/                     # Standalone mpv binary and libmpv-2.dll
├── mpv-lib/                 # MSVC mpv.lib import library and def file
├── src/
│   ├── main.rs              # Application entry point, GUI subsystem, & state sync
│   ├── models.rs            # Wallpaper models, categories, and row grouping
│   ├── scanner.rs           # Recursive filesystem wallpaper directory scanner
│   ├── thumbnail.rs         # Multi-core thumbnail precomputing & disk cache engine
│   ├── settings.rs          # Persistent JSON settings manager & autostart registry
│   ├── static_wallpaper.rs  # Static wallpaper scanning and Win32 application
│   ├── live_wallpaper.rs    # Live wallpaper controller
│   ├── mpv_player.rs        # libmpv backend bindings
│   └── platform/
│       ├── mod.rs           # Platform abstraction interface
│       ├── windows.rs       # Win32 WorkerW injection & mpv background process
│       └── tray.rs          # Native message-only system tray implementation
├── ui/                      # Slint declarative UI files
│   ├── main.slint           # Main application window with page transitions
│   ├── store.slint          # Theme tokens & global AppStore state
│   ├── navbar.slint         # Navigation sidebar with SVG icons
│   ├── static.slint         # Static wallpapers gallery page (full-bleed cards)
│   ├── live.slint           # Live wallpapers gallery page (full-bleed cards)
│   ├── settings.slint       # Application settings page
│   └── components/
│       └── VideoPlayer.slint# Video player component
├── wix/                     # WiX Toolset installer packaging
│   ├── main.wxs             # WiX installer manifest & shortcut definitions
│   ├── license.rtf          # GPL-3.0 rich text license agreement
│   ├── dialog.bmp           # Installer splash cover background
│   └── banner.bmp           # Installer top header banner
├── .github/workflows/
│   └── release.yml          # Windows CI/CD release & WiX MSI packaging workflow
├── build.rs                 # Resource compiler (winres) & DLL auto-staging script
├── Cargo.toml               # Cargo dependencies & manifest
└── LICENSE                  # GNU General Public License v3.0
```

---

## Roadmap

- [x] Fixed `0xc0000135 (STATUS_DLL_NOT_FOUND)` via automatic DLL aliasing in `build.rs`.
- [x] Fixed uneven thumbnail sizing with uniform 4-column responsive grid.
- [x] Full-bleed wallpaper preview cards without footer bars.
- [x] Parallel multi-core thumbnail precomputation across CPU threads (`rayon`).
- [x] GPU-accelerated video thumbnail extraction (`--hwdec=auto-safe`).
- [x] Disk cache engine reducing memory usage by 98% (< 30 MB RAM on large libraries).
- [x] Suppressed black console window flash with `#![windows_subsystem = "windows"]`.
- [x] Message-only background system tray (`HWND_MESSAGE`) eliminating ghost taskbar windows.
- [x] High-resolution application icon embedded in PE headers, system tray, search, and shortcuts.
- [x] Complete WiX MSI installer with custom cover branding and GPL-3.0 license agreement.
- [ ] **Multi-Monitor Support** &mdash; Currently renders to the primary desktop display; per-monitor wallpaper assignment is in progress.
- [ ] **Granular Audio Slider** &mdash; Live wallpapers default to muted; an in-app volume slider is in development.

---

## License

This project is licensed under the **GNU General Public License v3.0** &mdash; see the [LICENSE](LICENSE) file for details.

<p align="center">
  Made by <a href="https://github.com/theabsoluteray/rexpaper">theabsoluteray</a>
</p>
