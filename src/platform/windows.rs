use std::path::Path;
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::BOOL;

static MPV_PROCESS_ID: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(None);
static IS_WALLPAPER_ACTIVE: AtomicBool = AtomicBool::new(false);
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn apply_live_wallpaper(video_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !video_path.exists() {
        return Err(format!("Video file not found: {}", video_path.display()).into());
    }

    stop_live_wallpaper()?;

    let workerw = find_workerw()?;
    let hwnd = workerw.0 as usize;
    let mpv_exe = find_mpv_executable()?;

    // Unhide WorkerW canvas
    unsafe {
        let _ = ShowWindow(workerw, SW_SHOW);
    }

    let mut cmd = Command::new(mpv_exe);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd.args([
        &format!("--wid={}", hwnd),
        "--force-window=yes",
        "--loop=inf",
        "--mute",
        "--no-border",
        "--no-window-dragging",
        "--no-input-default-bindings",
        "--input-vo-keyboard=no",
        "--vo=gpu",
        "--gpu-api=auto",
        "--gpu-context=auto",
        "--hwdec=auto-safe",
        "--keep-open=yes",
        "--really-quiet",
        &video_path.to_string_lossy(),
    ]);

    let child = cmd.spawn()?;

    *MPV_PROCESS_ID.lock().unwrap() = Some(child.id());
    IS_WALLPAPER_ACTIVE.store(true, Ordering::SeqCst);

    Ok(())
}

pub fn stop_live_wallpaper() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pid) = *MPV_PROCESS_ID.lock().unwrap() {
        let mut cmd = Command::new("taskkill");
        cmd.creation_flags(CREATE_NO_WINDOW);
        let _ = cmd.args(["/PID", &pid.to_string(), "/F", "/T"]).status();
    }
    *MPV_PROCESS_ID.lock().unwrap() = None;
    IS_WALLPAPER_ACTIVE.store(false, Ordering::SeqCst);

    // Terminate any orphan mpv background instances
    let mut kill_all = Command::new("taskkill");
    kill_all.creation_flags(CREATE_NO_WINDOW);
    let _ = kill_all.args(["/IM", "mpv.exe", "/F", "/T"]).status();

    // Hide WorkerW overlay window so the static desktop wallpaper is visible immediately
    unsafe {
        let mut workerw = HWND(std::ptr::null_mut());
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(&mut workerw as *mut _ as isize));
        if !workerw.0.is_null() {
            let _ = ShowWindow(workerw, SW_HIDE);
        }
    }

    Ok(())
}

pub fn is_live_wallpaper_active() -> bool {
    IS_WALLPAPER_ACTIVE.load(Ordering::SeqCst)
}

fn find_workerw() -> Result<HWND, Box<dyn std::error::Error>> {
    unsafe {
        let progman = FindWindowW(windows::core::w!("Progman"), None)?;

        let mut res: usize = 0;
        let _ = SendMessageTimeoutW(
            progman,
            0x052C,
            WPARAM(0x0000000D),
            LPARAM(0x00000001),
            SMTO_NORMAL,
            1000,
            Some(&mut res),
        );

        let mut workerw = HWND(std::ptr::null_mut());
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(&mut workerw as *mut _ as isize));

        if workerw.0.is_null() {
            // Fallback to Progman handle if separate WorkerW was not spawned
            if !progman.0.is_null() {
                return Ok(progman);
            }
            return Err("WorkerW desktop window handle not found".into());
        }

        Ok(workerw)
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let shell_dll = FindWindowExW(Some(hwnd), None, windows::core::w!("SHELLDLL_DefView"), None);

        if let Ok(shell_dll) = shell_dll {
            if !shell_dll.0.is_null() {
                let workerw = FindWindowExW(None, Some(hwnd), windows::core::w!("WorkerW"), None);
                if let Ok(workerw) = workerw {
                    if !workerw.0.is_null() {
                        let workerw_ptr = lparam.0 as *mut HWND;
                        *workerw_ptr = workerw;
                        return BOOL(0);
                    }
                }
            }
        }

        BOOL(1)
    }
}

fn find_mpv_executable() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or("No parent directory")?;

    // 1. Check alongside the executable (target/release/mpv.exe or install dir)
    let bundled_mpv = exe_dir.join("mpv.exe");
    if bundled_mpv.exists() {
        return Ok(bundled_mpv);
    }

    // 2. Check mpv/ subfolder (e.g. exe_dir/mpv/mpv.exe)
    let subfolder_mpv = exe_dir.join("mpv").join("mpv.exe");
    if subfolder_mpv.exists() {
        return Ok(subfolder_mpv);
    }

    // 3. Check workspace root ./mpv/mpv.exe (for dev / cargo run)
    let workspace_mpv = std::path::Path::new("mpv").join("mpv.exe");
    if workspace_mpv.exists() {
        if let Ok(abs) = workspace_mpv.canonicalize() {
            return Ok(abs);
        }
        return Ok(workspace_mpv);
    }

    // 4. Check system PATH
    let mpv_in_path = which::which("mpv").ok();
    if let Some(path) = mpv_in_path {
        return Ok(path);
    }

    Err("mpv.exe not found. Please ensure mpv.exe is in the application folder or in PATH.".into())
}

#[allow(dead_code)]
pub fn pause_for_fullscreen() -> Result<(), Box<dyn std::error::Error>> {
    if is_live_wallpaper_active() {
        if let Some(pid) = *MPV_PROCESS_ID.lock().unwrap() {
            let mut cmd = Command::new("taskkill");
            cmd.creation_flags(CREATE_NO_WINDOW);
            let _ = cmd.args(["/PID", &pid.to_string(), "/F"]).status();
        }
        *MPV_PROCESS_ID.lock().unwrap() = None;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn resume_after_fullscreen(video_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !is_live_wallpaper_active() && MPV_PROCESS_ID.lock().unwrap().is_none() {
        apply_live_wallpaper(video_path)?;
    }
    Ok(())
}
