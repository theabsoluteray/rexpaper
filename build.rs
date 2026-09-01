fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".to_string());
    slint_build::compile_with_config("ui/main.slint", config).unwrap();

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "RexPaper");
        res.set("FileDescription", "RexPaper - Native Wallpaper Manager for Windows");
        res.set("LegalCopyright", "GPL-3.0");
        res.set_manifest(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
  <asmv3:application>
    <asmv3:windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </asmv3:windowsSettings>
  </asmv3:application>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
</assembly>"#);
        let _ = res.compile();
    }

    // Locate the mpv import library (mpv.lib) needed to link the `libmpv` crate.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let local_lib_dir = std::path::Path::new(&manifest_dir).join("mpv-lib");
    let char_dir = local_lib_dir.to_string_lossy().into_owned();

    let mpv_lib_dir = match std::env::var("MPV_LIB_DIR") {
        Ok(dir) if !dir.trim().is_empty() => dir,
        _ if local_lib_dir.join("mpv.lib").exists() => char_dir,
        _ => String::new(),
    };

    if !mpv_lib_dir.trim().is_empty() {
        println!("cargo:rustc-link-search=native={}", mpv_lib_dir);
    }

    // Helper to copy files and create aliases in a target folder
    let copy_mpv_to_dir = |target_dir: &std::path::Path| {
        let mpv_src_dir = std::path::Path::new(&manifest_dir).join("mpv");
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
    copy_mpv_to_dir(&std::path::Path::new(&manifest_dir).join("target").join("debug"));
    copy_mpv_to_dir(&std::path::Path::new(&manifest_dir).join("target").join("release"));
}
