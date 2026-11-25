use crate::models::{now_rails_timestamp, Office, User};
use crate::seed::{COUNTRIES_SEED, SCHEMA_SQL};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::async_runtime::RwLock;

pub const REQUIRED_TABLES: &[&str] = &[
    "users",
    "offices",
    "countries",
    "patients",
    "practitioners",
    "medical_treatment_types",
    "medical_treatments",
    "payments",
    "payment_bank_checks",
    "bank_accounts",
    "bank_deposits",
    "document_scans",
];

const DB_FILENAME: &str = "database.sqlite3";
const SETTINGS_FILENAME: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub db_path: String,
    pub active_user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbStatus {
    pub initialized: bool,
    pub db_path: Option<String>,
    pub active_user: Option<User>,
    pub office: Option<Office>,
    pub users_count: i64,
    pub needs_office_setup: bool,
}

pub struct AppState {
    pub app_data_dir: PathBuf,
    pub pool: RwLock<Option<SqlitePool>>,
    pub active_user_id: RwLock<Option<i64>>,
}

impl AppState {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self {
            app_data_dir,
            pool: RwLock::new(None),
            active_user_id: RwLock::new(None),
        }
    }

    pub fn db_file_path(&self) -> PathBuf {
        self.app_data_dir.join(DB_FILENAME)
    }

    pub fn settings_file_path(&self) -> PathBuf {
        self.app_data_dir.join(SETTINGS_FILENAME)
    }

    pub async fn init_from_saved_settings(&self) -> Result<DbStatus, String> {
        let settings_path = self.settings_file_path();
        if !settings_path.exists() {
            return Ok(uninitialized_status());
        }

        let raw = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings: {e}"))?;
        let settings: Settings =
            serde_json::from_str(&raw).map_err(|e| format!("Failed to parse settings: {e}"))?;

        let db_path = PathBuf::from(&settings.db_path);
        if !db_path.exists() {
            return Ok(uninitialized_status());
        }

        self.open_existing_db(&db_path, settings.active_user_id)
            .await
    }

    pub async fn import_sqlite_db(&self, source_path: &Path) -> Result<DbStatus, String> {
        if !source_path.exists() || !source_path.is_file() {
            return Err("Selected database file does not exist.".to_string());
        }

        validate_sqlite_schema(source_path).await?;

        fs::create_dir_all(&self.app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {e}"))?;

        let target_path = self.db_file_path();
        if source_path == target_path {
            return self.open_existing_db(&target_path, None).await;
        }

        self.close_pool().await;
        remove_sqlite_sidecars(&target_path);

        fs::copy(source_path, &target_path)
            .map_err(|e| format!("Failed to copy SQLite database: {e}"))?;

        self.open_existing_db(&target_path, None).await
    }

    pub async fn create_empty_db(&self) -> Result<DbStatus, String> {
        fs::create_dir_all(&self.app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {e}"))?;

        let target_path = self.db_file_path();
        self.close_pool().await;
        remove_sqlite_sidecars(&target_path);
        if target_path.exists() {
            fs::remove_file(&target_path)
                .map_err(|e| format!("Failed to replace existing database: {e}"))?;
        }

        let opts = SqliteConnectOptions::new()
            .filename(&target_path)
            .create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await
            .map_err(|e| format!("Failed to create SQLite database: {e}"))?;

        initialize_empty_schema_and_seed(&pool).await?;

        let status = self.finalize_open(pool, &target_path, None).await?;
        Ok(status)
    }

    pub async fn export_sqlite_db(&self, dest_path: &Path) -> Result<(), String> {
        let pool_guard = self.pool.read().await;
        let pool = pool_guard
            .as_ref()
            .ok_or_else(|| "No active database to export.".to_string())?;

        if dest_path.exists() {
            fs::remove_file(dest_path)
                .map_err(|e| format!("Failed to overwrite destination file: {e}"))?;
        }

        let dest_str = dest_path
            .to_str()
            .ok_or_else(|| "Destination path is not valid UTF-8.".to_string())?;

        sqlx::query("VACUUM INTO ?")
            .bind(dest_str)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to export database: {e}"))?;

        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<User>, String> {
        let pool_guard = self.pool.read().await;
        let pool = pool_guard
            .as_ref()
            .ok_or_else(|| "Database is not initialized.".to_string())?;

        sqlx::query_as::<_, User>(
            "SELECT id, email, office_id, created_at, updated_at FROM users ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to list users: {e}"))
    }

    pub async fn switch_user(&self, user_id: i64) -> Result<DbStatus, String> {
        let pool_guard = self.pool.read().await;
        let pool = pool_guard
            .as_ref()
            .ok_or_else(|| "Database is not initialized.".to_string())?;

        let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to check user {user_id}: {e}"))?;
        if exists.is_none() {
            return Err(format!("User {user_id} does not exist."));
        }

        let (user, office) = resolve_active_user(pool, Some(user_id)).await?;
        *self.active_user_id.write().await = Some(user.id);
        self.save_settings(&self.db_file_path(), Some(user.id))?;

        let users_count = count_users(pool).await?;
        Ok(build_status(self.db_file_path(), user, office, users_count))
    }

    pub async fn get_db_status(&self) -> Result<DbStatus, String> {
        let pool_guard = self.pool.read().await;
        let Some(pool) = pool_guard.as_ref() else {
            return Ok(uninitialized_status());
        };
        let preferred_user_id = *self.active_user_id.read().await;
        let (user, office) = resolve_active_user(pool, preferred_user_id).await?;
        let users_count = count_users(pool).await?;
        Ok(build_status(self.db_file_path(), user, office, users_count))
    }

    pub async fn require_pool(&self) -> Result<SqlitePool, String> {
        self.pool
            .read()
            .await
            .clone()
            .ok_or_else(|| "Database is not initialized.".to_string())
    }

    pub async fn require_active_context(&self) -> Result<(SqlitePool, User, Office), String> {
        let pool = self.require_pool().await?;
        let preferred_user_id = *self.active_user_id.read().await;
        let (user, office) = resolve_active_user(&pool, preferred_user_id).await?;
        Ok((pool, user, office))
    }

    async fn open_existing_db(
        &self,
        db_path: &Path,
        preferred_user_id: Option<i64>,
    ) -> Result<DbStatus, String> {
        validate_sqlite_schema(db_path).await?;
        self.close_pool().await;

        let opts = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(false);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await
            .map_err(|e| format!("Failed to open SQLite database: {e}"))?;

        self.finalize_open(pool, db_path, preferred_user_id).await
    }

    async fn finalize_open(
        &self,
        pool: SqlitePool,
        db_path: &Path,
        preferred_user_id: Option<i64>,
    ) -> Result<DbStatus, String> {
        let (user, office) = resolve_active_user(&pool, preferred_user_id).await?;
        let users_count = count_users(&pool).await?;
        let user_id = user.id;

        *self.pool.write().await = Some(pool);
        *self.active_user_id.write().await = Some(user_id);
        self.save_settings(db_path, Some(user_id))?;

        Ok(build_status(
            db_path.to_path_buf(),
            user,
            office,
            users_count,
        ))
    }

    async fn close_pool(&self) {
        if let Some(old_pool) = self.pool.write().await.take() {
            old_pool.close().await;
        }
        *self.active_user_id.write().await = None;
    }

    fn save_settings(&self, db_path: &Path, active_user_id: Option<i64>) -> Result<(), String> {
        fs::create_dir_all(&self.app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {e}"))?;
        let settings = Settings {
            db_path: db_path.to_string_lossy().to_string(),
            active_user_id,
        };
        let json = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Failed to serialize settings: {e}"))?;
        fs::write(self.settings_file_path(), json)
            .map_err(|e| format!("Failed to write settings: {e}"))?;
        Ok(())
    }
}

pub async fn validate_sqlite_schema(db_path: &Path) -> Result<(), String> {
    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .read_only(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await
        .map_err(|e| format!("Invalid SQLite file: {e}"))?;

    let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to inspect SQLite schema: {e}"))?;

    let present: HashSet<String> = rows
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("name").ok())
        .collect();

    pool.close().await;

    let missing: Vec<&str> = REQUIRED_TABLES
        .iter()
        .copied()
        .filter(|t| !present.contains(*t))
        .collect();

    if !missing.is_empty() {
        return Err(format!(
            "Incompatible database: missing required table(s): {}",
            missing.join(", ")
        ));
    }

    Ok(())
}

pub async fn initialize_empty_schema_and_seed(pool: &SqlitePool) -> Result<(), String> {
    sqlx::raw_sql(SCHEMA_SQL)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create schema: {e}"))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to begin seed transaction: {e}"))?;

    for &(id, iso, name) in COUNTRIES_SEED {
        sqlx::query("INSERT INTO countries (id, iso, name) VALUES (?, ?, ?)")
            .bind(id)
            .bind(iso)
            .bind(name)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to seed country {iso}: {e}"))?;
    }

    let now = now_rails_timestamp();
    let office_res = sqlx::query("INSERT INTO offices (created_at, updated_at) VALUES (?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to create default office: {e}"))?;
    let office_id = office_res.last_insert_rowid();

    sqlx::query(
        "INSERT INTO users (email, encrypted_password, sign_in_count, created_at, updated_at, office_id) VALUES (?, '', 0, ?, ?, ?)",
    )
    .bind("local@medical-office.app")
    .bind(&now)
    .bind(&now)
    .bind(office_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to create default user: {e}"))?;

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit seed transaction: {e}"))?;

    Ok(())
}

