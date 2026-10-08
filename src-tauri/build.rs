fn main() {
    // Locate the mpv import library (mpv.lib) needed to link `libmpv`, which
    // rexpaper-core depends on. Mirrors crates/rexpaper-core/build.rs so the
    // search path is definitely present when this binary is linked (build
    // script outputs do not reliably propagate through rlibs).
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let repo_root = std::path::Path::new(&crate_dir)
        .join("..")
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&crate_dir));
    let local_lib_dir = repo_root.join("mpv-lib");

    let mpv_lib_dir = match std::env::var("MPV_LIB_DIR") {
        Ok(dir) if !dir.trim().is_empty() => dir,
        _ if local_lib_dir.join("mpv.lib").exists() => {
            local_lib_dir.to_string_lossy().into_owned()
        }
        _ => String::new(),
    };

    if !mpv_lib_dir.trim().is_empty() {
        println!("cargo:rustc-link-search=native={}", mpv_lib_dir);
    }

    tauri_build::build()
}
