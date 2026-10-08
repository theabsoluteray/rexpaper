//! UI-free core engine for RexPaper.
//!
//! Everything in this crate is presentation-agnostic: filesystem scanning,
//! thumbnail generation/caching, static & live wallpaper application,
//! settings persistence, and Windows platform integration (WorkerW
//! injection, system tray, fullscreen monitoring). Both the legacy Slint
//! application and the Tauri v2 application consume this crate through
//! its public modules.

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
