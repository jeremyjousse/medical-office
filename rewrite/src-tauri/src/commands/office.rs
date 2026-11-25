use crate::db::{is_office_incomplete, AppState};
use crate::error::{is_valid_email, is_valid_http_url, AppError, ValidationBuilder};
use crate::models::{now_rails_timestamp, Country, Office};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeSummary {
    pub unpaid_medical_treatments_count: i64,
    pub office: Office,
    pub needs_office_setup: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OfficeInput {
    pub name: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub country_id: Option<i64>,
    pub phone: String,
    pub website: Option<String>,
    pub email: String,
}

#[tauri::command]
pub async fn get_home_summary(state: State<'_, AppState>) -> Result<HomeSummary, AppError> {
    let (pool, user, office) = state.require_active_context().await?;
    get_home_summary_inner(&pool, user.id, office).await
}

#[tauri::command]
pub async fn list_countries(state: State<'_, AppState>) -> Result<Vec<Country>, AppError> {
    let pool = state.require_pool().await?;
    list_countries_inner(&pool).await
}

#[tauri::command]
pub async fn get_office(state: State<'_, AppState>) -> Result<Office, AppError> {
    let (_, _, office) = state.require_active_context().await?;
    Ok(office)
}

#[tauri::command]
pub async fn update_office(
    input: OfficeInput,
    state: State<'_, AppState>,
) -> Result<Office, AppError> {
    let (pool, _, office) = state.require_active_context().await?;
    update_office_inner(&pool, office.id, input).await
}

pub async fn get_home_summary_inner(
    pool: &SqlitePool,
    user_id: i64,
    office: Office,
) -> Result<HomeSummary, AppError> {
    let unpaid_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM medical_treatments WHERE user_id = ? AND status = 0",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to count unpaid medical treatments: {e}")))?;

    let needs_office_setup = is_office_incomplete(&office);
    Ok(HomeSummary {
        unpaid_medical_treatments_count: unpaid_count,
        office,
        needs_office_setup,
    })
}

pub async fn list_countries_inner(pool: &SqlitePool) -> Result<Vec<Country>, AppError> {
    sqlx::query_as::<_, Country>("SELECT id, name, iso FROM countries ORDER BY name ASC")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list countries: {e}")))
}

pub async fn update_office_inner(
    pool: &SqlitePool,
    office_id: i64,
    input: OfficeInput,
) -> Result<Office, AppError> {
    let mut v = ValidationBuilder::new();
    let name = v.required_len("name", &input.name, 2, 255);
    let address = v.required_len("address", &input.address, 2, 255);
    let postal_code = v.required_len("postal_code", &input.postal_code, 2, 100);
    let city = v.required_len("city", &input.city, 2, 255);
    let phone = v.required_len("phone", &input.phone, 5, 100);
    let email = v.required_len("email", &input.email, 5, 255);
    if !email.is_empty() && !is_valid_email(&email) {
        v.push("email", "invalid_email", "email format is invalid.");
    }

    let website = v.optional_max_len("website", input.website.as_deref(), 255);
    if let Some(ref url) = website {
        if !is_valid_http_url(url) {
            v.push(
                "website",
                "invalid_url",
                "website must start with http:// or https://.",
            );
        }
    }

    let country_id = match input.country_id {
        Some(id) if id > 0 => {
            let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM countries WHERE id = ?")
                .bind(id)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::message(format!("Failed to verify country: {e}")))?;
            if exists.is_none() {
                v.push(
                    "country_id",
                    "invalid_country",
                    "Selected country is invalid.",
                );
                None
            } else {
                Some(id)
            }
        }
        _ => {
            v.push("country_id", "required", "country_id is required.");
            None
        }
    };

    v.finish()?;

    let now = now_rails_timestamp();
    let rows_affected = sqlx::query(
        "UPDATE offices SET name = ?, address = ?, postal_code = ?, city = ?, country_id = ?, phone = ?, website = ?, email = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&name)
    .bind(&address)
    .bind(&postal_code)
    .bind(&city)
    .bind(country_id)
    .bind(&phone)
    .bind(&website)
    .bind(&email)
    .bind(&now)
    .bind(office_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update office: {e}")))?
    .rows_affected();

    if rows_affected == 0 {
        return Err(AppError::not_found("Office not found."));
    }

    sqlx::query_as::<_, Office>(
        "SELECT id, name, address, city, postal_code, country_id, phone, website, email, created_at, updated_at FROM offices WHERE id = ?",
    )
    .bind(office_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to fetch updated office: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn validates_and_updates_office_and_counts_unpaid_treatments() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, office) = resolve_active_user(&pool, None).await.unwrap();

        let summary = get_home_summary_inner(&pool, user.id, office.clone())
            .await
            .unwrap();
        assert!(summary.needs_office_setup);
        assert_eq!(summary.unpaid_medical_treatments_count, 0);

        // Insert 2 unpaid and 1 paid treatments for user
        sqlx::query(
            "INSERT INTO medical_treatments (user_id, status) VALUES (?, 0), (?, 0), (?, 1)",
        )
        .bind(user.id)
        .bind(user.id)
        .bind(user.id)
        .execute(&pool)
        .await
        .unwrap();

        // Invalid office update
        let err = update_office_inner(
            &pool,
            office.id,
            OfficeInput {
                name: "A".to_string(),
                address: "".to_string(),
                postal_code: "75001".to_string(),
                city: "Paris".to_string(),
                country_id: None,
                phone: "123".to_string(),
                website: Some("ftp://bad".to_string()),
                email: "not-an-email".to_string(),
            },
        )
        .await
        .unwrap_err();
        let fields = err.validation_errors().unwrap();
        assert!(fields
            .iter()
            .any(|f| f.field == "name" && f.code == "too_short"));
        assert!(fields
            .iter()
            .any(|f| f.field == "address" && f.code == "required"));
        assert!(fields
            .iter()
            .any(|f| f.field == "country_id" && f.code == "required"));
        assert!(fields
            .iter()
            .any(|f| f.field == "phone" && f.code == "too_short"));
        assert!(fields
            .iter()
            .any(|f| f.field == "website" && f.code == "invalid_url"));
        assert!(fields
            .iter()
            .any(|f| f.field == "email" && f.code == "invalid_email"));

        // Valid office update
        let updated = update_office_inner(
            &pool,
            office.id,
            OfficeInput {
                name: "Cabinet Podologie".to_string(),
                address: "12 Rue de la Paix".to_string(),
                postal_code: "75002".to_string(),
                city: "Paris".to_string(),
                country_id: Some(75),
                phone: "0142685500".to_string(),
                website: Some("https://cabinet.example.fr".to_string()),
                email: "contact@cabinet.fr".to_string(),
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.name.as_deref(), Some("Cabinet Podologie"));

        let summary_after = get_home_summary_inner(&pool, user.id, updated)
            .await
            .unwrap();
        assert!(!summary_after.needs_office_setup);
        assert_eq!(summary_after.unpaid_medical_treatments_count, 2);

        let countries = list_countries_inner(&pool).await.unwrap();
        assert_eq!(countries.len(), 251);
    }
}
