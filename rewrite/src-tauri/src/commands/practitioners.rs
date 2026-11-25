use crate::db::AppState;
use crate::error::{escape_like_pattern, AppError, ValidationBuilder};
use crate::models::{normalize_pagination, now_rails_timestamp, PaginatedResult};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tauri::State;

pub const VALID_SPECIALITY_IDS: &[i64] = &[1, 2, 3];

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PractitionerView {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub note: Option<String>,
    pub speciality_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PractitionerFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub speciality_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PractitionerInput {
    pub speciality_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub country_id: Option<i64>,
    pub note: Option<String>,
}

#[tauri::command]
pub async fn list_practitioners(
    filter: PractitionerFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<PractitionerView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_practitioners_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_practitioner(
    id: i64,
    state: State<'_, AppState>,
) -> Result<PractitionerView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_practitioner_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_practitioner(
    input: PractitionerInput,
    state: State<'_, AppState>,
) -> Result<PractitionerView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_practitioner_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_practitioner(
    id: i64,
    input: PractitionerInput,
    state: State<'_, AppState>,
) -> Result<PractitionerView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_practitioner_inner(&pool, user.id, id, input).await
}

pub async fn list_practitioners_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: PractitionerFilter,
) -> Result<PaginatedResult<PractitionerView>, AppError> {
    let (page, per_page, offset) = normalize_pagination(filter.page, filter.per_page);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM practitioners WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count practitioners: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM practitioners p WHERE p.user_id = ");
    count_qb.push_bind(user_id);
    apply_practitioner_filters(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered practitioners: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT p.id, p.first_name, p.last_name, p.phone, p.mobile_phone, p.email, p.address, p.postal_code, p.city, p.country_id, c.name AS country_name, p.note, p.speciality_id, p.created_at, p.updated_at, p.user_id FROM practitioners p LEFT JOIN countries c ON c.id = p.country_id WHERE p.user_id = ",
    );
    list_qb.push_bind(user_id);
    apply_practitioner_filters(&mut list_qb, &filter);
    list_qb.push(" ORDER BY p.id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<PractitionerView> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list practitioners: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_practitioner_filters<'a>(
    qb: &mut QueryBuilder<'a, Sqlite>,
    filter: &'a PractitionerFilter,
) {
    if let Some(spec_id) = filter.speciality_id {
        qb.push(" AND p.speciality_id = ");
        qb.push_bind(spec_id);
    }
    if let Some(ref first_name) = filter.first_name {
        if !first_name.trim().is_empty() {
            qb.push(" AND LOWER(COALESCE(p.first_name, '')) LIKE ");
            qb.push_bind(escape_like_pattern(first_name));
            qb.push(" ESCAPE '\\'");
        }
    }
    if let Some(ref last_name) = filter.last_name {
        if !last_name.trim().is_empty() {
            qb.push(" AND LOWER(COALESCE(p.last_name, '')) LIKE ");
            qb.push_bind(escape_like_pattern(last_name));
            qb.push(" ESCAPE '\\'");
        }
    }
    if let Some(ref city) = filter.city {
        if !city.trim().is_empty() {
            qb.push(" AND LOWER(COALESCE(p.city, '')) LIKE ");
            qb.push_bind(escape_like_pattern(city));
            qb.push(" ESCAPE '\\'");
        }
    }
}

pub async fn get_practitioner_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<PractitionerView, AppError> {
    sqlx::query_as::<_, PractitionerView>(
        "SELECT p.id, p.first_name, p.last_name, p.phone, p.mobile_phone, p.email, p.address, p.postal_code, p.city, p.country_id, c.name AS country_name, p.note, p.speciality_id, p.created_at, p.updated_at, p.user_id FROM practitioners p LEFT JOIN countries c ON c.id = p.country_id WHERE p.id = ? AND p.user_id = ?",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load practitioner {id}: {e}")))?
    .ok_or_else(|| AppError::not_found(format!("Practitioner {id} not found.")))
}

struct ValidatedPractitioner {
    speciality_id: i64,
    first_name: String,
    last_name: String,
    phone: Option<String>,
    mobile_phone: Option<String>,
    email: Option<String>,
    address: String,
    postal_code: String,
    city: String,
    country_id: i64,
    note: Option<String>,
}

async fn validate_practitioner_input(
    pool: &SqlitePool,
    input: PractitionerInput,
) -> Result<ValidatedPractitioner, AppError> {
    let mut v = ValidationBuilder::new();

    let speciality_id = match input.speciality_id {
        Some(id) if VALID_SPECIALITY_IDS.contains(&id) => id,
        Some(_) => {
            v.push(
                "speciality_id",
                "invalid_speciality",
                "speciality_id must be 1 (Family doctor), 2 (Physical Therapists), or 3 (Osteopath).",
            );
            0
        }
        None => {
            v.push("speciality_id", "required", "speciality_id is required.");
            0
        }
    };

    let first_name = v.required_len("first_name", &input.first_name, 2, 255);
    let last_name = v.required_len("last_name", &input.last_name, 2, 255);
    let phone = v.optional_max_len("phone", input.phone.as_deref(), 255);
    let mobile_phone = v.optional_max_len("mobile_phone", input.mobile_phone.as_deref(), 255);
    let email = v.optional_max_len("email", input.email.as_deref(), 255);
    let address = v.required_len("address", &input.address, 2, 255);
    let postal_code = v.required_len("postal_code", &input.postal_code, 2, 255);
    let city = v.required_len("city", &input.city, 2, 255);
    let note = v.optional_max_len("note", input.note.as_deref(), 5000);

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
                0
            } else {
                id
            }
        }
        _ => {
            v.push("country_id", "required", "country_id is required.");
            0
        }
    };

    v.finish()?;

    Ok(ValidatedPractitioner {
        speciality_id,
        first_name,
        last_name,
        phone,
        mobile_phone,
        email,
        address,
        postal_code,
        city,
        country_id,
        note,
    })
}

