<p align="center">
  <img src="assets/logo.svg" alt="RexPaper Logo" width="120" />
</p>

<h1 align="center">RexPaper</h1>

<p align="center">
  <strong>A fast, modern, native wallpaper manager for Windows</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-1.0.0-blue?style=flat-square" alt="Version" />
  <img src="https://img.shields.io/badge/rust-edition%202024-orange?style=flat-square&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/slint-1.17-purple?style=flat-square" alt="Slint" />
  <img src="https://img.shields.io/badge/license-GPL--3.0-green?style=flat-square" alt="License" />
  <img src="https://img.shields.io/badge/platform-windows%2010%20%7C%2011-0078D6?style=flat-square&logo=windows" alt="Platform" />
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

- **Static Wallpaper Manager** &mdash; Recursively scans image directories (`.png`, `.jpg`, `.jpeg`, `.webp`), generates high-quality thumbnails, and sets backgrounds instantly.
- **Live Animated Wallpapers** &mdash; Seamlessly plays hardware-accelerated video wallpapers (`.mp4`, `.webm`, `.mkv`, `.mov`) directly behind Windows desktop icons using Direct3D11 `mpv`.
- **Real Video Thumbnail Generation** &mdash; Automatically extracts high-definition frame captures at 0.5s from video files via `mpv` and caches them locally for instant gallery browsing.
- **Native Windows System Tray** &mdash; Integrates directly with the Windows taskbar notification area via Win32 `Shell_NotifyIconW`, offering quick access, page navigation, and window controls.
- **Silent Windows Autostart** &mdash; Automatically registers in `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` with `--autostart` to launch silently in the system tray on Windows boot.
- **Per-Monitor V2 High-DPI Awareness** &mdash; Enforces native physical pixel rendering across any display scaling factor (125%, 150%, 175%) for crisp typography and vector assets.
- **Slint Fluent Style & Animations** &mdash; Native Windows 11 Fluent aesthetic featuring smooth page cross-fade transitions, card hover animations, and micro-interactions.
- **Uniform 4-Column Grid** &mdash; Mathematically computed responsive column layout that maintains equal card widths regardless of individual image aspect ratios.
- **Two-Way Seamless Switching** &mdash; Switching between static and live wallpapers automatically manages the background `mpv` process and desktop canvas without lingering artifacts.
- **Category Discovery & Instant Search** &mdash; Automatically extracts categories from folder hierarchies and provides instant name filtering.
- **Dark & Light Themes** &mdash; Instant theme switching with smooth animated color token transitions.
- **Persistent Configuration** &mdash; Remembers configured directories, appearance preferences, and system behavior across app launches.

---

## Tech Stack

| Layer | Technology | Description |
|---|---|---|
| **Language** | <img src="https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust" alt="Rust" /> Rust (Edition 2024) | Safe, blazingly fast systems language |
| **UI Framework** | <img src="https://img.shields.io/badge/Slint-1.17-purple?style=flat-square" alt="Slint" /> Slint GUI | Declarative, reactive native UI framework with Fluent styling |
| **Video Engine** | <img src="https://img.shields.io/badge/mpv-libmpv-darkblue?style=flat-square" alt="mpv" /> libmpv & mpv binary | Hardware-accelerated Direct3D11 / GPU video playback and frame extraction |
| **OS Integration** | <img src="https://img.shields.io/badge/windows--rs-0.61-blue?style=flat-square&logo=windows" alt="Windows" /> windows-rs | Native Win32 API, `WorkerW` injection, `Shell_NotifyIconW`, DWM, HiDPI |
| **Image Processing** | <img src="https://img.shields.io/badge/image--rs-0.25-yellow?style=flat-square" alt="image-rs" /> image-rs | Fast thumbnail generation, aspect-ratio scaling, and image format decoding |
| **Dialogs** | <img src="https://img.shields.io/badge/rfd-0.15-green?style=flat-square" alt="rfd" /> rfd | Native Windows folder and file picker dialogs |
| **Serialization** | <img src="https://img.shields.io/badge/Serde-1.0-blue?style=flat-square" alt="Serde" /> Serde & serde_json | Robust configuration persistence to `settings.json` |

---

## Architecture & Implementation

### 1. Slint UI Architecture
- **`ui/main.slint`**: Root window with fixed sidebar (`NavBar`) and smooth page cross-fading (`StaticPage`, `LivePage`, `SettingsPage`).
- **`ui/navbar.slint`**: Left navigation drawer with vector SVG branding, active tab indicators, and hover easing.
- **`ui/static.slint` & `ui/live.slint`**:
  - Uniform 4-column card grid with equal-weight layout distribution.
  - Controls bar with vertically centered `ComboBox` (category selector) and `LineEdit` (instant search).
  - Cards featuring `image-fit: cover`, subtle hover sheens, and category pill tags.
