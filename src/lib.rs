// Re-export the UI-free core engine so dependents can reach everything from
// the root crate (mirrors the pre-Tauri monolith's API).
pub use rexpaper_core::{
    live_wallpaper, models, mpv_player, platform, scanner, settings, static_wallpaper, thumbnail,
};
pub use rexpaper_core::{AppState, LiveWallpaperItem, Settings, SharedState, WallpaperItem};

// Include Slint modules to generate types
slint::include_modules!();

// The Slint-generated types (WallpaperData, AppStore, Theme, StaticConstants, LiveConstants)
// are already available in the crate root after slint::include_modules!()

// Helper function to load image on main thread
pub fn load_image_from_path(path: &std::path::Path) -> Result<slint::Image, Box<dyn std::error::Error>> {
    Ok(slint::Image::load_from_path(path)?)
}

// Re-export slint types for convenience
pub use slint::{Image, VecModel, ModelRc, SharedString, Weak};
pub use slint::Model;
pub use std::rc::Rc;
