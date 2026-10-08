//! IPC command layer between the Preact frontend and rexpaper-core.
//!
//! Contract: keep these signatures in sync with `frontend/src/api.ts`.
//! Invoke argument names are deliberately single-word (`path`, `enabled`)
//! so JS/Rust naming-conversion rules can never diverge. Mutating commands
//! return the full updated `Settings` snapshot so the frontend can
//! re-assign its store in one step.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tauri::State;

use rexpaper_core::settings::Settings;
use rexpaper_core::{LiveWallpaperItem, SharedState, WallpaperItem};

/// Managed by the Tauri app; the same settings state v1 keeps behind its
/// window callbacks.
pub type SettingsState = Arc<Mutex<Settings>>;

#[tauri::command]
pub fn get_settings(settings: State<'_, SettingsState>) -> Result<Settings, String> {
    let s = settings.lock().map_err(|e| e.to_string())?;
    Ok(s.clone())
}

#[tauri::command]
pub fn set_wallpaper_dir(
    path: String,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.set_wallpaper_dir(PathBuf::from(path));
    Ok(s.clone())
}

#[tauri::command]
pub fn set_live_wallpaper_dir(
    path: String,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.set_live_wallpaper_dir(PathBuf::from(path));
    Ok(s.clone())
}

#[tauri::command]
pub fn set_run_on_startup(
    enabled: bool,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.set_run_on_startup(enabled);
    Ok(s.clone())
}

#[tauri::command]
pub fn set_pause_on_fullscreen(
    enabled: bool,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.set_pause_on_fullscreen(enabled);
    Ok(s.clone())
}

#[tauri::command]
pub fn set_mute_live_wallpapers(
    enabled: bool,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.set_mute_live_wallpapers(enabled);
    Ok(s.clone())
}

/// Scans `settings.wallpaper_dir` and returns the flat item list; returns an
/// empty list when no directory has been configured yet. Heavy: directory
/// walk, so it runs off the main thread.
#[tauri::command(async)]
pub fn scan_static(
    settings: State<'_, SettingsState>,
    state: State<'_, SharedState>,
) -> Result<Vec<WallpaperItem>, String> {
    let dir = settings
        .lock()
        .map_err(|e| e.to_string())?
        .wallpaper_dir
        .clone();
    let Some(dir) = dir else {
        return Ok(Vec::new());
    };

    rexpaper_core::scanner::scan_static(&dir, state.inner().clone()).map_err(|e| e.to_string())?;

    let items = state
        .lock()
        .map_err(|e| e.to_string())?
        .static_wallpapers
        .clone();
    Ok(items)
}

/// Same contract as [`scan_static`] for `settings.live_wallpaper_dir`.
#[tauri::command(async)]
pub fn scan_live(
    settings: State<'_, SettingsState>,
    state: State<'_, SharedState>,
) -> Result<Vec<LiveWallpaperItem>, String> {
    let dir = settings
        .lock()
        .map_err(|e| e.to_string())?
        .live_wallpaper_dir
        .clone();
    let Some(dir) = dir else {
        return Ok(Vec::new());
    };

    rexpaper_core::scanner::scan_live(&dir, state.inner().clone()).map_err(|e| e.to_string())?;

    let items = state
        .lock()
        .map_err(|e| e.to_string())?
        .live_wallpapers
        .clone();
    Ok(items)
}

/// Applies `path` as the static wallpaper and records it in settings,
/// mirroring v1's `on_apply_static_wallpaper`. Blocking (live-wallpaper
/// teardown + BMP conversion), so it runs off the main thread.
#[tauri::command(async)]
pub fn apply_static(
    path: String,
    settings: State<'_, SettingsState>,
) -> Result<Settings, String> {
    let wallpaper = PathBuf::from(path);
    rexpaper_core::static_wallpaper::apply_static_wallpaper(&wallpaper)
        .map_err(|e| e.to_string())?;

    let mut s = settings.lock().map_err(|e| e.to_string())?;
    s.active_live_wallpaper = None;
    s.active_static_wallpaper = Some(wallpaper);
    s.wallpaper_mode = "static".to_string();
    s.save().map_err(|e| e.to_string())?;
    Ok(s.clone())
}
