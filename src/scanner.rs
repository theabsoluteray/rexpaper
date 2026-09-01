use std::path::Path;
use walkdir::WalkDir;
use crate::{SharedState, WallpaperItem, LiveWallpaperItem};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "gif", "avif", "tiff", "tif"];
const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "webm", "mkv", "mov", "avi", "flv", "m4v", "gif", "mpg", "mpeg", "wmv",
    "ts", "m2ts", "mts", "ogv", "ogg", "3gp", "vob"
];

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AppState;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_scan_static_and_live_wallpapers() {
        let temp_dir = std::env::temp_dir().join(format!("rexpaper_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let anime_sub = temp_dir.join("Anime");
        let nature_sub = temp_dir.join("Nature");
        std::fs::create_dir_all(&anime_sub).unwrap();
        std::fs::create_dir_all(&nature_sub).unwrap();

        // Create test files
        std::fs::write(anime_sub.join("hero.png"), b"fake_png").unwrap();
        std::fs::write(anime_sub.join("fight.mp4"), b"fake_mp4").unwrap();
        std::fs::write(nature_sub.join("forest.jpg"), b"fake_jpg").unwrap();
        std::fs::write(nature_sub.join("waterfall.webm"), b"fake_webm").unwrap();
        std::fs::write(nature_sub.join("stars.mkv"), b"fake_mkv").unwrap();
        std::fs::write(temp_dir.join("root_video.mov"), b"fake_mov").unwrap();
        std::fs::write(temp_dir.join("readme.txt"), b"not_a_wallpaper").unwrap();

        let state = Arc::new(Mutex::new(AppState::default()));

        // Scan static
        scan_static(&temp_dir, state.clone()).unwrap();
        {
            let locked = state.lock().unwrap();
            assert_eq!(locked.static_wallpapers.len(), 2);
            let categories = locked.get_categories();
            assert!(categories.contains(&"All".to_string()));
            assert!(categories.contains(&"Anime".to_string()));
            assert!(categories.contains(&"Nature".to_string()));
        }

        // Scan live
        scan_live(&temp_dir, state.clone()).unwrap();
        {
            let locked = state.lock().unwrap();
            assert_eq!(locked.live_wallpapers.len(), 4);
            let live_cats = locked.get_live_categories();
            assert!(live_cats.contains(&"All".to_string()));
            assert!(live_cats.contains(&"Anime".to_string()));
            assert!(live_cats.contains(&"Nature".to_string()));
            assert!(live_cats.contains(&"Root".to_string()));

            let anime_live = locked.filter_live("Anime", "");
            assert_eq!(anime_live.len(), 1);
            assert_eq!(anime_live[0].path.file_name().unwrap(), "fight.mp4");

            let nature_live = locked.filter_live("Nature", "");
            assert_eq!(nature_live.len(), 2);

            let search_waterfall = locked.filter_live("All", "waterfall");
            assert_eq!(search_waterfall.len(), 1);
        }

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}