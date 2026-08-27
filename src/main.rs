slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.set_active_page("static".into());

    // Defer maximize to event loop
    let startup_window = main_window.as_weak();
    slint::invoke_from_event_loop(move || {
        if let Some(app) = startup_window.upgrade() {
            app.window().set_maximized(true);
        }
    }).unwrap();

    let main_window_weak = main_window.as_weak();
    
    main_window.global::<AppStore>().on_select_wallpaper_dir(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let folder_str = folder.to_string_lossy().to_string();
            
            if let Some(window) = main_window_weak.upgrade() {
                window.global::<AppStore>().set_wallpaper_dir(folder_str.into());
            }
        }
    });

    main_window.run()
}
