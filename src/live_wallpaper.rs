use std::path::Path;

pub fn scan_and_load_live(root: &Path, state: crate::SharedState) -> Result<(), Box<dyn std::error::Error>> {
    crate::scanner::scan_live(root, state)?;
    Ok(())
}

pub struct LiveWallpaperController {
    player: Option<crate::mpv_player::MpvPlayer>,
    current_path: Option<std::path::PathBuf>,
}

impl LiveWallpaperController {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            player: Some(crate::mpv_player::MpvPlayer::new()?),
            current_path: None,
        })
    }

    pub fn play(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(player) = &self.player {
            player.load_file(path)?;
            player.play()?;
            self.current_path = Some(path.to_path_buf());
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(player) = &self.player {
            player.stop()?;
        }
        self.current_path = None;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(player) = &self.player {
            player.pause()?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn render_frame(&mut self, width: u32, height: u32) -> Result<slint::Image, Box<dyn std::error::Error>> {
        if let Some(player) = &mut self.player {
            player.render_frame(width, height)
        } else {
            Err("Player not initialized".into())
        }
    }

    #[allow(dead_code)]
    pub fn set_mute(&self, mute: bool) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(player) = &self.player {
            player.set_mute(mute)
        } else {
            Err("Player not initialized".into())
        }
    }

    #[allow(dead_code)]
    pub fn current_path(&self) -> Option<&Path> {
        self.current_path.as_deref()
    }

    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.current_path.is_some()
    }
}

impl Drop for LiveWallpaperController {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}