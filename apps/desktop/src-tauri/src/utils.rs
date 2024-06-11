use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

pub struct AppState {
    pub pool: sqlx::sqlite::SqlitePool,
}

pub fn tray_setup(app: &AppHandle) {
    let menu = MenuBuilder::new(app)
        .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>).unwrap())
        .item(&MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap())
        .build()
        .unwrap();

    let tray = TrayIconBuilder::with_id("main_tray")
        .menu(&menu)
        .build(app)
        .unwrap();

    tray.on_menu_event(move |app, event| match event.id().as_ref() {
        "open" => {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        "quit" => app.app_handle().exit(0),
        _ => {}
    });

    tray.set_icon(Some(app.default_window_icon().unwrap().clone()))
        .unwrap();
}
