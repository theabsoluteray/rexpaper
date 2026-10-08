#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::{Arc, Mutex};

use rexpaper_core::{AppState, Settings};

fn main() {
    // Settings are shared with the installed v1 app via the same
    // ProjectDirs config file; don't run both apps at once.
    let settings = Arc::new(Mutex::new(Settings::load()));
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(settings)
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::set_wallpaper_dir,
            commands::set_live_wallpaper_dir,
            commands::set_run_on_startup,
            commands::set_pause_on_fullscreen,
            commands::set_mute_live_wallpapers,
            commands::scan_static,
            commands::scan_live,
            commands::apply_static,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
