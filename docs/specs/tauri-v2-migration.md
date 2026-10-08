# RexPaper v2 — Tauri v2 Migration Spec

Status: `ready-for-agent`
Decisions: settled in grilling session (2026-10-07)

---

## Problem Statement

RexPaper's Slint UI cannot evolve beyond what the Slint toolkit offers: no web-standard
tooling, no HTML/CSS layout model, no ecosystem of web components, and no way for
contributors unfamiliar with Slint to work on the interface. The user wants a modern,
minimal UI built with web-standard technology (Preact + Tailwind) while keeping every
piece of the proven Rust backend — the Win32 desktop injection, mpv integration,
thumbnail pipeline, tray, and settings — intact.

## Solution

Rebuild RexPaper's presentation layer on Tauri v2. The existing Rust core is extracted
into a reusable `rexpaper-core` library crate. A new Tauri app (`src-tauri/`) exposes
that core through typed commands and events. The UI is a minimal Preact + Tailwind
single-page app served by Tauri's WebView, migrated one page at a time so the working
Slint application remains usable until the new UI reaches parity.

Design language is explicitly minimal: flat surfaces, hairline borders, no gradients,
no neon, no glow, no decorative AI-generated visual filler.

## User Stories

1. As a RexPaper user, I want to launch the Tauri build and see a minimal, flat-styled
   wallpaper manager, so that the app feels modern without visual noise.
2. As a user, I want to browse my static wallpaper library in a responsive grid, so
   that I can find an image quickly.
3. As a user, I want thumbnails to lazy-load as I scroll, so that libraries with
   thousands of 4K/8K images stay smooth and low-memory.
4. As a user, I want to filter wallpapers by category (subfolder), so that I can
   navigate large collections.
5. As a user, I want live text search across wallpaper names, so that I can jump
   directly to a known wallpaper.
6. As a user, I want to click a static wallpaper to apply it to my desktop
   immediately, so that the apply flow stays one-click.
7. As a user, I want to choose my static wallpaper folder from within the app, so
   that I can point RexPaper at any drive or directory.
8. As a user, I want to browse live video wallpapers in the same grid layout, so
   that the experience is consistent across media types.
9. As a user, I want to hover a video thumbnail and see a streamed video preview,
   so that I can judge motion content before applying it.
10. As a user, I want video previews to stream over HTTP range requests, so that
    playback starts fast without downloading whole files.
11. As a user, I want to click a live wallpaper to apply it behind my desktop
    icons, so that I get the WorkerW live background experience.
12. As a user, I want a Stop control for the live desktop wallpaper, so that I can
    return to a static background at any time.
13. As a user, I want the system tray to offer Open, Static Page, Live Page,
    Settings, Pause Live Wallpaper, Next Wallpaper, Open Wallpaper Folder, and
    Quit, so that I can control the app without opening the main window.
14. As a user, I want the tray icon to render crisply at all DPI scales and never
    produce a ghost taskbar entry, so that the app behaves like a native utility.
15. As a user, I want window state (show/minimize/page) managed when triggered from
    the tray, so that tray actions land on the right page.
16. As a user, I want settings persisted in the standard app-data directory, so
    that configuration survives upgrades.
17. As a user, I want to toggle "Run on system startup" and have it write the
    Windows registry Run key with the same autostart flags as v1, so that silent
    tray autostart keeps working.
18. As a user, I want to toggle "Pause fullscreen applications" so that live
    wallpapers do not consume GPU while I game.
19. As a user, I want to toggle "Mute live wallpaper audio" so that the desktop
    stays silent by default.
20. As a user, I want to pick dark or light theme, so that the app matches my
    system preference.
21. As a user, I want my active static/live wallpaper restored on startup, so
    that a reboot returns my desktop to the previous state.
22. As a user, I want live wallpaper playback to auto-pause when a fullscreen
    app is detected, so that GPU resources are freed automatically.
23. As a user, I want to open the wallpaper folder in Explorer from the tray or
    UI, so that I can manage files without hunting for the path.
24. As a user, I want a "Next Wallpaper" shuffle action, so that I can rotate
    backgrounds without manual effort.
25. As a user, I want the app to start hidden when launched with
    `--autostart`/`--minimized`, so that boot is not interrupted.
26. As a user, I want arrow-key navigation and Enter to apply in the grid, so
    that the app is usable without a mouse.
27. As a user, I want Escape to clear the active search filter, so that I can
    reset the view quickly.
28. As a user, I want a responsive grid that adapts columns to window width, so
    that both 900px and ultrawide windows lay out cleanly.
29. As a user, I want empty states that explain how to select a folder, so that
    first run is self-explanatory.
30. As a user, I want errors (corrupt files, failed apply) surfaced inline
    rather than as crashes, so that one bad file never breaks the library.
31. As a developer, I want the Rust backend extracted into a library crate with
    no UI dependencies, so that core logic can be unit-tested and reused.
32. As a developer, I want a typed command layer between WebView and core, so
    that UI-backend contracts are explicit and compile-checked.
33. As a developer, I want async progress events for scans and thumbnail
    generation, so that the UI can show loading states without polling.
34. As a developer, I want the Slint app to keep building until parity is
    reached, so that migration is reversible page by page.
35. As a user, I want an NSIS installer and portable zip produced by Tauri's
    bundler, so that distribution gets simpler than the WiX pipeline.
36. As a user, I want existing `settings.json` values imported automatically, so
    that upgrading from v1 does not lose configuration.
