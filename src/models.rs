use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WallpaperItem {
    pub path: PathBuf,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct LiveWallpaperItem {
    pub path: PathBuf,
    pub category: String,
    #[allow(dead_code)]
    pub duration: Option<f64>,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub static_wallpapers: Vec<WallpaperItem>,
    pub live_wallpapers: Vec<LiveWallpaperItem>,
    pub categories: Vec<String>,
    pub live_categories: Vec<String>,
    #[allow(dead_code)]
    pub active_category: String,
    #[allow(dead_code)]
    pub active_live_category: String,
}

pub type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    #[allow(dead_code)]
    pub fn new() -> SharedState {
        Arc::new(Mutex::new(AppState::default()))
    }

    pub fn add_static_wallpaper(&mut self, item: WallpaperItem) {
        if !self.categories.contains(&item.category) {
            self.categories.push(item.category.clone());
        }
        self.static_wallpapers.push(item);
    }

    pub fn add_live_wallpaper(&mut self, item: LiveWallpaperItem) {
        if !self.live_categories.contains(&item.category) {
            self.live_categories.push(item.category.clone());
        }
        self.live_wallpapers.push(item);
    }

    pub fn get_categories(&self) -> Vec<String> {
        let mut cats = vec!["All".to_string()];
        cats.extend(self.categories.iter().cloned());
        cats.sort();
        cats.dedup();
        cats
    }

    pub fn get_live_categories(&self) -> Vec<String> {
        let mut cats = vec!["All".to_string()];
        cats.extend(self.live_categories.iter().cloned());
        cats.sort();
        cats.dedup();
        cats
    }

    pub fn filter_static(&self, category: &str, search: &str) -> Vec<WallpaperItem> {
        let search_lower = search.trim().to_lowercase();
        self.static_wallpapers
            .iter()
            .filter(|w| {
                let matches_cat = category == "All" || w.category == category;
                let matches_search = search_lower.is_empty()
                    || w.path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase().contains(&search_lower))
                        .unwrap_or(false)
                    || w.category.to_lowercase().contains(&search_lower);
                matches_cat && matches_search
            })
            .cloned()
            .collect()
    }

    pub fn filter_live(&self, category: &str, search: &str) -> Vec<LiveWallpaperItem> {
        let search_lower = search.trim().to_lowercase();
        self.live_wallpapers
            .iter()
            .filter(|w| {
                let matches_cat = category == "All" || w.category == category;
                let matches_search = search_lower.is_empty()
                    || w.path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase().contains(&search_lower))
                        .unwrap_or(false)
                    || w.category.to_lowercase().contains(&search_lower);
                matches_cat && matches_search
            })
            .cloned()
            .collect()
    }
}