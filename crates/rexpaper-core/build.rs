fn main() {
    // Locate the mpv import library (mpv.lib) needed to link the `libmpv` crate.
    // Paths are resolved from the repository root (this crate lives at crates/rexpaper-core).
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let repo_root = std::path::Path::new(&crate_dir)
        .join("..")
        .join("..")
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&crate_dir));
    let local_lib_dir = repo_root.join("mpv-lib");
    let char_dir = local_lib_dir.to_string_lossy().into_owned();

    let mpv_lib_dir = match std::env::var("MPV_LIB_DIR") {
        Ok(dir) if !dir.trim().is_empty() => dir,
        _ if local_lib_dir.join("mpv.lib").exists() => char_dir,
        _ => String::new(),
    };

    if !mpv_lib_dir.trim().is_empty() {
        println!("cargo:rustc-link-search=native={}", mpv_lib_dir);
    }

    // Helper to copy mpv runtime files (exe + DLLs) into a target folder so the
    // built executables can find them next to their own binary.
    let copy_mpv_to_dir = |target_dir: &std::path::Path| {
        let mpv_src_dir = repo_root.join("mpv");
        if mpv_src_dir.exists() && target_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&mpv_src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name() {
                        let _ = std::fs::copy(&path, target_dir.join(name));
                    }
                }
            }
            let libmpv2 = mpv_src_dir.join("libmpv-2.dll");
            if libmpv2.exists() {
                let _ = std::fs::copy(&libmpv2, target_dir.join("mpv.dll"));
                let _ = std::fs::copy(&libmpv2, target_dir.join("mpv-2.dll"));
                let _ = std::fs::copy(&libmpv2, target_dir.join("libmpv-2.dll"));
            }
        }
    };

    // 1. Copy via OUT_DIR ancestry (target/debug or target/release)
    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let out_path = std::path::Path::new(&out_dir);
        if let Some(target_dir) = out_path.ancestors().nth(3) {
            copy_mpv_to_dir(target_dir);
        }
    }

    // 2. Explicitly ensure target/release and target/debug get copies if they exist
    copy_mpv_to_dir(&repo_root.join("target").join("debug"));
    copy_mpv_to_dir(&repo_root.join("target").join("release"));
}
