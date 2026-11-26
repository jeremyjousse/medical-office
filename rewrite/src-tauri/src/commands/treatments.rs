use crate::commands::treatment_types::get_treatment_type_inner;
use crate::db::AppState;
use crate::error::{AppError, ValidationBuilder};
use crate::models::{
    from_cents, normalize_listing_pagination, normalize_rails_date, now_rails_timestamp, to_cents,
    MedicalTreatmentType, PaginatedResult, Payment, RailsDecimal,
};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tauri::State;

pub const DEFAULT_TREATMENTS_PER_PAGE: i64 = 10;
pub const VALID_LOCATION_IDS: &[i64] = &[1, 2];

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MedicalTreatmentView {
    pub id: i64,
    pub user_id: Option<i64>,
    pub patient_id: Option<i64>,
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub patient_city: Option<String>,
    pub date: Option<String>,
    pub location_id: Option<i64>,
    pub medical_treatment_type_id: Option<i64>,
    pub treatment_type_name: Option<String>,
    pub treatment_type_price: Option<RailsDecimal>,
    pub price: Option<RailsDecimal>,
    pub note: Option<String>,
    pub status: Option<i64>,
    pub payments_count: i64,
    pub payments_total: RailsDecimal,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalTreatmentDetail {
    pub treatment: MedicalTreatmentView,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TreatmentFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub patient_id: Option<i64>,
    pub medical_treatment_type_id: Option<i64>,
    pub date_gteq: Option<String>,
    pub date_lteq: Option<String>,
    pub location_id: Option<i64>,
    pub status: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MedicalTreatmentInput {
    pub patient_id: Option<i64>,
    pub date: String,
    pub location_id: Option<i64>,
    pub medical_treatment_type_id: Option<i64>,
    pub price: Option<f64>,
    pub note: Option<String>,
}

const TREATMENT_SELECT_SQL: &str = "
SELECT
    mt.id,
    mt.user_id,
    mt.patient_id,
    p.first_name AS patient_first_name,
    p.last_name AS patient_last_name,
    p.city AS patient_city,
    mt.date,
    mt.location_id,
    mt.medical_treatment_type_id,
    mtt.name AS treatment_type_name,
    mtt.price AS treatment_type_price,
    mt.price,
    mt.note,
    mt.status,
    COALESCE(pay.cnt, 0) AS payments_count,
    COALESCE(pay.total_amount, 0.0) AS payments_total,
    mt.created_at,
    mt.updated_at
FROM medical_treatments mt
LEFT JOIN patients p ON p.id = mt.patient_id
LEFT JOIN medical_treatment_types mtt ON mtt.id = mt.medical_treatment_type_id
LEFT JOIN (
    SELECT medical_treatment_id, COUNT(*) AS cnt, COALESCE(SUM(amount), 0.0) AS total_amount
    FROM payments
    GROUP BY medical_treatment_id
) pay ON pay.medical_treatment_id = mt.id
WHERE mt.user_id = ";

#[tauri::command]
pub async fn list_treatments(
    filter: TreatmentFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<MedicalTreatmentView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_treatments_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_treatment(
    id: i64,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentDetail, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_treatment_detail_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_treatment(
    input: MedicalTreatmentInput,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_treatment_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_treatment(
    id: i64,
    input: MedicalTreatmentInput,
    state: State<'_, AppState>,
) -> Result<MedicalTreatmentView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_treatment_inner(&pool, user.id, id, input).await
}

#[tauri::command]
pub async fn delete_treatment(id: i64, state: State<'_, AppState>) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    delete_treatment_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn price_finder(
    medical_treatment_type_id: i64,
    state: State<'_, AppState>,
) -> Result<Option<f64>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    price_finder_inner(&pool, user.id, medical_treatment_type_id).await
}

#[tauri::command]
pub async fn list_treatment_type_options(
    state: State<'_, AppState>,
) -> Result<Vec<MedicalTreatmentType>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_treatment_type_options_inner(&pool, user.id).await
}

pub async fn list_treatments_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: TreatmentFilter,
) -> Result<PaginatedResult<MedicalTreatmentView>, AppError> {
    let (page, per_page, offset) =
        normalize_listing_pagination(filter.page, filter.per_page, DEFAULT_TREATMENTS_PER_PAGE);

    let mut total_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM medical_treatments mt WHERE mt.user_id = ");
    total_qb.push_bind(user_id);
    if let Some(pid) = filter.patient_id {
        total_qb.push(" AND mt.patient_id = ");
        total_qb.push_bind(pid);
    }
    let total: i64 = total_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count treatments: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM medical_treatments mt WHERE mt.user_id = ");
    count_qb.push_bind(user_id);
    apply_treatment_filters(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered treatments: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(TREATMENT_SELECT_SQL);
    list_qb.push_bind(user_id);
    apply_treatment_filters(&mut list_qb, &filter);
    list_qb.push(" ORDER BY mt.date DESC, mt.id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<MedicalTreatmentView> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list treatments: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_treatment_filters<'a>(qb: &mut QueryBuilder<'a, Sqlite>, filter: &'a TreatmentFilter) {
    if let Some(patient_id) = filter.patient_id {
        qb.push(" AND mt.patient_id = ");
        qb.push_bind(patient_id);
    }
    if let Some(type_id) = filter.medical_treatment_type_id {
        qb.push(" AND mt.medical_treatment_type_id = ");
        qb.push_bind(type_id);
    }
    if let Some(ref gteq) = filter.date_gteq {
        if let Some(norm) = normalize_rails_date(gteq) {
            qb.push(" AND SUBSTR(COALESCE(mt.date, ''), 1, 10) >= ");
            qb.push_bind(norm);
        }
    }
    if let Some(ref lteq) = filter.date_lteq {
        if let Some(norm) = normalize_rails_date(lteq) {
            qb.push(" AND SUBSTR(COALESCE(mt.date, ''), 1, 10) <= ");
            qb.push_bind(norm);
        }
    }
    if let Some(location_id) = filter.location_id {
        qb.push(" AND mt.location_id = ");
        qb.push_bind(location_id);
    }
    if let Some(status) = filter.status {
        qb.push(" AND COALESCE(mt.status, 0) = ");
        qb.push_bind(status);
    }
}

pub async fn get_treatment_view_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<MedicalTreatmentView, AppError> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(TREATMENT_SELECT_SQL);
    qb.push_bind(user_id);
    qb.push(" AND mt.id = ");
    qb.push_bind(id);

    qb.build_query_as::<MedicalTreatmentView>()
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to load treatment {id}: {e}")))?
        .ok_or_else(|| AppError::not_found(format!("Medical treatment {id} not found.")))
}

pub async fn get_treatment_detail_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<MedicalTreatmentDetail, AppError> {
    let treatment = get_treatment_view_inner(pool, user_id, id).await?;
    let payments = sqlx::query_as::<_, Payment>(
        "SELECT id, medical_treatment_id, paid_at, payment_type, amount, user_id, bank_deposit_id, created_at, updated_at FROM payments WHERE medical_treatment_id = ? ORDER BY paid_at DESC, id DESC",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load payments for treatment {id}: {e}")))?;

    Ok(MedicalTreatmentDetail {
        treatment,
        payments,
    })
}

pub async fn price_finder_inner(
    pool: &SqlitePool,
    user_id: i64,
    medical_treatment_type_id: i64,
) -> Result<Option<f64>, AppError> {
    let tt = get_treatment_type_inner(pool, user_id, medical_treatment_type_id).await?;
    Ok(tt.price.map(|p| p.0))
}

pub async fn list_treatment_type_options_inner(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<MedicalTreatmentType>, AppError> {
    sqlx::query_as::<_, MedicalTreatmentType>(
        "SELECT id, name, price, user_id, created_at, updated_at FROM medical_treatment_types WHERE user_id = ? ORDER BY LOWER(COALESCE(name, '')) ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list treatment type options: {e}")))
}

/// Computes `sum(payments.amount)` in integer cents, enforces `sum <= price`, and returns `(status)` (`1: paid` when `sum == price`, `0: unpaid` otherwise).
pub async fn compute_treatment_status_from_payments(
    pool: &SqlitePool,
    treatment_id: Option<i64>,
    price_cents: i64,
    v: &mut ValidationBuilder,
) -> Result<i64, AppError> {
    let total_payments_cents: i64 = if let Some(tid) = treatment_id {
        let amounts: Vec<RailsDecimal> =
            sqlx::query_scalar("SELECT amount FROM payments WHERE medical_treatment_id = ?")
                .bind(tid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::message(format!("Failed to sum payments: {e}")))?;
        amounts.iter().map(|a| to_cents(a.0)).sum()
    } else {
        0
    };

    if total_payments_cents > price_cents {
        v.push(
            "price",
            "payments_exceed_price",
            "Total payments cannot exceed treatment price.",
        );
    }

    Ok(if total_payments_cents == price_cents {
        1
    } else {
        0
    })
}

struct ValidatedTreatment {
    patient_id: i64,
    date: String,
    location_id: i64,
    medical_treatment_type_id: i64,
    price: f64,
    note: Option<String>,
    status: i64,
}

async fn validate_treatment_input(
    pool: &SqlitePool,
    user_id: i64,
    existing_treatment_id: Option<i64>,
    input: MedicalTreatmentInput,
) -> Result<ValidatedTreatment, AppError> {
    let mut v = ValidationBuilder::new();

    let patient_id = match input.patient_id {
        Some(pid) if pid > 0 => {
            let exists: Option<i64> =
                sqlx::query_scalar("SELECT id FROM patients WHERE id = ? AND user_id = ?")
                    .bind(pid)
                    .bind(user_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| AppError::message(format!("Failed to verify patient: {e}")))?;
            if exists.is_none() {
                v.push(
                    "patient_id",
                    "invalid_patient",
                    "Selected patient is invalid.",
                );
                0
            } else {
                pid
            }
        }
        _ => {
            v.push("patient_id", "required", "patient_id is required.");
            0
        }
    };

    let date = if input.date.trim().is_empty() {
        v.push("date", "required", "date is required.");
        String::new()
    } else if let Some(norm) = normalize_rails_date(&input.date) {
        norm
    } else {
        v.push(
            "date",
            "invalid_date",
            "date must be a valid YYYY-MM-DD date.",
        );
        String::new()
    };

    let location_id = match input.location_id {
        Some(loc) if VALID_LOCATION_IDS.contains(&loc) => loc,
        Some(_) => {
            v.push(
                "location_id",
                "invalid_location",
                "location_id must be 1 (Office) or 2 (Home).",
            );
            0
        }
        None => {
            v.push("location_id", "required", "location_id is required.");
            0
        }
    };

    let medical_treatment_type_id = match input.medical_treatment_type_id {
        Some(tid) if tid > 0 => {
            let exists: Option<i64> = sqlx::query_scalar(
                "SELECT id FROM medical_treatment_types WHERE id = ? AND user_id = ?",
            )
            .bind(tid)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::message(format!("Failed to verify treatment type: {e}")))?;
            if exists.is_none() {
                v.push(
                    "medical_treatment_type_id",
                    "invalid_treatment_type",
                    "Selected medical treatment type is invalid.",
                );
                0
            } else {
                tid
            }
        }
        _ => {
            v.push(
                "medical_treatment_type_id",
                "required",
                "medical_treatment_type_id is required.",
            );
            0
        }
    };

    let (price, price_cents) = match input.price {
        Some(p) if p.is_finite() && p >= 0.0 => {
            let cents = to_cents(p);
            (from_cents(cents), cents)
        }
        Some(_) => {
            v.push(
                "price",
                "invalid_price",
                "price must be a valid non-negative number.",
            );
            (0.0, 0)
        }
        None => {
            v.push("price", "required", "price is required.");
            (0.0, 0)
        }
    };

    let note = v.optional_max_len("note", input.note.as_deref(), 5000);

    let status =
        compute_treatment_status_from_payments(pool, existing_treatment_id, price_cents, &mut v)
            .await?;

    v.finish()?;

    Ok(ValidatedTreatment {
        patient_id,
        date,
        location_id,
        medical_treatment_type_id,
        price,
        note,
        status,
    })
}

pub async fn create_treatment_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: MedicalTreatmentInput,
) -> Result<MedicalTreatmentView, AppError> {
    let data = validate_treatment_input(pool, user_id, None, input).await?;
    let now = now_rails_timestamp();

    let res = sqlx::query(
        "INSERT INTO medical_treatments (user_id, patient_id, date, location_id, medical_treatment_type_id, price, note, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(data.patient_id)
    .bind(&data.date)
    .bind(data.location_id)
    .bind(data.medical_treatment_type_id)
    .bind(data.price)
    .bind(&data.note)
    .bind(data.status)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to create medical treatment: {e}")))?;

    get_treatment_view_inner(pool, user_id, res.last_insert_rowid()).await
}

pub async fn update_treatment_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: MedicalTreatmentInput,
) -> Result<MedicalTreatmentView, AppError> {
    let _existing = get_treatment_view_inner(pool, user_id, id).await?;
    let data = validate_treatment_input(pool, user_id, Some(id), input).await?;
    let now = now_rails_timestamp();

    sqlx::query(
        "UPDATE medical_treatments SET patient_id = ?, date = ?, location_id = ?, medical_treatment_type_id = ?, price = ?, note = ?, status = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(data.patient_id)
    .bind(&data.date)
    .bind(data.location_id)
    .bind(data.medical_treatment_type_id)
    .bind(data.price)
    .bind(&data.note)
    .bind(data.status)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update medical treatment {id}: {e}")))?;

    get_treatment_view_inner(pool, user_id, id).await
}

pub async fn delete_treatment_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let _existing = get_treatment_view_inner(pool, user_id, id).await?;

    let payments_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM payments WHERE medical_treatment_id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::message(format!("Failed to verify payments: {e}")))?;

    if payments_count > 0 {
        return Err(AppError::blocked(
            "treatment_has_payments",
            "Cannot delete medical treatment because it has existing payments.",
        ));
    }

    sqlx::query("DELETE FROM medical_treatments WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete medical treatment {id}: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::patients::{create_patient_inner, PatientInput};
    use crate::commands::treatment_types::{create_treatment_type_inner, TreatmentTypeInput};
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_fixture() -> (SqlitePool, i64, i64, i64) {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        let patient = create_patient_inner(
            &pool,
            user.id,
            PatientInput {
                first_name: Some("Alice".to_string()),
                last_name: "Martin".to_string(),
                birthdate: None,
                profession: None,
                phone: None,
                mobile_phone: None,
                email: None,
                address: None,
                postal_code: None,
                city: "Lyon".to_string(),
                country_id: Some(75),
                national_insurance_number: None,
                family_doctor_id: None,
                physical_therapists_id: None,
                osteopath_id: None,
                legal_guardian_id: None,
                note: None,
                diabetic: false,
                universal_healthcare_coverage: false,
                long_duration_disease: false,
                diabete_network_id: None,
            },
        )
        .await
        .unwrap();

        let tt = create_treatment_type_inner(
            &pool,
            user.id,
            TreatmentTypeInput {
                name: "Soin podologique".to_string(),
                price: Some(50.0),
            },
        )
        .await
        .unwrap();

        (pool, user.id, patient.id, tt.id)
    }

    #[tokio::test]
    async fn enforces_payment_sum_rule_auto_status_and_delete_guard() {
        let (pool, user_id, patient_id, tt_id) = setup_fixture().await;

        assert_eq!(
            price_finder_inner(&pool, user_id, tt_id).await.unwrap(),
            Some(50.0)
        );

        let created = create_treatment_inner(
            &pool,
            user_id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-10".to_string(),
                location_id: Some(1),
                medical_treatment_type_id: Some(tt_id),
                price: Some(50.0),
                note: Some("First visit".to_string()),
            },
        )
        .await
        .unwrap();
        assert_eq!(created.status, Some(0)); // unpaid when payments == 0 and price == 50

        // Add partial payment of 20.00 -> still unpaid on update
        sqlx::query(
            "INSERT INTO payments (medical_treatment_id, paid_at, payment_type, amount, user_id) VALUES (?, '2026-03-10', 1, 20.0, ?)",
        )
        .bind(created.id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

        let still_unpaid = update_treatment_inner(
            &pool,
            user_id,
            created.id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-10".to_string(),
                location_id: Some(1),
                medical_treatment_type_id: Some(tt_id),
                price: Some(50.0),
                note: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(still_unpaid.status, Some(0));
        assert_eq!(still_unpaid.payments_count, 1);

        // Cannot delete when payments_count > 0
        let blocked = delete_treatment_inner(&pool, user_id, created.id)
            .await
            .unwrap_err();
        assert!(matches!(
            blocked,
            AppError::Blocked { ref code, .. } if code == "treatment_has_payments"
        ));

        // Add second payment of 30.00 (total 50.00) -> status auto-updates to 1 (paid)
        sqlx::query(
            "INSERT INTO payments (medical_treatment_id, paid_at, payment_type, amount, user_id) VALUES (?, '2026-03-11', 2, 30.0, ?)",
        )
        .bind(created.id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

        let now_paid = update_treatment_inner(
            &pool,
            user_id,
            created.id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-10".to_string(),
                location_id: Some(2),
                medical_treatment_type_id: Some(tt_id),
                price: Some(50.0),
                note: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(now_paid.status, Some(1));
        assert_eq!(now_paid.location_id, Some(2));

        // Lowering price below sum(payments.amount) (50.00 > 45.00) is rejected
        let exceed_err = update_treatment_inner(
            &pool,
            user_id,
            created.id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-10".to_string(),
                location_id: Some(2),
                medical_treatment_type_id: Some(tt_id),
                price: Some(45.0),
                note: None,
            },
        )
        .await
        .unwrap_err();
        assert!(exceed_err
            .validation_errors()
            .unwrap()
            .iter()
            .any(|f| f.field == "price" && f.code == "payments_exceed_price"));

        // Raising price above sum(payments.amount) reverts status to 0 (unpaid)
        let reverted = update_treatment_inner(
            &pool,
            user_id,
            created.id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-10".to_string(),
                location_id: Some(2),
                medical_treatment_type_id: Some(tt_id),
                price: Some(60.0),
                note: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(reverted.status, Some(0));

        // Zero-price treatment with 0 payments is immediately paid (0 == 0)
        let free_treatment = create_treatment_inner(
            &pool,
            user_id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-03-15".to_string(),
                location_id: Some(1),
                medical_treatment_type_id: Some(tt_id),
                price: Some(0.0),
                note: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(free_treatment.status, Some(1));

        // Filtering by date range and status
        let filtered = list_treatments_inner(
            &pool,
            user_id,
            TreatmentFilter {
                patient_id: Some(patient_id),
                date_gteq: Some("2026-03-12".to_string()),
                date_lteq: Some("2026-03-20".to_string()),
                status: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(filtered.total, 2);
        assert_eq!(filtered.filtered_total, 1);
        assert_eq!(filtered.items[0].id, free_treatment.id);

        // Unpaid treatment with 0 payments deletes cleanly
        delete_treatment_inner(&pool, user_id, free_treatment.id)
            .await
            .unwrap();
    }
}
