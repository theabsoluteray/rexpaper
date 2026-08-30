#![windows_subsystem = "windows"]

slint::include_modules!();

mod models;
mod scanner;
mod thumbnail;
mod static_wallpaper;
mod live_wallpaper;
mod mpv_player;
mod platform;
mod settings;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use slint::{VecModel, ModelRc, SharedString, Image};
use std::rc::Rc;
use std::thread;

use crate::models::AppState;

// Re-export types used by submodules via `crate::` paths
pub use crate::models::{SharedState, WallpaperItem, LiveWallpaperItem};
use crate::static_wallpaper::{apply_static_wallpaper, scan_and_load_static};
use crate::live_wallpaper::{scan_and_load_live, LiveWallpaperController};
use crate::platform::windows::{apply_live_wallpaper, stop_live_wallpaper, is_live_wallpaper_active};
use crate::settings::Settings;

type ThreadSafeState = Arc<Mutex<AppState>>;

pub fn load_image_from_path(path: &PathBuf) -> Result<Image, Box<dyn std::error::Error>> {
    Ok(Image::load_from_path(path)?)
}

fn static_to_data(item: &WallpaperItem) -> WallpaperData {
    let name = item
        .path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| item.category.clone());
    WallpaperData {
        name: name.into(),
        thumb: crate::thumbnail::load_static_thumbnail(&item.path),
        category: item.category.clone().into(),
        is_live: false,
        path: item.path.to_string_lossy().to_string().into(),
    }
}

fn live_to_data(item: &LiveWallpaperItem) -> WallpaperData {
    let name = item
        .path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| item.category.clone());
    WallpaperData {
        name: name.into(),
        thumb: crate::thumbnail::load_video_thumbnail(&item.path),
        category: item.category.clone().into(),
        is_live: true,
        path: item.path.to_string_lossy().to_string().into(),
    }
}

fn group_rows(items: &[WallpaperData], cols: usize) -> ModelRc<RowData> {
    let mut rows: Vec<RowData> = Vec::new();
    for chunk in items.chunks(cols.max(1)) {
        rows.push(RowData {
            items: ModelRc::from(Rc::new(VecModel::from(chunk.to_vec()))),
        });
    }
    ModelRc::from(Rc::new(VecModel::from(rows)))
}

const MAX_GRID_ITEMS: usize = 250;

/// Updates the static wallpaper UI: categories, filtered list, row grid, active category, and search query.
fn refresh_static_ui(window: &MainWindow, state: &ThreadSafeState, category: &str, search: &str) {
    let state_locked = state.lock().unwrap();
    let categories: Vec<SharedString> = state_locked
        .get_categories()
        .into_iter()
        .map(|s| s.into())
        .collect();
    window
        .global::<AppStore>()
        .set_categories(ModelRc::from(Rc::new(VecModel::from(categories))));
    let filtered = state_locked.filter_static(category, search);
    let converted: Vec<WallpaperData> = filtered.iter().take(MAX_GRID_ITEMS).map(static_to_data).collect();
    window
        .global::<AppStore>()
        .set_filtered_static_wallpapers(ModelRc::from(Rc::new(VecModel::from(
            converted.clone(),
        ))));
    let cols = window.global::<AppStore>().get_grid_columns() as usize;
    window
        .global::<AppStore>()
        .set_static_rows(group_rows(&converted, cols));
    window
        .global::<AppStore>()
        .set_active_category(category.into());
    window
        .global::<AppStore>()
        .set_static_search_query(search.into());
}

/// Updates the live wallpaper UI: categories, filtered list, row grid, active category, and search query.
fn refresh_live_ui(window: &MainWindow, state: &ThreadSafeState, category: &str, search: &str) {
    let state_locked = state.lock().unwrap();
    let categories: Vec<SharedString> = state_locked
        .get_live_categories()
        .into_iter()
        .map(|s| s.into())
        .collect();
    window
        .global::<AppStore>()
        .set_live_categories(ModelRc::from(Rc::new(VecModel::from(categories))));
    let filtered = state_locked.filter_live(category, search);
    let converted: Vec<WallpaperData> = filtered.iter().take(MAX_GRID_ITEMS).map(live_to_data).collect();
    window
        .global::<AppStore>()
        .set_filtered_live_wallpapers(ModelRc::from(Rc::new(VecModel::from(
            converted.clone(),
        ))));
    let cols = window.global::<AppStore>().get_grid_columns() as usize;
    window
        .global::<AppStore>()
        .set_live_rows(group_rows(&converted, cols));
    window
        .global::<AppStore>()
        .set_active_live_category(category.into());
    window
        .global::<AppStore>()
        .set_live_search_query(search.into());
}

