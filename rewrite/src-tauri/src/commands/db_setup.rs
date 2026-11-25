use crate::db::{AppState, DbStatus};
use crate::models::User;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn get_db_status(state: State<'_, AppState>) -> Result<DbStatus, String> {
    state.get_db_status().await
}

#[tauri::command]
pub async fn import_sqlite_db(
    source_path: String,
    state: State<'_, AppState>,
) -> Result<DbStatus, String> {
    state.import_sqlite_db(&PathBuf::from(source_path)).await
}

#[tauri::command]
pub async fn create_empty_db(state: State<'_, AppState>) -> Result<DbStatus, String> {
    state.create_empty_db().await
}

#[tauri::command]
pub async fn export_sqlite_db(dest_path: String, state: State<'_, AppState>) -> Result<(), String> {
    state.export_sqlite_db(&PathBuf::from(dest_path)).await
}

#[tauri::command]
pub async fn list_users(state: State<'_, AppState>) -> Result<Vec<User>, String> {
    state.list_users().await
}

#[tauri::command]
pub async fn switch_user(user_id: i64, state: State<'_, AppState>) -> Result<DbStatus, String> {
    state.switch_user(user_id).await
}
