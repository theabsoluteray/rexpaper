use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use rayon::prelude::*;
use slint::Image;

const THUMB_WIDTH: u32 = 384;
const THUMB_HEIGHT: u32 = 216;
const CACHE_VERSION: u64 = 8;
const MIN_VALID_THUMB_BYTES: u64 = 3500;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Precomputes static image thumbnails in parallel using Rayon.
pub fn precompute_static_thumbnails(paths: &[PathBuf]) {
    let cache_dir = get_cache_dir().join("static-thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);

    // Filter only files needing generation
    let missing: Vec<&PathBuf> = paths
        .iter()
        .filter(|p| !is_valid_cache_file(&cache_path(p, &cache_dir, CACHE_VERSION)))
        .collect();

    if missing.is_empty() {
        return;
    }

    missing.par_iter().for_each(|image_path| {
        let cached = cache_path(image_path, &cache_dir, CACHE_VERSION);
        if !is_valid_cache_file(&cached) {
            let _ = std::panic::catch_unwind(|| {
                if let Ok(dynamic_img) = image::open(image_path) {
                    let thumb = dynamic_img.thumbnail_exact(THUMB_WIDTH, THUMB_HEIGHT);
                    let _ = thumb.save(&cached);
                }
            });
        }
    });
}

/// Precomputes video thumbnails in parallel using Windows Shell API and mpv fallback.
pub fn precompute_video_thumbnails(paths: &[PathBuf]) {
    let cache_dir = get_cache_dir().join("video-thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);

    // Filter only files needing generation
    let missing: Vec<&PathBuf> = paths
        .iter()
        .filter(|p| !is_valid_cache_file(&cache_path(p, &cache_dir, CACHE_VERSION)))
        .collect();

    if missing.is_empty() {
        return;
    }

    missing.par_iter().for_each(|video_path| {
        let cached = cache_path(video_path, &cache_dir, CACHE_VERSION);
        if !is_valid_cache_file(&cached) {
            let _ = std::panic::catch_unwind(|| {
                let _ = extract_video_thumbnail(video_path, &cache_dir, &cached);
            });
        }
    });
}

/// Loads a lightweight cached thumbnail for a static image.
pub fn load_static_thumbnail(image_path: &Path) -> Image {
    let cache_dir = get_cache_dir().join("static-thumbs");
    let cached = cache_path(image_path, &cache_dir, CACHE_VERSION);
    if is_valid_cache_file(&cached) {
        if let Ok(img) = Image::load_from_path(&cached) {
            return img;
        }
    }
    Image::default()
}

/// Loads a lightweight cached thumbnail for a video file.
pub fn load_video_thumbnail(video_path: &Path) -> Image {
    let cache_dir = get_cache_dir().join("video-thumbs");
    let cached = cache_path(video_path, &cache_dir, CACHE_VERSION);
    if is_valid_cache_file(&cached) {
        if let Ok(img) = Image::load_from_path(&cached) {
            return img;
        }
    }
    Image::default()
}

fn is_valid_cache_file(path: &Path) -> bool {
    if let Ok(metadata) = path.metadata() {
        if metadata.is_file() && metadata.len() >= MIN_VALID_THUMB_BYTES {
            return true;
        }
        // Purge corrupt / empty placeholder file
        let _ = std::fs::remove_file(path);
    }
    false
}

fn cache_path(file_path: &Path, cache_dir: &Path, version: u64) -> PathBuf {
    cache_dir.join(format!("v{:016x}.jpg", hash_path(file_path) ^ version))
}

fn hash_path(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    hasher.finish()
}

/// Extracts a video frame thumbnail:
/// 1. Uses native Windows Shell API (hardware accelerated, instant, no external exe required).
/// 2. Falls back to mpv image output extraction if available.
pub fn extract_video_thumbnail(video_path: &Path, cache_dir: &Path, out_path: &Path) -> Option<Image> {
    let _ = std::fs::create_dir_all(cache_dir);

    // 1. Try Windows Shell native thumbnail extraction
    #[cfg(target_os = "windows")]
    {
        if extract_thumbnail_via_shell(video_path, out_path) {
            if is_valid_cache_file(out_path) {
                if let Ok(img) = Image::load_from_path(out_path) {
                    return Some(img);
                }
            }
        }
    }

    // 2. Fallback to mpv image extraction
    if extract_thumbnail_via_mpv(video_path, cache_dir, out_path) {
        if is_valid_cache_file(out_path) {
            if let Ok(img) = Image::load_from_path(out_path) {
                return Some(img);
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn extract_thumbnail_via_shell(video_path: &Path, out_path: &Path) -> bool {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::SIZE;
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, BITMAPINFO, BITMAPINFOHEADER,
        BI_RGB, DIB_RGB_COLORS,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};
    use windows::Win32::UI::Shell::{
        IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_BIGGERSIZEOK,
        SIIGBF_RESIZETOFIT,
    };

    unsafe {
        let com_init = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();

        let wide_path: Vec<u16> = video_path
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();

        let item_res: Result<IShellItemImageFactory, _> =
            SHCreateItemFromParsingName(PCWSTR(wide_path.as_ptr()), None);

        let mut success = false;

        if let Ok(factory) = item_res {
            let size = SIZE {
                cx: THUMB_WIDTH as i32,
                cy: THUMB_HEIGHT as i32,
            };
            if let Ok(hbitmap) = factory.GetImage(size, SIIGBF_RESIZETOFIT | SIIGBF_BIGGERSIZEOK) {
                if !hbitmap.0.is_null() {
                    let mut bmi = BITMAPINFO {
                        bmiHeader: BITMAPINFOHEADER {
                            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                            biWidth: THUMB_WIDTH as i32,
                            biHeight: -(THUMB_HEIGHT as i32), // Top-down DIB
                            biPlanes: 1,
                            biBitCount: 32,
                            biCompression: BI_RGB.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    let hdc = CreateCompatibleDC(None);
                    let mut buffer = vec![0u8; (THUMB_WIDTH * THUMB_HEIGHT * 4) as usize];

                    let lines = GetDIBits(
                        hdc,
                        hbitmap,
                        0,
                        THUMB_HEIGHT,
                        Some(buffer.as_mut_ptr() as *mut _),
                        &mut bmi,
                        DIB_RGB_COLORS,
                    );

                    let _ = DeleteDC(hdc);
                    let _ = DeleteObject(hbitmap.into());

                    if lines as u32 == THUMB_HEIGHT {
                        // Buffer is in BGRA format; convert to RGBA
                        for chunk in buffer.chunks_exact_mut(4) {
                            chunk.swap(0, 2); // Swap B and R
                        }

                        if let Some(rgba_img) = image::RgbaImage::from_raw(THUMB_WIDTH, THUMB_HEIGHT, buffer) {
                            let rgb_img = image::DynamicImage::ImageRgba8(rgba_img).into_rgb8();
                            if rgb_img.save(out_path).is_ok() {
                                success = true;
                            }
                        }
                    }
                }
            }
        }

        if com_init {
            CoUninitialize();
        }

        success
    }
}

fn extract_thumbnail_via_mpv(video_path: &Path, cache_dir: &Path, out_path: &Path) -> bool {
    let mpv_exe = match find_mpv_for_thumbnail() {
        Some(exe) => exe,
        None => return false,
    };

    let temp_hash = hash_path(video_path);
    let temp_out_dir = cache_dir.join(format!("mpv_tmp_{:016x}", temp_hash));
    let _ = std::fs::create_dir_all(&temp_out_dir);

    // Try extraction at 0.5s, then fallback to 0.0s
    let seek_times = ["0.5", "0.0"];
    let mut extracted = false;

    for start_time in seek_times {
        let mut cmd = Command::new(&mpv_exe);
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.args([
            "--no-config",
            "--no-audio",
            &format!("--start={}", start_time),
            "--frames=1",
            "--vo=image",
            "--vo-image-format=jpg",
            "--vo-image-jpeg-quality=88",
            &format!("--vo-image-outdir={}", temp_out_dir.to_string_lossy()),
            "--demuxer-max-bytes=8M",
            "--cache=no",
            "--really-quiet",
            &video_path.to_string_lossy(),
        ]);

        if let Ok(status) = cmd.status() {
            if status.success() {
                // Find any output image generated in temp_out_dir
                if let Ok(entries) = std::fs::read_dir(&temp_out_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            if let Ok(dynamic_img) = image::open(&path) {
                                let thumb = dynamic_img.thumbnail_exact(THUMB_WIDTH, THUMB_HEIGHT);
                                if thumb.save(out_path).is_ok() {
                                    extracted = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if extracted {
            break;
        }
    }

    let _ = std::fs::remove_dir_all(&temp_out_dir);
    extracted
}

fn find_mpv_for_thumbnail() -> Option<PathBuf> {
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            let p1 = exe_dir.join("mpv.exe");
            if p1.exists() { return Some(p1); }
            let p2 = exe_dir.join("mpv").join("mpv.exe");
            if p2.exists() { return Some(p2); }
        }
    }
    let p3 = std::path::Path::new("mpv").join("mpv.exe");
    if p3.exists() {
        if let Ok(abs) = p3.canonicalize() { return Some(abs); }
        return Some(p3);
    }
    which::which("mpv").ok()
}

pub fn get_cache_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "rexpaper", "RexPaper")
        .map(|dirs| dirs.cache_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("rexpaper_cache"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_thumbnail_real_or_fallback() {
        let test_dir = PathBuf::from(r"C:\stuff ricing\live-papers");
        if test_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&test_dir) {
                let video_files: Vec<PathBuf> = entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| {
                        p.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|s| s == "mp4" || s == "webm")
                            .unwrap_or(false)
                    })
                    .collect();

                if let Some(video) = video_files.first() {
                    let temp_cache = std::env::temp_dir().join("rexpaper_test_thumb_cache");
                    let out_path = temp_cache.join("test_out.jpg");
                    let _ = std::fs::create_dir_all(&temp_cache);

                    let result = extract_video_thumbnail(video, &temp_cache, &out_path);
                    assert!(out_path.exists(), "Thumbnail file should be generated");
                    assert!(out_path.metadata().unwrap().len() > MIN_VALID_THUMB_BYTES);
                    assert!(result.is_some());

                    let _ = std::fs::remove_dir_all(&temp_cache);
                }
            }
        }
    }
}