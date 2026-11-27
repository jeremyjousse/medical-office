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
            commands::patients::list_patients,
            commands::patients::get_patient,
            commands::patients::create_patient,
            commands::patients::update_patient,
            commands::patients::delete_patient,
            commands::patients::finder_patients,
            commands::patients::list_practitioner_options,
            commands::patients::import_patients_csv,
            commands::treatments::list_treatments,
            commands::treatments::get_treatment,
            commands::treatments::create_treatment,
            commands::treatments::update_treatment,
            commands::treatments::delete_treatment,
            commands::treatments::price_finder,
            commands::treatments::list_treatment_type_options,
            commands::payments::list_payments,
            commands::payments::list_payment_listing,
            commands::payments::get_payment,
            commands::payments::create_payment,
            commands::payments::update_payment,
            commands::payments::delete_payment,
            commands::payments::list_treatment_options_for_payment,
            commands::bank_deposits::list_bank_deposits,
            commands::bank_deposits::get_bank_deposit,
            commands::bank_deposits::list_pending_bank_checks,
            commands::bank_deposits::list_bank_account_options,
            commands::bank_deposits::create_bank_deposit,
            commands::bank_deposits::delete_bank_deposit,
            commands::document_scans::list_document_scans,
            commands::document_scans::get_document_scan,
            commands::document_scans::create_document_scan,
            commands::document_scans::open_document_scan_in_viewer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
