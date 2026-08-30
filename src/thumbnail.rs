use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use slint::Image;

const THUMB_WIDTH: u32 = 480;
const THUMB_HEIGHT: u32 = 270;
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Loads (and if needed generates and caches) a fast lightweight thumbnail for a static image file.
/// This prevents loading full-size 4K/8K images into memory, saving gigabytes of RAM/VRAM.
pub fn load_static_thumbnail(image_path: &Path) -> Image {
    let cache_dir = get_cache_dir().join("static-thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);

    let cached = cache_path(image_path, &cache_dir, 4);
    if cached.is_file() {
        if let Ok(img) = Image::load_from_path(&cached) {
            return img;
        }
    }

    if let Ok(dynamic_img) = image::open(image_path) {
        let thumb = dynamic_img.thumbnail_exact(THUMB_WIDTH, THUMB_HEIGHT);
        let _ = thumb.save(&cached);
        if cached.is_file() {
            if let Ok(img) = Image::load_from_path(&cached) {
                return img;
            }
        }
    }

    // Fallback
    Image::load_from_path(image_path).unwrap_or_default()
}

/// Loads (and if needed generates) a real frame thumbnail image for a video file.
pub fn load_video_thumbnail(video_path: &Path) -> Image {
    let cache_dir = get_cache_dir().join("video-thumbs");
    let _ = std::fs::create_dir_all(&cache_dir);

    let cached = cache_path(video_path, &cache_dir, 4);
    if cached.is_file() {
        if let Ok(img) = Image::load_from_path(&cached) {
            return img;
        }
    }

    extract_video_thumbnail(video_path, &cache_dir, &cached).unwrap_or_default()
}

fn cache_path(file_path: &Path, cache_dir: &Path, version: u64) -> PathBuf {
    cache_dir.join(format!("v{:016x}.jpg", hash_path(file_path) ^ version))
}

fn hash_path(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    hasher.finish()
}

/// Uses mpv to extract an actual video frame at 0.5s and save as a high-quality thumbnail
pub fn extract_video_thumbnail(video_path: &Path, cache_dir: &Path, out_path: &Path) -> Option<Image> {
    let _ = std::fs::create_dir_all(cache_dir);
    let temp_hash = hash_path(video_path);
    let temp_out_dir = cache_dir.join(format!("tmp_{:016x}", temp_hash));
    let _ = std::fs::create_dir_all(&temp_out_dir);

    // Locate mpv executable
    let mpv_exe = find_mpv_for_thumbnail();

    if let Some(exe) = mpv_exe {
        let mut cmd = Command::new(exe);
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.args([
            "--no-config",
            "--no-audio",
            "--vo=image",
            "--vo-image-format=jpeg",
            "--vo-image-jpeg-quality=90",
            &format!("--vo-image-outdir={}", temp_out_dir.to_string_lossy()),
            "--frames=1",
            "--start=0.5",
            "--really-quiet",
            &video_path.to_string_lossy(),
        ]);

        let _ = cmd.status();

        let extracted_frame = temp_out_dir.join("00000001.jpg");
        if extracted_frame.is_file() {
            // Resize and optimize thumbnail with image crate
            if let Ok(dynamic_img) = image::open(&extracted_frame) {
                let thumb = dynamic_img.thumbnail_exact(THUMB_WIDTH, THUMB_HEIGHT);
                let _ = thumb.save(out_path);
                let _ = std::fs::remove_dir_all(&temp_out_dir);
                if out_path.is_file() {
                    return Image::load_from_path(out_path).ok();
                }
            } else {
                let _ = std::fs::copy(&extracted_frame, out_path);
                let _ = std::fs::remove_dir_all(&temp_out_dir);
                if out_path.is_file() {
                    return Image::load_from_path(out_path).ok();
                }
            }
        }
    }

    let _ = std::fs::remove_dir_all(&temp_out_dir);

    // Clean neutral dark fallback without any icons or shapes
    draw_clean_placeholder(out_path).ok()?;
    if out_path.is_file() {
        Image::load_from_path(out_path).ok()
    } else {
        None
    }
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

fn draw_clean_placeholder(out_path: &Path) -> Result<(), image::ImageError> {
    use image::{ImageBuffer, Rgb};

    if let Some(parent) = out_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(THUMB_WIDTH, THUMB_HEIGHT, Rgb([24u8, 25, 32]));

    img.save(out_path)
}

pub fn get_cache_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "rexpaper", "RexPaper")
        .map(|dirs| dirs.cache_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("rexpaper_cache"))
}