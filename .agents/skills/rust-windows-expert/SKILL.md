---
name: rust-windows-expert
description: >-
  Expert guide for Rust systems programming on Windows.
  Use when interacting with Win32 APIs (windows-rs), WorkerW desktop wallpaper injection,
  process lifecycle management, MSVC dynamic linking, C-ABI FFI, and Cargo build scripts.
---

# Rust Windows Systems Expert Guide

Comprehensive technical guide for building native, high-performance Windows applications in Rust.

---

## 1. Win32 API Integration with `windows-rs` (0.61+)

### Wide Strings & Windows Types
In `windows-rs` 0.61+, use the compile-time `w!` macro for UTF-16 strings:
```rust
use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, FindWindowExW};

// ✅ Correct compile-time UTF-16 wide string
let progman = FindWindowW(w!("Progman"), None)?;

// Sub-window query (note Option<HWND> parameters in 0.61)
let shell_view = FindWindowExW(Some(hwnd), None, w!("SHELLDLL_DefView"), None)?;
```

---

## 2. Windows WorkerW Desktop Wallpaper Injection

To draw animated content (videos, shaders, webviews) behind desktop icons on Windows 10 & 11:

### The Injection Protocol
1. Find `Progman` (Program Manager).
2. Send shell message `0x052C` with parameters `(0x0000000D, 0x00000001)` to trigger Explorer to spawn a `WorkerW` layer.
3. Enumerate top-level windows (`EnumWindows`) to find the `WorkerW` window sibling that directly follows `SHELLDLL_DefView`.
4. Attach the rendering process to `WorkerW` via window ID (`--wid=<hwnd>`).

```rust
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::BOOL;

pub fn find_workerw() -> Result<HWND, Box<dyn std::error::Error>> {
    unsafe {
        let progman = FindWindowW(windows::core::w!("Progman"), None)?;
        let mut res: usize = 0;

        // Trigger WorkerW spawn
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
            if !progman.0.is_null() {
                return Ok(progman);
            }
            return Err("WorkerW window handle not found".into());
        }

        Ok(workerw)
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        if let Ok(shell_dll) = FindWindowExW(Some(hwnd), None, windows::core::w!("SHELLDLL_DefView"), None) {
            if !shell_dll.0.is_null() {
                if let Ok(workerw) = FindWindowExW(None, Some(hwnd), windows::core::w!("WorkerW"), None) {
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
```

### Live ⇄ Static Switching
- **Switch to Static**: Kill video process ➔ `ShowWindow(workerw, SW_HIDE)` ➔ Call `SystemParametersInfoW(SPI_SETDESKWALLPAPER, ...)`.
- **Switch to Live**: `ShowWindow(workerw, SW_SHOW)` ➔ Spawn video process with `--wid=<hwnd>`.

---

## 3. Silent Background Process Management

When launching background helpers (`mpv.exe`, `taskkill`) on Windows, prevent command prompt popups with `CREATE_NO_WINDOW`:
```rust
use std::process::Command;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

let mut cmd = Command::new("mpv.exe");
cmd.creation_flags(CREATE_NO_WINDOW);
cmd.args([
    &format!("--wid={}", hwnd),
    "--loop=inf",
    "--mute",
    "--no-border",
    "--no-window-dragging",
    "--vo=gpu",
    "--gpu-api=d3d11",
    "--hwdec=auto",
    "--really-quiet",
    video_path.to_str().unwrap(),
]);
let child = cmd.spawn()?;
```

---

## 4. MSVC Import Libraries & Runtime DLL Staging

### Linker Binding (`build.rs`)
When linking C/C++ libraries on Windows via MSVC (`mpv.lib` generated from `.def`):
- Ensure `build.rs` instructs the linker:
  ```rust
  println!("cargo:rustc-link-search=native={}", lib_dir);
  ```
- Auto-stage runtime DLLs into Cargo target directories (`target/debug` and `target/release`):
  ```rust
  // Copy libmpv-2.dll and provide aliases (mpv.dll, mpv-2.dll) to satisfy all loader paths
  std::fs::copy(&libmpv2, target_dir.join("mpv.dll"))?;
  std::fs::copy(&libmpv2, target_dir.join("libmpv-2.dll"))?;
  std::fs::copy(&libmpv2, target_dir.join("mpv-2.dll"))?;
  ```

---

## 5. Thread-Safe State & Performance

- **Shared State**: Wrap application state in `Arc<Mutex<AppState>>`.
- **Thumbnail Generation**: Spawn worker threads for heavy image resizing using `image::imageops::thumbnail` with fast filtering (`FilterType::Triangle` or `Nearest`).
- **Settings Persistence**: Use atomic file writes or serde JSON serialization to avoid corrupted configuration on sudden process termination.
