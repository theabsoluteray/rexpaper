use std::path::PathBuf;
use std::os::windows::ffi::OsStrExt;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use windows::Win32::System::Registry::*;
use windows::core::PCWSTR;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub wallpaper_dir: Option<PathBuf>,
    pub live_wallpaper_dir: Option<PathBuf>,
    pub run_on_startup: bool,
    pub pause_on_fullscreen: bool,
    pub mute_live_wallpapers: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            wallpaper_dir: None,
            live_wallpaper_dir: None,
            run_on_startup: false,
            pause_on_fullscreen: true,
            mute_live_wallpapers: true,
        }
    }
}

pub fn get_app_dir() -> PathBuf {
    if let Some(dirs) = ProjectDirs::from("com", "rexpaper", "RexPaper") {
        dirs.config_dir().to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Some(dirs) = ProjectDirs::from("com", "rexpaper", "RexPaper") {
            let config_path = dirs.config_dir().join("settings.json");
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        Settings::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dirs) = ProjectDirs::from("com", "rexpaper", "RexPaper") {
            let config_dir = dirs.config_dir();
            std::fs::create_dir_all(config_dir)?;
            let config_path = config_dir.join("settings.json");
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_wallpaper_dir(&mut self, path: PathBuf) {
        self.wallpaper_dir = Some(path);
        let _ = self.save();
    }

    #[allow(dead_code)]
    pub fn set_live_wallpaper_dir(&mut self, path: PathBuf) {
        self.live_wallpaper_dir = Some(path);
        let _ = self.save();
    }

    pub fn set_run_on_startup(&mut self, enabled: bool) {
        self.run_on_startup = enabled;
        let _ = self.save();
        Self::update_startup_registry(enabled);
    }

    pub fn set_pause_on_fullscreen(&mut self, enabled: bool) {
        self.pause_on_fullscreen = enabled;
        let _ = self.save();
    }

    pub fn set_mute_live_wallpapers(&mut self, enabled: bool) {
        self.mute_live_wallpapers = enabled;
        let _ = self.save();
    }

    fn update_startup_registry(enabled: bool) {
        unsafe {
            let key_path: Vec<u16> = std::ffi::OsStr::new(r"Software\Microsoft\Windows\CurrentVersion\Run")
                .encode_wide()
                .chain(Some(0))
                .collect();
            
            let mut hkey = HKEY(std::ptr::null_mut());
            let result = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(key_path.as_ptr()),
                Some(0),
                KEY_SET_VALUE,
                &mut hkey,
            );
            
            if result.0 == 0 {
                let app_name: Vec<u16> = std::ffi::OsStr::new("RexPaper")
                    .encode_wide()
                    .chain(Some(0))
                    .collect();
                
                if enabled {
                    if let Ok(exe_path) = std::env::current_exe() {
                        let cmd_str = format!("\"{}\" --autostart", exe_path.display());
                        let value: Vec<u16> = cmd_str.encode_utf16().chain(Some(0)).collect();
                        let value_bytes: Vec<u8> = value.iter().flat_map(|c| c.to_le_bytes()).collect();
                        let _ = RegSetValueExW(
                            hkey,
                            PCWSTR(app_name.as_ptr()),
                            Some(0),
                            REG_SZ,
                            Some(value_bytes.as_slice()),
                        );
                    }
                } else {
                    let _ = RegDeleteValueW(hkey, PCWSTR(app_name.as_ptr()));
                }
                let _ = RegCloseKey(hkey);
            }
        }
    }
}