fn main() -> Result<(), slint::PlatformError> {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::UI::HiDpi::*;
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }

    let main_window = MainWindow::new()?;

    let state: ThreadSafeState = Arc::new(Mutex::new(AppState::default()));
    let settings = Arc::new(Mutex::new(Settings::load()));
    let live_controller = Arc::new(Mutex::new(LiveWallpaperController::new().ok()));

    let app_store = main_window.global::<AppStore>();
    let _theme = main_window.global::<Theme>();

    main_window.set_active_page("static".into());

    let is_autostart = std::env::args().any(|arg| arg == "--autostart" || arg == "--minimized");

    let weak_window = main_window.as_weak();
    if is_autostart {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = weak_window.upgrade() {
                let _ = app.hide();
            }
        });
    } else {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(app) = weak_window.upgrade() {
                app.window().set_maximized(true);
            }
        });
    }

    // Setup Windows System Tray (Taskbar Notification Area)
    let window_weak_tray = main_window.as_weak();
    let _ = crate::platform::tray::setup_tray(move |action| {
        let win_weak = window_weak_tray.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = win_weak.upgrade() {
                match action {
                    crate::platform::tray::TrayAction::Open => {
                        let _ = window.show();
                        window.window().set_minimized(false);
                    }
                    crate::platform::tray::TrayAction::StaticPage => {
                        window.set_active_page("static".into());
                        let _ = window.show();
                        window.window().set_minimized(false);
                    }
                    crate::platform::tray::TrayAction::LivePage => {
                        window.set_active_page("live".into());
                        let _ = window.show();
                        window.window().set_minimized(false);
                    }
                    crate::platform::tray::TrayAction::SettingsPage => {
                        window.set_active_page("settings".into());
                        let _ = window.show();
                        window.window().set_minimized(false);
                    }
                    crate::platform::tray::TrayAction::Quit => {
                        let _ = slint::quit_event_loop();
                        std::process::exit(0);
                    }
                }
            }
        });
    });

    // Load static wallpapers
    if let Some(dir) = settings.lock().unwrap().wallpaper_dir.clone() {
        let state_clone = state.clone();
        let window_weak = main_window.as_weak();
        thread::spawn(move || {
            let _ = scan_and_load_static(&dir, state_clone.clone());
            let paths: Vec<PathBuf> = state_clone
                .lock()
                .unwrap()
                .static_wallpapers
                .iter()
                .map(|w| w.path.clone())
                .collect();
            crate::thumbnail::precompute_static_thumbnails(&paths);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    refresh_static_ui(&window, &state_clone, "All", "");
                }
            });
        });
    }

    // Load live wallpapers
    if let Some(dir) = settings.lock().unwrap().live_wallpaper_dir.clone() {
        let state_clone = state.clone();
        let window_weak = main_window.as_weak();
        thread::spawn(move || {
            let _ = scan_and_load_live(&dir, state_clone.clone());
            let paths: Vec<PathBuf> = state_clone
                .lock()
                .unwrap()
                .live_wallpapers
                .iter()
                .map(|w| w.path.clone())
                .collect();
            crate::thumbnail::precompute_video_thumbnails(&paths);
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    refresh_live_ui(&window, &state_clone, "All", "");
                }
            });
        });
    }

    // --- Static wallpaper directory picker ---
    let state_for_select = state.clone();
    let settings_for_select = settings.clone();
    let window_weak = main_window.as_weak();
    app_store.on_select_wallpaper_dir(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let folder_str = folder.to_string_lossy().to_string();
            if let Some(window) = window_weak.upgrade() {
                window
                    .global::<AppStore>()
                    .set_wallpaper_dir(folder_str.clone().into());

                // Persist the selected directory
                if let Ok(mut s) = settings_for_select.lock() {
                    s.wallpaper_dir = Some(PathBuf::from(&folder_str));
                    let _ = s.save();
                }

                let state_inner = state_for_select.clone();
                let win = window_weak.clone();
                thread::spawn(move || {
                    let _ = scan_and_load_static(&PathBuf::from(folder_str), state_inner.clone());
                    let paths: Vec<PathBuf> = state_inner
                        .lock()
                        .unwrap()
                        .static_wallpapers
                        .iter()
                        .map(|w| w.path.clone())
                        .collect();
                    crate::thumbnail::precompute_static_thumbnails(&paths);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(window) = win.upgrade() {
                            refresh_static_ui(&window, &state_inner, "All", "");
                        }
                    });
                });
            }
        }
    });

    // --- Static category filter ---
    let state_clone = state.clone();
    let window_weak = main_window.as_weak();
    app_store.on_set_active_category(move |category| {
        let cat: SharedString = category.into();
        if let Some(window) = window_weak.upgrade() {
            let search = window.global::<AppStore>().get_static_search_query().to_string();
            refresh_static_ui(&window, &state_clone, cat.as_str(), &search);
        }
    });

    // --- Static search/filter ---
    let state_clone = state.clone();
    let window_weak = main_window.as_weak();
    app_store.on_filter_static_search(move |search| {
        let s = search.to_string();
        if let Some(window) = window_weak.upgrade() {
            let category = window.global::<AppStore>().get_active_category().to_string();
            refresh_static_ui(&window, &state_clone, &category, &s);
        }
    });

    let window_weak_static = main_window.as_weak();
    app_store.on_apply_static_wallpaper(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        let _ = apply_static_wallpaper(&path_buf);
        if let Some(window) = window_weak_static.upgrade() {
            window.global::<AppStore>().set_live_wallpaper_active(false);
        }
    });

    // --- Settings toggle callbacks ---
    let settings_clone = settings.clone();
    let window_weak = main_window.as_weak();
    app_store.on_toggle_startup(move |enabled| {
        if let Ok(mut s) = settings_clone.lock() {
            s.set_run_on_startup(enabled);
        }
        if let Some(window) = window_weak.upgrade() {
            window.global::<AppStore>().set_run_on_startup(enabled);
        }
    });

    let settings_clone = settings.clone();
    let window_weak = main_window.as_weak();
    app_store.on_toggle_pause_fullscreen(move |enabled| {
        if let Ok(mut s) = settings_clone.lock() {
            s.set_pause_on_fullscreen(enabled);
        }
        if let Some(window) = window_weak.upgrade() {
            window
                .global::<AppStore>()
                .set_pause_on_fullscreen(enabled);
        }
    });

    let settings_clone = settings.clone();
    let window_weak = main_window.as_weak();
    app_store.on_toggle_mute_live(move |enabled| {
        if let Ok(mut s) = settings_clone.lock() {
            s.set_mute_live_wallpapers(enabled);
        }
        if let Some(window) = window_weak.upgrade() {
            window
                .global::<AppStore>()
                .set_mute_live_wallpapers(enabled);
        }
    });

    // --- Live wallpaper directory picker ---
    let state_clone = state.clone();
    let settings_for_live = settings.clone();
    let window_weak = main_window.as_weak();
    app_store.on_select_live_wallpaper_dir(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let folder_str = folder.to_string_lossy().to_string();
            if let Some(window) = window_weak.upgrade() {
                window
                    .global::<AppStore>()
                    .set_live_wallpaper_dir(folder_str.clone().into());

                // Persist the selected directory
                if let Ok(mut s) = settings_for_live.lock() {
                    s.live_wallpaper_dir = Some(PathBuf::from(&folder_str));
                    let _ = s.save();
                }

                let state_inner = state_clone.clone();
                let win = window_weak.clone();
                thread::spawn(move || {
                    let _ = scan_and_load_live(&PathBuf::from(folder_str), state_inner.clone());
                    let paths: Vec<PathBuf> = state_inner
                        .lock()
                        .unwrap()
                        .live_wallpapers
                        .iter()
                        .map(|w| w.path.clone())
                        .collect();
                    crate::thumbnail::precompute_video_thumbnails(&paths);
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(window) = win.upgrade() {
                            refresh_live_ui(&window, &state_inner, "All", "");
                        }
                    });
                });
            }
        }
    });

    // --- Live category filter ---
    let state_clone = state.clone();
    let window_weak = main_window.as_weak();
    app_store.on_set_active_live_category(move |category| {
        let cat: SharedString = category.into();
        if let Some(window) = window_weak.upgrade() {
            let search = window.global::<AppStore>().get_live_search_query().to_string();
            refresh_live_ui(&window, &state_clone, cat.as_str(), &search);
        }
    });

    // --- Live search/filter ---
    let state_clone = state.clone();
    let window_weak = main_window.as_weak();
    app_store.on_filter_live_search(move |search| {
        let s = search.to_string();
        if let Some(window) = window_weak.upgrade() {
            let category = window.global::<AppStore>().get_active_live_category().to_string();
            refresh_live_ui(&window, &state_clone, &category, &s);
        }
    });

    // --- Live wallpaper playback ---
    let live_ctrl = live_controller.clone();
    let window_weak = main_window.as_weak();
    app_store.on_play_live_wallpaper(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        if let Ok(mut guard) = live_ctrl.lock() {
            if let Some(ctrl) = guard.as_mut() {
                let _ = ctrl.play(&path_buf);
            }
        }
        if let Some(window) = window_weak.upgrade() {
            window.global::<AppStore>().set_live_preview_path(path);
            window.global::<AppStore>().set_live_preview_visible(true);
        }
    });

    let live_ctrl = live_controller.clone();
    let window_weak = main_window.as_weak();
    app_store.on_stop_live_wallpaper(move || {
        if let Ok(mut guard) = live_ctrl.lock() {
            if let Some(ctrl) = guard.as_mut() {
                let _ = ctrl.stop();
            }
        }
        if let Some(window) = window_weak.upgrade() {
            window
                .global::<AppStore>()
                .set_live_preview_visible(false);
        }
    });

    let live_ctrl_apply = live_controller.clone();
    let window_weak = main_window.as_weak();
    app_store.on_apply_live_wallpaper(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        // Stop the in-process preview player before spawning desktop mpv
        if let Ok(mut guard) = live_ctrl_apply.lock() {
            if let Some(ctrl) = guard.as_mut() {
                let _ = ctrl.stop();
            }
        }
        // Apply as desktop wallpaper via WorkerW + mpv
        let _ = apply_live_wallpaper(&path_buf);
        // Update UI state
        if let Some(window) = window_weak.upgrade() {
            window.global::<AppStore>().set_live_wallpaper_active(true);
            window.global::<AppStore>().set_live_preview_visible(false);
        }
    });

    let window_weak = main_window.as_weak();
    app_store.on_stop_desktop_live_wallpaper(move || {
        let _ = stop_live_wallpaper();
        if let Some(window) = window_weak.upgrade() {
            window
                .global::<AppStore>()
                .set_live_wallpaper_active(false);
        }
    });

    app_store.set_live_wallpaper_active(is_live_wallpaper_active());

    // Initial UI setup
    let empty_categories: Vec<SharedString> = Vec::new();
    app_store.set_categories(ModelRc::from(Rc::new(VecModel::from(empty_categories))));
    app_store.set_live_categories(ModelRc::from(Rc::new(VecModel::from(
        Vec::<SharedString>::new(),
    ))));
    app_store.set_active_category("All".into());
    app_store.set_active_live_category("All".into());
    app_store.set_static_search_query("".into());
    app_store.set_live_search_query("".into());

    let empty_wallpapers: Vec<WallpaperData> = Vec::new();
    app_store.set_filtered_static_wallpapers(ModelRc::from(Rc::new(VecModel::from(
        empty_wallpapers.clone(),
    ))));
    app_store.set_filtered_live_wallpapers(ModelRc::from(Rc::new(VecModel::from(
        Vec::<WallpaperData>::new(),
    ))));

    app_store.set_live_preview_visible(false);
    app_store.set_live_preview_path("".into());
    app_store.set_live_wallpaper_active(false);
    app_store.set_run_on_startup(settings.lock().unwrap().run_on_startup);
    app_store.set_pause_on_fullscreen(settings.lock().unwrap().pause_on_fullscreen);
    app_store.set_mute_live_wallpapers(settings.lock().unwrap().mute_live_wallpapers);

    // Display saved directory paths in the UI
    if let Ok(s) = settings.lock() {
        if let Some(ref dir) = s.wallpaper_dir {
            app_store.set_wallpaper_dir(dir.to_string_lossy().to_string().into());
        }
        if let Some(ref dir) = s.live_wallpaper_dir {
            app_store.set_live_wallpaper_dir(dir.to_string_lossy().to_string().into());
        }
    }

    main_window.run()
}
