use crate::db::AppState;
use crate::error::{escape_like_pattern, AppError, ValidationBuilder};
use crate::models::{
    from_cents, normalize_pagination, now_rails_timestamp, to_cents, MedicalTreatmentType,
    PaginatedResult,
};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TreatmentTypeFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TreatmentTypeInput {
    pub name: String,
    pub price: Option<f64>,
}

#[tauri::command]
pub async fn list_treatment_types(
    filter: TreatmentTypeFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<MedicalTreatmentType>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_treatment_types_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_treatment_type(
    id: i64,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentType, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_treatment_type_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_treatment_type(
    input: TreatmentTypeInput,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentType, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_treatment_type_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_treatment_type(
    id: i64,
    input: TreatmentTypeInput,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentType, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_treatment_type_inner(&pool, user.id, id, input).await
}

#[tauri::command]
pub async fn delete_treatment_type(id: i64, state: State<'_, AppState>) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    delete_treatment_type_inner(&pool, user.id, id).await
}

pub async fn list_treatment_types_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: TreatmentTypeFilter,
) -> Result<PaginatedResult<MedicalTreatmentType>, AppError> {
    let (page, per_page, offset) = normalize_pagination(filter.page, filter.per_page);

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM medical_treatment_types WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::message(format!("Failed to count treatment types: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM medical_treatment_types WHERE user_id = ");
    count_qb.push_bind(user_id);
    apply_treatment_type_filter(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered treatment types: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, name, price, user_id, created_at, updated_at FROM medical_treatment_types WHERE user_id = ",
    );
    list_qb.push_bind(user_id);
    apply_treatment_type_filter(&mut list_qb, &filter);
    list_qb.push(" ORDER BY id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<MedicalTreatmentType> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list treatment types: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_treatment_type_filter<'a>(
    qb: &mut QueryBuilder<'a, Sqlite>,
    filter: &'a TreatmentTypeFilter,
) {
    if let Some(ref name) = filter.name {
        if !name.trim().is_empty() {
            qb.push(" AND LOWER(COALESCE(name, '')) LIKE ");
            qb.push_bind(escape_like_pattern(name));
            qb.push(" ESCAPE '\\'");
        }
    }
}

pub async fn get_treatment_type_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<MedicalTreatmentType, AppError> {
    sqlx::query_as::<_, MedicalTreatmentType>(
        "SELECT id, name, price, user_id, created_at, updated_at FROM medical_treatment_types WHERE id = ? AND user_id = ?",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load treatment type {id}: {e}")))?
    .ok_or_else(|| AppError::not_found(format!("Medical treatment type {id} not found.")))
}

fn validate_treatment_type_input(input: TreatmentTypeInput) -> Result<(String, f64), AppError> {
    let mut v = ValidationBuilder::new();
    let name = v.required_len("name", &input.name, 2, 255);

    let price = match input.price {
        Some(p) if p.is_finite() && p >= 0.0 => from_cents(to_cents(p)),
        Some(_) => {
            v.push(
                "price",
                "invalid_price",
                "price must be a valid non-negative number.",
            );
            0.0
        }
        None => {
            v.push("price", "required", "price is required.");
            0.0
        }
    };

    v.finish()?;
    Ok((name, price))
}

pub async fn create_treatment_type_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: TreatmentTypeInput,
) -> Result<MedicalTreatmentType, AppError> {
    let (name, price) = validate_treatment_type_input(input)?;
    let now = now_rails_timestamp();

    let res = sqlx::query(
        "INSERT INTO medical_treatment_types (name, price, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&name)
    .bind(price)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to create treatment type: {e}")))?;

    get_treatment_type_inner(pool, user_id, res.last_insert_rowid()).await
}

pub async fn update_treatment_type_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: TreatmentTypeInput,
) -> Result<MedicalTreatmentType, AppError> {
    let _existing = get_treatment_type_inner(pool, user_id, id).await?;
    let (name, price) = validate_treatment_type_input(input)?;
    let now = now_rails_timestamp();

    sqlx::query(
        "UPDATE medical_treatment_types SET name = ?, price = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(&name)
    .bind(price)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update treatment type {id}: {e}")))?;

    get_treatment_type_inner(pool, user_id, id).await
}

pub async fn delete_treatment_type_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let _existing = get_treatment_type_inner(pool, user_id, id).await?;

    let ref_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM medical_treatments WHERE medical_treatment_type_id = ? OR treatment_type_id = ?",
    )
    .bind(id)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to check treatment references: {e}")))?;

    if ref_count > 0 {
        return Err(AppError::blocked(
            "referenced_by_treatments",
            "Cannot delete medical treatment type because it is referenced by existing medical treatments.",
        ));
    }

    sqlx::query("DELETE FROM medical_treatment_types WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete treatment type {id}: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn crud_and_restrict_delete_for_treatment_types() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        // Validation error
        let err = create_treatment_type_inner(
            &pool,
            user.id,
            TreatmentTypeInput {
                name: "A".to_string(),
                price: Some(-5.0),
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
            .any(|f| f.field == "price" && f.code == "invalid_price"));

        let created = create_treatment_type_inner(
            &pool,
            user.id,
            TreatmentTypeInput {
                name: "Bilan podologique".to_string(),
                price: Some(55.5),
            },
        )
        .await
        .unwrap();
        assert_eq!(created.name.as_deref(), Some("Bilan podologique"));
        assert_eq!(created.price.map(|p| p.0), Some(55.5));

        let unreferenced = create_treatment_type_inner(
            &pool,
            user.id,
            TreatmentTypeInput {
                name: "Semelles".to_string(),
                price: Some(120.0),
            },
        )
        .await
        .unwrap();

        // Reference first type from medical_treatments
        sqlx::query(
            "INSERT INTO medical_treatments (user_id, medical_treatment_type_id, status) VALUES (?, ?, 0)",
        )
        .bind(user.id)
        .bind(created.id)
        .execute(&pool)
        .await
        .unwrap();

        let blocked = delete_treatment_type_inner(&pool, user.id, created.id)
            .await
            .unwrap_err();
        assert!(matches!(
            blocked,
            AppError::Blocked { ref code, .. } if code == "referenced_by_treatments"
        ));

        // Unreferenced type deletes cleanly
        delete_treatment_type_inner(&pool, user.id, unreferenced.id)
            .await
            .unwrap();

        let listed = list_treatment_types_inner(
            &pool,
            user.id,
            TreatmentTypeFilter {
                name: Some("bilan".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(listed.total, 1);
        assert_eq!(listed.filtered_total, 1);
    }
}
