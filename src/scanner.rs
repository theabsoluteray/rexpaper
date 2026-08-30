use std::path::Path;
use walkdir::WalkDir;
use crate::{SharedState, WallpaperItem, LiveWallpaperItem};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "gif", "avif", "tiff", "tif"];
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "webm", "mkv", "mov", "avi", "flv", "m4v", "gif", "mpg", "mpeg", "wmv"];

pub fn scan_static(root: &Path, state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let mut state_locked = state.lock().unwrap();
    state_locked.static_wallpapers.clear();
    state_locked.categories.clear();

    for entry in WalkDir::new(root).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                state_locked.add_static_wallpaper(WallpaperItem {
                    path: path.to_path_buf(),
                    category: get_category(root, path),
                });
            }
        }
    }
    Ok(())
}

pub fn scan_live(root: &Path, state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let mut state_locked = state.lock().unwrap();
    state_locked.live_wallpapers.clear();
    state_locked.live_categories.clear();

    for entry in WalkDir::new(root).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                state_locked.add_live_wallpaper(LiveWallpaperItem {
                    path: path.to_path_buf(),
                    category: get_category(root, path),
                    duration: None,
                });
            }
        }
    }
    Ok(())
}

fn get_category(root: &Path, file: &Path) -> String {
    file.parent()
        .and_then(|p| p.strip_prefix(root).ok())
        .and_then(|p| p.components().next())
        .and_then(|c| c.as_os_str().to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Root".to_string())
}

#[allow(dead_code)]
pub fn get_categories(state: &SharedState) -> Vec<String> {
    state.lock().unwrap().get_categories()
}

#[allow(dead_code)]
pub fn get_live_categories(state: &SharedState) -> Vec<String> {
    state.lock().unwrap().get_live_categories()
}