// Re-export public types
pub mod models;
pub mod scanner;
pub mod thumbnail;
pub mod static_wallpaper;
pub mod live_wallpaper;
pub mod mpv_player;
pub mod platform;
pub mod settings;

pub use models::{AppState, SharedState, WallpaperItem, LiveWallpaperItem};
pub use settings::Settings;

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