37. As a user, I want mpv DLLs and the mpv binary staged next to the executable
    as they are today, so that live wallpapers keep working after packaging.
38. As a user, I want the app to keep working offline with no WebView2
    download prompt on machines that already ship Windows 11, so that install
    stays frictionless.

## Implementation Decisions

- **Framework**: Tauri v2 (Rust backend, WebView2 frontend on Windows).
- **Frontend**: Preact + Vite + TypeScript. Preact Signals for state; no
  Redux/Context boilerplate.
- **Styling**: Tailwind CSS, flat design tokens (solid fills, 1px borders,
  no gradients, no neon/glow effects).
- **Backend boundary**: `#[tauri::command]` functions for request/response;
  `AppHandle::emit` events for async progress (scan started/finished,
  thumbnail batches).
- **Core extraction**: current `src/` (excluding `main.rs` and Slint bindings)
  becomes the `rexpaper-core` library crate. `src-tauri` depends on it by path.
  Ownership: core keeps `scanner`, `thumbnail`, `settings`, `static_wallpaper`,
  `live_wallpaper`, `mpv_player`, `models`, `platform`.
- **Video preview**: streamed via Tauri's HTTP asset protocol with `Range`
  support; the `<video>` element uses MediaSource/range requests against
  `tauri://localhost`-style URLs. No in-app libmpv rendering in v2 preview.
- **Thumbnails**: standard `<img loading="lazy">` plus CSS
  `content-visibility: auto` on grid rows. Virtualization deferred unless
  profiling shows it is needed.
- **Tray**: Tauri built-in tray + menu API. Menu items: Open, Static Page,
  Live Page, Settings, Pause Live Wallpaper, Next Wallpaper, Open Wallpaper
  Folder, Quit. Tray icon rendering keeps the v1 embedded-icon resource so
  DPI metrics stay correct.
- **Settings**: `tauri-plugin-store` for JSON persistence in the app data
  directory, with an import path that migrates v1
  `%APPDATA%/rexpaper/settings.json` on first run.
- **Autostart**: stays a Rust-side command writing
  `HKCU\...\Windows\CurrentVersion\Run` with `--autostart --minimized`,
  preserving v1 behavior; Tauri's autostart plugin is not used.
- **Fullscreen pause**: existing monitor thread moved into core and driven
  through events so the UI can reflect paused state.
- **Navigation**: single-page app with three routes (static, live, settings)
  mirroring the v1 page names so tray actions map 1:1.
- **Grid model**: flat arrays in the store; row grouping is a view concern
  performed with CSS grid rather than pre-chunked row structs (the v1
  `RowData` chunking does not carry over).
- **Migration**: incremental. The Slint target remains buildable; each page is
  cut over to Tauri only after parity is verified side by side.
- **Packaging**: Tauri bundler with NSIS; mpv binaries/DLLs staged via Tauri's
  `resources`/`externalBin` config plus a post-build copy step equivalent to
  v1 `build.rs`.
- **Window**: min 900×640, default ~1200×800, maximized on normal launch,
  hidden on `--autostart`/`--minimized`.
- **i18n**: English only, matching v1.

## Testing Decisions

A good test asserts external behavior through the narrowest stable seam and
never asserts on implementation details such as widget structure or internal
state layout.

- **Primary seam**: the Tauri command layer. Commands are invoked exactly as
  the WebView invokes them and asserted on their returned values / emitted
  events. This is the single cross-boundary seam and is preferred over
  testing core internals directly.
- **Core unit tests** cover pure logic with no Win32 side effects: category
  grouping, search filtering, settings serialization/migration (v1 JSON →
  store schema), and thumbnail cache key derivation.
- **Windows-integration tests** (behind a `#[cfg(target_os = "windows")]`
  gate and marked to run only on CI Windows runners) cover
  `apply_static_wallpaper` file selection, tray menu construction, and
  registry Run-key write/cleanup round-trips in an isolated test key.
- **Frontend tests** use Vitest against Preact components at the signal
  boundary: given a store snapshot, assert rendered grid/empty-state/filter
  output. No DOM implementation-detail assertions.
- **Prior art**: none exists in the repo today; tests are introduced with
  this migration. Use `cargo test` for core/commands and Vitest for the UI.

## Out of Scope

- Cross-platform support (macOS/Linux).
- Multi-monitor per-display wallpaper assignment (remains on the roadmap,
  not part of this migration).
- Volume slider for live wallpapers (remains on the roadmap).
- Third-party JS plugin/extension system.
- In-app live wallpaper preview using libmpv rendering inside the WebView.
- Redesign of the WorkerW/mpv desktop injection mechanism (ported as-is).
- WiX MSI packaging (replaced by NSIS in this spec; WiX stays only until
  cutover if a hotfix release is needed on v1).
- Dark/light theme beyond the existing two-token palette.
- Automated UI screenshot/regression testing.

## Further Notes

- The v1 codebase is Windows-only and deeply Win32-dependent; Tauri was chosen
  for UI modernization, not portability. `windows-rs` usage carries over
  unchanged.
- Incremental migration implies both UIs can exist in the repo for a while.
  Keep `ui/` (Slint) and the Tauri frontend in clearly separated trees and
  make the default `cargo run` target explicit to avoid ambiguity.
- The command/event contract should be documented in one place (a TS types
  module generated or hand-maintained alongside the Rust command module) so
  the two sides cannot silently drift.
- First-run settings import must be one-way and non-destructive: if the store
  already exists, do not overwrite it with v1 values.
