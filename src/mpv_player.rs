use std::path::Path;
use std::sync::{Arc, Mutex};
use libmpv::Mpv;
use slint::Image;

pub struct MpvPlayer {
    mpv: Arc<Mutex<Mpv>>,
}

impl MpvPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mpv = Mpv::new()?;
        
        mpv.set_property("vo", "gpu")?;
        mpv.set_property("gpu-api", "d3d11")?;
        mpv.set_property("keep-open", "yes")?;
        mpv.set_property("idle", "yes")?;
        
        Ok(Self {
            mpv: Arc::new(Mutex::new(mpv)),
        })
    }

    pub fn load_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.command("loadfile", &[path.to_string_lossy().as_ref()])?;
        mpv.set_property("loop", "inf")?;
        mpv.set_property("mute", true)?;
        mpv.set_property("vid", "auto")?;
        mpv.set_property("aid", "no")?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.set_property("pause", false)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn pause(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.set_property("pause", true)?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.command("stop", &[])?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn seek(&self, percent: f64) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.command("seek", &[&percent.to_string(), "absolute-percent"])?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_time_pos(&self) -> Result<Option<f64>, Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        let val: f64 = mpv.get_property("time-pos")?;
        Ok(Some(val))
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> Result<Option<f64>, Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        let val: f64 = mpv.get_property("duration")?;
        Ok(Some(val))
    }

    #[allow(dead_code)]
    pub fn set_volume(&self, volume: f64) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.set_property("volume", volume)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_mute(&self, mute: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mpv = self.mpv.lock().unwrap();
        mpv.set_property("mute", mute)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn render_frame(&mut self, _width: u32, _height: u32) -> Result<Image, Box<dyn std::error::Error>> {
        Err("Preview not implemented in this version".into())
    }

    #[allow(dead_code)]
    pub fn wakeup(&self) {
        // Not needed in libmpv 2.x
    }
}