- **`ui/store.slint`**: Centralized reactive state store managing wallpaper models, active themes, directory paths, and search queries.

### 2. Desktop Wallpaper Engine (`src/platform/windows.rs`)
- **WorkerW Injection**: Sends Windows shell message `0x052C` to `Progman` to spawn a `WorkerW` canvas layer directly behind desktop icons (`SHELLDLL_DefView`).
- **Silent Background Execution**: Launches `mpv.exe` with `CREATE_NO_WINDOW (0x08000000)`, attached directly to the desktop window handle (`--wid=<hwnd>`) using Direct3D11 hardware decoding.
- **Clean Two-Way Switching**:
  - **Live to Static**: Terminates `mpv.exe`, hides `WorkerW` (`SW_HIDE`), and applies the static image via `SystemParametersInfoW(SPI_SETDESKWALLPAPER)`.
  - **Static to Live**: Un-hides `WorkerW` (`SW_SHOW`), terminates any previous video process, and attaches `mpv.exe`.

### 3. System Tray & Autostart Lifecycle (`src/platform/tray.rs` & `src/settings.rs`)
- **System Tray Notification Area**: Spawns a dedicated Win32 message loop managing a `NOTIFYICONDATAW` tray icon.
  - **Left-Click / Double-Click**: Restores and focuses the main application window.
  - **Right-Click Menu**: Quick navigation to Static Wallpapers, Live Wallpapers, Settings, or Quit.
- **Autostart Mode**: Launches with `--autostart` to initialize silently in the tray on Windows boot without opening the main window.

### 4. Automatic Dependency Staging (`build.rs`)
- Bundles standalone `libmpv-2.dll` (auto-aliased as `mpv.dll`, `mpv-2.dll`, `libmpv-2.dll`), `mpv.exe`, and `mpv.com` directly into `target/release/` and `target/debug/` upon compilation.

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
├── assets/                  # Application vector SVG icons and branding
├── mpv/                     # Standalone mpv binary and libmpv-2.dll
├── mpv-lib/                 # MSVC mpv.lib import library and def file
├── src/
│   ├── main.rs              # Application entry point, tray wiring, & state sync
│   ├── models.rs            # Wallpaper models, categories, and row grouping
│   ├── scanner.rs           # Recursive filesystem wallpaper directory scanner
│   ├── thumbnail.rs         # Real video frame extraction (mpv) & thumbnail caching
│   ├── settings.rs          # Persistent JSON settings manager & autostart registry
│   ├── static_wallpaper.rs  # Static wallpaper scanning and Win32 application
│   ├── live_wallpaper.rs    # Live wallpaper controller
│   ├── mpv_player.rs        # libmpv backend bindings
│   └── platform/
│       ├── mod.rs           # Platform abstraction interface
│       ├── windows.rs       # Win32 WorkerW injection & mpv background process
│       └── tray.rs          # Native Windows taskbar notification area (system tray)
├── ui/                      # Slint declarative UI files
│   ├── main.slint           # Main application window with page transitions
│   ├── store.slint          # Theme tokens & global AppStore state
│   ├── navbar.slint         # Navigation sidebar with SVG icons
│   ├── static.slint         # Static wallpapers gallery page
│   ├── live.slint           # Live wallpapers gallery page
│   ├── settings.slint       # Application settings page
│   └── components/
│       └── VideoPlayer.slint# Video player component
├── .github/workflows/
│   └── release.yml          # Windows CI/CD release & WiX MSI workflow
├── build.rs                 # Slint compilation & DLL auto-staging script
└── Cargo.toml               # Cargo dependencies & manifest
```

---

## Roadmap

- [x] Fixed `0xc0000135 (STATUS_DLL_NOT_FOUND)` via automatic DLL aliasing in `build.rs`.
- [x] Fixed uneven thumbnail sizing with uniform 4-column responsive grid.
- [x] Fixed vertical alignment between dropdowns, search inputs, and sidebar icons.
- [x] Seamless two-way switching between Live and Static wallpapers with proper `WorkerW` state transitions.
- [x] Extracted real video frame thumbnails via `mpv` `--vo=image`.
- [x] Integrated native Windows taskbar notification area (System Tray) with context menu.
- [x] Enabled silent autostart on Windows boot (`--autostart`).
- [x] Converted all UI assets to vector SVG.
- [ ] **Multi-Monitor Support** &mdash; Currently renders to the primary desktop display; per-monitor wallpaper assignment is in progress.
- [ ] **Granular Audio Slider** &mdash; Live wallpapers default to muted; an in-app volume slider is in development.

---

## License

This project is licensed under the **GNU General Public License v3.0** &mdash; see the [LICENSE](LICENSE) file for details.

<p align="center">
  Made by <a href="https://github.com/theabsoluteray">theabsoluteray</a>
</p>
