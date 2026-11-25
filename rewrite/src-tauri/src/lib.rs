pub mod commands;
pub mod db;
pub mod error;
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
            commands::office::get_home_summary,
            commands::office::list_countries,
            commands::office::get_office,
            commands::office::update_office,
            commands::practitioners::list_practitioners,
            commands::practitioners::get_practitioner,
            commands::practitioners::create_practitioner,
            commands::practitioners::update_practitioner,
            commands::treatment_types::list_treatment_types,
            commands::treatment_types::get_treatment_type,
            commands::treatment_types::create_treatment_type,
            commands::treatment_types::update_treatment_type,
            commands::treatment_types::delete_treatment_type,
            commands::bank_accounts::list_bank_accounts,
            commands::bank_accounts::get_bank_account,
            commands::bank_accounts::create_bank_account,
            commands::bank_accounts::update_bank_account,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
