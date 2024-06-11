// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

use commands::cmd_reviews::*;
use commands::decks_commands::*;
use commands::flashcards_commands::*;

use sql::DbManager;
use tauri::Manager;
use utils::tray_setup;
use utils::AppState;

fn main() {
    let devtools = tauri_plugin_devtools::init();
    // TODO: look into the tauri docs of how it handles it's state
    // to ensure a new connection is not created each time.
    // TODO: handle DbManager error.
    // TODO: put the .setup function content in another file.
    tauri::Builder::default()
        .setup(|app| {
            let app = app.handle().clone();
            tray_setup(&app);
            tauri::async_runtime::spawn(async move {
                let db_manager = DbManager::new().await.unwrap();
                let state = AppState {
                    pool: db_manager.get_db_instance(),
                };
                app.manage(state);

                // scheduler
                scheduler::setup_scheduler(app).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .plugin(devtools)
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_deck,
            find_all_decks,
            find_deck_by_id,
            create_flashcard,
            find_flashcard_by_id,
            find_flashcards_by_deck_id,
            review_flashcard,
            find_flashcard_for_review,
            update_flashcard,
            get_total_reviews_by_month,
            find_reviews_by_deck_id,
            get_total_reviews_by_quality,
            get_correct_percentage,
            get_total_flashcards_by_deck,
            find_flashcards_left_for_review
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
