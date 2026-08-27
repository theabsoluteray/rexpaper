slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // 1. Set the default page
    main_window.set_active_page("static".into());

    // THE STACK OVERFLOW FIX:
    // Schedule the maximize command in the event loop instead of calling it directly
    let startup_window = main_window.as_weak();
    slint::invoke_from_event_loop(move || {
        if let Some(app) = startup_window.upgrade() {
            app.window().set_maximized(true);
        }
    }).unwrap();

    // 2. Handle Settings callbacks via AppStore
    let main_window_weak = main_window.as_weak();
    
    main_window.global::<AppStore>().on_select_wallpaper_dir(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let folder_str = folder.to_string_lossy().to_string();
            
            if let Some(window) = main_window_weak.upgrade() {
                window.global::<AppStore>().set_wallpaper_dir(folder_str.into());
            }
        }
    });

    // 3. Start the event loop (this will trigger the maximize command above)
    main_window.run()
}