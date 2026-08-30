use std::path::Path;

pub fn apply_static_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
        use windows::Win32::UI::WindowsAndMessaging::*;
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        // Terminate any running live wallpaper process and hide the WorkerW canvas
        let _ = crate::platform::windows::stop_live_wallpaper();

        let wide_path: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();
        
        unsafe {
            SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                Some(wide_path.as_ptr() as *mut _),
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            )?;
        }
    }
    Ok(())
}

pub fn scan_and_load_static(root: &Path, state: crate::SharedState) -> Result<(), Box<dyn std::error::Error>> {
    crate::scanner::scan_static(root, state)?;
    Ok(())
}