pub async fn create_practitioner_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: PractitionerInput,
) -> Result<PractitionerView, AppError> {
    let data = validate_practitioner_input(pool, input).await?;
    let now = now_rails_timestamp();

    let res = sqlx::query(
        "INSERT INTO practitioners (speciality_id, first_name, last_name, phone, mobile_phone, email, address, postal_code, city, country_id, note, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(data.speciality_id)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.phone)
    .bind(&data.mobile_phone)
    .bind(&data.email)
    .bind(&data.address)
    .bind(&data.postal_code)
    .bind(&data.city)
    .bind(data.country_id)
    .bind(&data.note)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to create practitioner: {e}")))?;

    get_practitioner_inner(pool, user_id, res.last_insert_rowid()).await
}

pub async fn update_practitioner_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: PractitionerInput,
) -> Result<PractitionerView, AppError> {
    let _existing = get_practitioner_inner(pool, user_id, id).await?;
    let data = validate_practitioner_input(pool, input).await?;
    let now = now_rails_timestamp();

    sqlx::query(
        "UPDATE practitioners SET speciality_id = ?, first_name = ?, last_name = ?, phone = ?, mobile_phone = ?, email = ?, address = ?, postal_code = ?, city = ?, country_id = ?, note = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(data.speciality_id)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.phone)
    .bind(&data.mobile_phone)
    .bind(&data.email)
    .bind(&data.address)
    .bind(&data.postal_code)
    .bind(&data.city)
    .bind(data.country_id)
    .bind(&data.note)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update practitioner {id}: {e}")))?;

    get_practitioner_inner(pool, user_id, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    fn sample_input(first_name: &str, speciality_id: Option<i64>) -> PractitionerInput {
        PractitionerInput {
            speciality_id,
            first_name: first_name.to_string(),
            last_name: "Dupont".to_string(),
            phone: Some("0102030405".to_string()),
            mobile_phone: None,
            email: Some("dupont@example.fr".to_string()),
            address: "10 Rue Victor Hugo".to_string(),
            postal_code: "69002".to_string(),
            city: "Lyon".to_string(),
            country_id: Some(75),
            note: Some("Referent".to_string()),
        }
    }

    #[tokio::test]
    async fn validates_specialities_creates_filters_and_paginates_practitioners() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        // Rejects invalid speciality and too-short fields
        let err = create_practitioner_inner(&pool, user.id, sample_input("A", Some(99)))
            .await
            .unwrap_err();
        let fields = err.validation_errors().unwrap();
        assert!(fields
            .iter()
            .any(|f| f.field == "speciality_id" && f.code == "invalid_speciality"));
        assert!(fields
            .iter()
            .any(|f| f.field == "first_name" && f.code == "too_short"));

        // Rejects note > 5000 chars
        let mut long_note = sample_input("Jean", Some(1));
        long_note.note = Some("x".repeat(5001));
        let note_err = create_practitioner_inner(&pool, user.id, long_note)
            .await
            .unwrap_err();
        assert!(note_err
            .validation_errors()
            .unwrap()
            .iter()
            .any(|f| f.field == "note" && f.code == "too_long"));

        // Create 10 practitioners (9 in Lyon, 1 in Paris)
        for i in 1..=9 {
            let mut input = sample_input(&format!("Doc{i:02}"), Some(((i - 1) % 3) + 1));
            if i == 9 {
                input.city = "Paris".to_string();
                input.speciality_id = Some(3);
            }
            create_practitioner_inner(&pool, user.id, input)
                .await
                .unwrap();
        }

        // Page 1 (8 per page)
        let page1 = list_practitioners_inner(&pool, user.id, PractitionerFilter::default())
            .await
            .unwrap();
        assert_eq!(page1.total, 9);
        assert_eq!(page1.filtered_total, 9);
        assert_eq!(page1.items.len(), 8);
        assert!(page1.items[0].country_name.is_some());

        // Page 2
        let page2 = list_practitioners_inner(
            &pool,
            user.id,
            PractitionerFilter {
                page: Some(2),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(page2.items.len(), 1);

        // Filter by city & speciality
        let filtered = list_practitioners_inner(
            &pool,
            user.id,
            PractitionerFilter {
                speciality_id: Some(3),
                city: Some("par".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(filtered.total, 9);
        assert_eq!(filtered.filtered_total, 1);
        assert_eq!(filtered.items[0].first_name.as_deref(), Some("Doc09"));

        // Update practitioner
        let target_id = filtered.items[0].id;
        let updated = update_practitioner_inner(
            &pool,
            user.id,
            target_id,
            sample_input("UpdatedDoc", Some(2)),
        )
        .await
        .unwrap();
        assert_eq!(updated.first_name.as_deref(), Some("UpdatedDoc"));
        assert_eq!(updated.speciality_id, Some(2));

        // Cross-user isolation
        let other_err = get_practitioner_inner(&pool, user.id + 999, target_id)
            .await
            .unwrap_err();
        assert!(matches!(other_err, AppError::NotFound { .. }));
    }
}