pub async fn resolve_active_user(
    pool: &SqlitePool,
    preferred_user_id: Option<i64>,
) -> Result<(User, Office), String> {
    let mut user = if let Some(uid) = preferred_user_id {
        sqlx::query_as::<_, User>(
            "SELECT id, email, office_id, created_at, updated_at FROM users WHERE id = ?",
        )
        .bind(uid)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to query user {uid}: {e}"))?
    } else {
        None
    };

    if user.is_none() {
        user = sqlx::query_as::<_, User>(
            "SELECT id, email, office_id, created_at, updated_at FROM users ORDER BY id ASC LIMIT 1",
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to query default user: {e}"))?;
    }

    let mut user = match user {
        Some(u) => u,
        None => {
            let now = now_rails_timestamp();
            let office_id = create_empty_office(pool).await?;
            let res = sqlx::query(
                "INSERT INTO users (email, encrypted_password, sign_in_count, created_at, updated_at, office_id) VALUES (?, '', 0, ?, ?, ?)",
            )
            .bind("local@medical-office.app")
            .bind(&now)
            .bind(&now)
            .bind(office_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to insert fallback user: {e}"))?;

            User {
                id: res.last_insert_rowid(),
                email: "local@medical-office.app".to_string(),
                office_id: Some(office_id),
                created_at: Some(now.clone()),
                updated_at: Some(now),
            }
        }
    };

    let office = match user.office_id {
        Some(oid) => {
            let existing = sqlx::query_as::<_, Office>(
                "SELECT id, name, address, city, postal_code, country_id, phone, website, email, created_at, updated_at FROM offices WHERE id = ?",
            )
            .bind(oid)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to load office {oid}: {e}"))?;

            match existing {
                Some(o) => o,
                None => attach_new_office(pool, &mut user).await?,
            }
        }
        None => attach_new_office(pool, &mut user).await?,
    };

    Ok((user, office))
}

async fn attach_new_office(pool: &SqlitePool, user: &mut User) -> Result<Office, String> {
    let now = now_rails_timestamp();
    let office_id = create_empty_office(pool).await?;
    sqlx::query("UPDATE users SET office_id = ?, updated_at = ? WHERE id = ?")
        .bind(office_id)
        .bind(&now)
        .bind(user.id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to link office to user: {e}"))?;
    user.office_id = Some(office_id);

    sqlx::query_as::<_, Office>(
        "SELECT id, name, address, city, postal_code, country_id, phone, website, email, created_at, updated_at FROM offices WHERE id = ?",
    )
    .bind(office_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to fetch created office: {e}"))
}

async fn create_empty_office(pool: &SqlitePool) -> Result<i64, String> {
    let now = now_rails_timestamp();
    let res = sqlx::query("INSERT INTO offices (created_at, updated_at) VALUES (?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create empty office: {e}"))?;
    Ok(res.last_insert_rowid())
}

async fn count_users(pool: &SqlitePool) -> Result<i64, String> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to count users: {e}"))
}

pub fn is_office_incomplete(office: &Office) -> bool {
    let is_blank = |s: &Option<String>| s.as_deref().map(|v| v.trim().is_empty()).unwrap_or(true);
    is_blank(&office.name)
        || is_blank(&office.address)
        || is_blank(&office.postal_code)
        || is_blank(&office.city)
        || office.country_id.is_none()
        || is_blank(&office.phone)
        || is_blank(&office.email)
}

fn build_status(db_path: PathBuf, user: User, office: Office, users_count: i64) -> DbStatus {
    let needs_office_setup = is_office_incomplete(&office);

    DbStatus {
        initialized: true,
        db_path: Some(db_path.to_string_lossy().to_string()),
        active_user: Some(user),
        office: Some(office),
        users_count,
        needs_office_setup,
    }
}

fn uninitialized_status() -> DbStatus {
    DbStatus {
        initialized: false,
        db_path: None,
        active_user: None,
        office: None,
        users_count: 0,
        needs_office_setup: false,
    }
}

fn remove_sqlite_sidecars(db_path: &Path) {
    let base = db_path.to_string_lossy();
    for suffix in ["-wal", "-shm", "-journal"] {
        let sidecar = PathBuf::from(format!("{base}{suffix}"));
        if sidecar.exists() {
            let _ = fs::remove_file(sidecar);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn creates_empty_db_seeds_countries_and_resolves_user() {
        let tmp = TempDir::new().unwrap();
        let state = AppState::new(tmp.path().to_path_buf());

        let status = state.create_empty_db().await.unwrap();
        assert!(status.initialized);
        assert_eq!(status.users_count, 1);
        assert!(status.needs_office_setup);
        assert!(status.active_user.is_some());
        assert!(status.office.is_some());

        let pool_guard = state.pool.read().await;
        let pool = pool_guard.as_ref().unwrap();
        let country_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM countries")
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(country_count, 251);
    }

    #[tokio::test]
    async fn rejects_invalid_sqlite_file_and_imports_valid_copy() {
        let source_dir = TempDir::new().unwrap();
        let invalid_db = source_dir.path().join("invalid.sqlite3");
        {
            let opts = SqliteConnectOptions::new()
                .filename(&invalid_db)
                .create_if_missing(true);
            let p = SqlitePoolOptions::new().connect_with(opts).await.unwrap();
            sqlx::query("CREATE TABLE foo (id INTEGER PRIMARY KEY)")
                .execute(&p)
                .await
                .unwrap();
            p.close().await;
        }

        let app_dir = TempDir::new().unwrap();
        let state = AppState::new(app_dir.path().to_path_buf());

        let err = state.import_sqlite_db(&invalid_db).await.unwrap_err();
        assert!(err.contains("missing required table"));

        // Create a valid legacy-like DB without office_id on user
        let valid_db = source_dir.path().join("legacy.sqlite3");
        {
            let opts = SqliteConnectOptions::new()
                .filename(&valid_db)
                .create_if_missing(true);
            let p = SqlitePoolOptions::new().connect_with(opts).await.unwrap();
            sqlx::raw_sql(SCHEMA_SQL).execute(&p).await.unwrap();
            sqlx::query(
                "INSERT INTO users (id, email, encrypted_password, sign_in_count) VALUES (7, 'dr@cabinet.fr', '$2a$10$hash', 3)",
            )
            .execute(&p)
            .await
            .unwrap();
            p.close().await;
        }

        let status = state.import_sqlite_db(&valid_db).await.unwrap();
        assert!(status.initialized);
        assert_eq!(status.active_user.as_ref().unwrap().id, 7);
        assert!(status.active_user.as_ref().unwrap().office_id.is_some());
        assert!(status.needs_office_setup);

        // Verify export produces a valid standalone copy
        let exported = source_dir.path().join("backup.sqlite3");
        state.export_sqlite_db(&exported).await.unwrap();
        validate_sqlite_schema(&exported).await.unwrap();

        // Verify saved settings reload cleanly on new AppState
        let reloaded_state = AppState::new(app_dir.path().to_path_buf());
        let reloaded_status = reloaded_state.init_from_saved_settings().await.unwrap();
        assert!(reloaded_status.initialized);
        assert_eq!(reloaded_status.active_user.unwrap().id, 7);

        // Verify switch_user rejects unknown user_id
        let switch_err = reloaded_state.switch_user(999).await.unwrap_err();
        assert!(switch_err.contains("999"));
    }
}
