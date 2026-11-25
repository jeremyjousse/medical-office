pub mod commands;
pub mod db;
pub mod models;
pub mod seed;

use db::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve macOS app data directory");
            let state = AppState::new(app_data_dir);
            tauri::async_runtime::block_on(async {
                if let Err(err) = state.init_from_saved_settings().await {
                    eprintln!("Failed to load saved DB settings: {err}");
                }
            });
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::db_setup::get_db_status,
            commands::db_setup::import_sqlite_db,
            commands::db_setup::create_empty_db,
            commands::db_setup::export_sqlite_db,
            commands::db_setup::list_users,
            commands::db_setup::switch_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
