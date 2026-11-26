use crate::db::AppState;
use crate::error::{AppError, ValidationBuilder};
use crate::models::{
    from_cents, normalize_listing_pagination, now_rails_timestamp, to_cents, today_date_str,
    BankAccount, Office, PaginatedResult, RailsDecimal,
};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use std::collections::HashSet;
use tauri::State;

pub const DEFAULT_BANK_DEPOSITS_PER_PAGE: i64 = 10;
pub const VALID_DEPOSIT_TYPES: &[i64] = &[1, 2];

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankDepositView {
    pub id: i64,
    pub deposit_type: Option<i64>,
    pub number: Option<i64>,
    pub amount: Option<RailsDecimal>,
    pub user_id: Option<i64>,
    pub deposit_date: Option<String>,
    pub bank_account_id: Option<i64>,
    pub bank_account_name: Option<String>,
    pub bank_account_number: Option<i64>,
    pub checks_count: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankCheckDepositRow {
    pub id: i64,
    pub account_owner: Option<String>,
    pub bank_name: Option<String>,
    pub check_number: Option<String>,
    pub amount: Option<RailsDecimal>,
    pub status: Option<i64>,
    pub bank_deposit_id: Option<i64>,
    pub payment_id: Option<i64>,
    pub paid_at: Option<String>,
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDepositDetail {
    pub deposit: BankDepositView,
    pub office: Office,
    pub checks: Vec<BankCheckDepositRow>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BankDepositFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub deposit_type: Option<i64>,
    pub number: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BankDepositInput {
    pub deposit_type: Option<i64>,
    pub bank_account_id: Option<i64>,
    pub payment_bank_check_ids: Vec<i64>,
}

const DEPOSIT_SELECT_SQL: &str = "
SELECT
    bd.id,
    bd.deposit_type,
    bd.number,
    bd.amount,
    bd.user_id,
    bd.deposit_date,
    bd.bank_account_id,
    ba.bank_name AS bank_account_name,
    ba.account_number AS bank_account_number,
    COALESCE(chk.cnt, 0) AS checks_count,
    bd.created_at,
    bd.updated_at
FROM bank_deposits bd
LEFT JOIN bank_accounts ba ON ba.id = bd.bank_account_id
LEFT JOIN (
    SELECT bank_deposit_id, COUNT(*) AS cnt
    FROM payment_bank_checks
    GROUP BY bank_deposit_id
) chk ON chk.bank_deposit_id = bd.id
WHERE bd.user_id = ";

#[tauri::command]
pub async fn list_bank_deposits(
    filter: BankDepositFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<BankDepositView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_bank_deposits_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_bank_deposit(
    id: i64,
    state: State<'_, AppState>,
) -> Result<BankDepositDetail, AppError> {
    let (pool, user, office) = state.require_active_context().await?;
    get_bank_deposit_inner(&pool, user.id, office, id).await
}

#[tauri::command]
pub async fn list_pending_bank_checks(
    state: State<'_, AppState>,
) -> Result<Vec<BankCheckDepositRow>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_pending_bank_checks_inner(&pool, user.id, &today_date_str()).await
}

#[tauri::command]
pub async fn list_bank_account_options(
    state: State<'_, AppState>,
) -> Result<Vec<BankAccount>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_bank_account_options_inner(&pool, user.id).await
}

#[tauri::command]
pub async fn create_bank_deposit(
    input: BankDepositInput,
    state: State<'_, AppState>,
) -> Result<BankDepositDetail, AppError> {
    let (pool, user, office) = state.require_active_context().await?;
    create_bank_deposit_inner(&pool, user.id, office, input).await
}

#[tauri::command]
pub async fn delete_bank_deposit(id: i64, state: State<'_, AppState>) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    delete_bank_deposit_inner(&pool, user.id, id).await
}

pub async fn list_bank_deposits_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: BankDepositFilter,
) -> Result<PaginatedResult<BankDepositView>, AppError> {
    let (page, per_page, offset) =
        normalize_listing_pagination(filter.page, filter.per_page, DEFAULT_BANK_DEPOSITS_PER_PAGE);

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM bank_deposits bd WHERE bd.user_id = ?")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::message(format!("Failed to count bank deposits: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM bank_deposits bd WHERE bd.user_id = ");
    count_qb.push_bind(user_id);
    apply_bank_deposit_filters(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered bank deposits: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(DEPOSIT_SELECT_SQL);
    list_qb.push_bind(user_id);
    apply_bank_deposit_filters(&mut list_qb, &filter);
    list_qb.push(" ORDER BY bd.created_at DESC, bd.id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<BankDepositView> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list bank deposits: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_bank_deposit_filters<'a>(
    qb: &mut QueryBuilder<'a, Sqlite>,
    filter: &'a BankDepositFilter,
) {
    if let Some(deposit_type) = filter.deposit_type {
        qb.push(" AND bd.deposit_type = ");
        qb.push_bind(deposit_type);
    }
    if let Some(number) = filter.number {
        qb.push(" AND bd.number = ");
        qb.push_bind(number);
    }
}

pub async fn get_bank_deposit_view_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<BankDepositView, AppError> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(DEPOSIT_SELECT_SQL);
    qb.push_bind(user_id);
    qb.push(" AND bd.id = ");
    qb.push_bind(id);

    qb.build_query_as::<BankDepositView>()
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to load bank deposit {id}: {e}")))?
        .ok_or_else(|| AppError::not_found(format!("Bank deposit {id} not found.")))
}

pub async fn get_bank_deposit_inner(
    pool: &SqlitePool,
    user_id: i64,
    office: Office,
    id: i64,
) -> Result<BankDepositDetail, AppError> {
    let deposit = get_bank_deposit_view_inner(pool, user_id, id).await?;

    let checks = sqlx::query_as::<_, BankCheckDepositRow>(
        "SELECT
            pbc.id,
            pbc.account_owner,
            pbc.bank_name,
            pbc.check_number,
            pbc.amount,
            pbc.status,
            pbc.bank_deposit_id,
            pbc.payment_id,
            SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) AS paid_at,
            pat.first_name AS patient_first_name,
            pat.last_name AS patient_last_name
         FROM payment_bank_checks pbc
         INNER JOIN payments pay ON pay.id = pbc.payment_id
         LEFT JOIN medical_treatments mt ON mt.id = pay.medical_treatment_id
         LEFT JOIN patients pat ON pat.id = mt.patient_id
         WHERE pbc.bank_deposit_id = ? AND pbc.user_id = ?
         ORDER BY SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) ASC, pbc.id ASC",
    )
    .bind(id)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load deposit checks: {e}")))?;

    Ok(BankDepositDetail {
        deposit,
        office,
        checks,
    })
}

pub async fn list_pending_bank_checks_inner(
    pool: &SqlitePool,
    user_id: i64,
    as_of_date: &str,
) -> Result<Vec<BankCheckDepositRow>, AppError> {
    sqlx::query_as::<_, BankCheckDepositRow>(
        "SELECT
            pbc.id,
            pbc.account_owner,
            pbc.bank_name,
            pbc.check_number,
            pbc.amount,
            pbc.status,
            pbc.bank_deposit_id,
            pbc.payment_id,
            SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) AS paid_at,
            pat.first_name AS patient_first_name,
            pat.last_name AS patient_last_name
         FROM payment_bank_checks pbc
         INNER JOIN payments pay ON pay.id = pbc.payment_id
         LEFT JOIN medical_treatments mt ON mt.id = pay.medical_treatment_id
         LEFT JOIN patients pat ON pat.id = mt.patient_id
         WHERE pbc.user_id = ?
           AND COALESCE(pbc.status, 0) = 0
           AND pbc.bank_deposit_id IS NULL
           AND SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) <= ?
         ORDER BY SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) ASC, pbc.id ASC",
    )
    .bind(user_id)
    .bind(as_of_date)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list pending bank checks: {e}")))
}

pub async fn list_bank_account_options_inner(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<BankAccount>, AppError> {
    sqlx::query_as::<_, BankAccount>(
        "SELECT id, bank_name, bank_check_deposit_number, account_number, user_id, created_at, updated_at
         FROM bank_accounts
         WHERE user_id = ?
         ORDER BY LOWER(COALESCE(bank_name, '')) ASC, id ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list bank account options: {e}")))
}

pub async fn create_bank_deposit_inner(
    pool: &SqlitePool,
    user_id: i64,
    office: Office,
    input: BankDepositInput,
) -> Result<BankDepositDetail, AppError> {
    let mut v = ValidationBuilder::new();

    let deposit_type = match input.deposit_type {
        Some(1) => 1,
        Some(dt) if VALID_DEPOSIT_TYPES.contains(&dt) => {
            v.push(
                "deposit_type",
                "invalid_deposit_type",
                "Only check deposits (type 1) are supported.",
            );
            0
        }
        Some(_) => {
            v.push(
                "deposit_type",
                "invalid_deposit_type",
                "deposit_type must be 1 (Check).",
            );
            0
        }
        None => {
            v.push("deposit_type", "required", "deposit_type is required.");
            0
        }
    };

    let bank_account_id = match input.bank_account_id {
        Some(id) if id > 0 => id,
        _ => {
            v.push(
                "bank_account_id",
                "required",
                "bank_account_id is required.",
            );
            0
        }
    };

    let mut seen = HashSet::new();
    let check_ids: Vec<i64> = input
        .payment_bank_check_ids
        .into_iter()
        .filter(|&id| id > 0 && seen.insert(id))
        .collect();

    if check_ids.is_empty() {
        v.push(
            "payment_bank_check_ids",
            "no_checks_selected",
            "Please select at least one pending check.",
        );
    }

    v.finish()?;

    let now = now_rails_timestamp();
    let deposit_date = today_date_str();

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::message(format!("Failed to begin bank deposit transaction: {e}")))?;

    let bank_account = sqlx::query_as::<_, BankAccount>(
        "SELECT id, bank_name, bank_check_deposit_number, account_number, user_id, created_at, updated_at
         FROM bank_accounts
         WHERE id = ? AND user_id = ?",
    )
    .bind(bank_account_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to load bank account: {e}")))?;

    let Some(bank_account) = bank_account else {
        let mut v = ValidationBuilder::new();
        v.push(
            "bank_account_id",
            "invalid_bank_account",
            "Selected bank account is invalid.",
        );
        v.finish()?;
        unreachable!();
    };

    let next_number = bank_account.bank_check_deposit_number.unwrap_or(0) + 1;

    #[derive(sqlx::FromRow)]
    struct CheckTarget {
        amount: Option<RailsDecimal>,
        status: Option<i64>,
        bank_deposit_id: Option<i64>,
        payment_id: Option<i64>,
    }

    let mut total_cents: i64 = 0;
    let mut payment_ids: Vec<i64> = Vec::with_capacity(check_ids.len());

    for &check_id in &check_ids {
        let check = sqlx::query_as::<_, CheckTarget>(
            "SELECT amount, status, bank_deposit_id, payment_id
             FROM payment_bank_checks
             WHERE id = ? AND user_id = ?",
        )
        .bind(check_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to load check {check_id}: {e}")))?;

        let Some(check) = check else {
            let mut v = ValidationBuilder::new();
            v.push(
                "payment_bank_check_ids",
                "invalid_bank_check",
                format!("Bank check {check_id} does not exist."),
            );
            v.finish()?;
            unreachable!();
        };

        if check.status.unwrap_or(0) != 0 || check.bank_deposit_id.is_some() {
            let mut v = ValidationBuilder::new();
            v.push(
                "payment_bank_check_ids",
                "invalid_bank_check",
                format!("Bank check {check_id} is already cashed or deposited."),
            );
            v.finish()?;
        }

        let Some(pid) = check.payment_id else {
            return Err(AppError::message(format!(
                "Bank check {check_id} has no linked payment."
            )));
        };

        let payment_exists: Option<i64> =
            sqlx::query_scalar("SELECT id FROM payments WHERE id = ? AND user_id = ?")
                .bind(pid)
                .bind(user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| AppError::message(format!("Failed to verify payment {pid}: {e}")))?;

        if payment_exists.is_none() {
            return Err(AppError::message(format!(
                "Linked payment {pid} for bank check {check_id} was not found."
            )));
        }

        total_cents += check.amount.map(|a| to_cents(a.0)).unwrap_or(0);
        payment_ids.push(pid);
    }

    let total_amount = from_cents(total_cents);

    let deposit_res = sqlx::query(
        "INSERT INTO bank_deposits (deposit_type, number, amount, user_id, deposit_date, bank_account_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(deposit_type)
    .bind(next_number)
    .bind(total_amount)
    .bind(user_id)
    .bind(&deposit_date)
    .bind(bank_account.id)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to create bank deposit: {e}")))?;

    let deposit_id = deposit_res.last_insert_rowid();

    for (&check_id, &pid) in check_ids.iter().zip(payment_ids.iter()) {
        sqlx::query(
            "UPDATE payment_bank_checks SET status = 1, bank_deposit_id = ?, updated_at = ? WHERE id = ? AND user_id = ?",
        )
        .bind(deposit_id)
        .bind(&now)
        .bind(check_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to update bank check {check_id}: {e}")))?;

        sqlx::query(
            "UPDATE payments SET bank_deposit_id = ?, updated_at = ? WHERE id = ? AND user_id = ?",
        )
        .bind(deposit_id)
        .bind(&now)
        .bind(pid)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to update payment {pid}: {e}")))?;
    }

    sqlx::query(
        "UPDATE bank_accounts SET bank_check_deposit_number = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(next_number)
    .bind(&now)
    .bind(bank_account.id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to increment bank deposit number: {e}")))?;

    tx.commit().await.map_err(|e| {
        AppError::message(format!("Failed to commit bank deposit transaction: {e}"))
    })?;

    get_bank_deposit_inner(pool, user_id, office, deposit_id).await
}

pub async fn delete_bank_deposit_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let _existing = get_bank_deposit_view_inner(pool, user_id, id).await?;
    let now = now_rails_timestamp();

    let mut tx = pool.begin().await.map_err(|e| {
        AppError::message(format!(
            "Failed to begin bank deposit delete transaction: {e}"
        ))
    })?;

    sqlx::query(
        "UPDATE payment_bank_checks SET status = 0, bank_deposit_id = NULL, updated_at = ? WHERE bank_deposit_id = ? AND user_id = ?",
    )
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to reset bank checks: {e}")))?;

    sqlx::query(
        "UPDATE payments SET bank_deposit_id = NULL, updated_at = ? WHERE bank_deposit_id = ? AND user_id = ?",
    )
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to reset payments: {e}")))?;

    sqlx::query("DELETE FROM bank_deposits WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete bank deposit {id}: {e}")))?;

    tx.commit()
        .await
        .map_err(|e| AppError::message(format!("Failed to commit bank deposit delete: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::bank_accounts::{
        create_bank_account_inner, get_bank_account_inner, BankAccountInput,
    };
    use crate::commands::patients::{create_patient_inner, PatientInput};
    use crate::commands::payments::{
        create_payment_inner, delete_payment_inner, get_payment_inner, PaymentBankCheckInput,
        PaymentInput,
    };
    use crate::commands::treatment_types::{create_treatment_type_inner, TreatmentTypeInput};
    use crate::commands::treatments::{create_treatment_inner, MedicalTreatmentInput};
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_deposit_fixture() -> (SqlitePool, i64, Office, i64, Vec<i64>) {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, office) = resolve_active_user(&pool, None).await.unwrap();

        let patient = create_patient_inner(
            &pool,
            user.id,
            PatientInput {
                first_name: Some("Jean".to_string()),
                last_name: "Moreau".to_string(),
                birthdate: None,
                profession: None,
                phone: None,
                mobile_phone: None,
                email: None,
                address: None,
                postal_code: None,
                city: "Paris".to_string(),
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
                name: "Consultation".to_string(),
                price: Some(50.0),
            },
        )
        .await
        .unwrap();

        let bank_account = create_bank_account_inner(
            &pool,
            user.id,
            BankAccountInput {
                bank_name: "Societe Generale".to_string(),
                account_number: Some(123456789),
                bank_check_deposit_number: Some(10),
            },
        )
        .await
        .unwrap();

        let mut check_ids = Vec::new();
        for (date, amount, num) in [
            ("2026-03-05", 20.15, "C-2"),
            ("2026-03-01", 29.85, "C-1"),
            ("2099-12-31", 40.00, "C-FUTURE"),
        ] {
            let t = create_treatment_inner(
                &pool,
                user.id,
                MedicalTreatmentInput {
                    patient_id: Some(patient.id),
                    date: date.to_string(),
                    location_id: Some(1),
                    medical_treatment_type_id: Some(tt.id),
                    price: Some(50.0),
                    note: None,
                },
            )
            .await
            .unwrap();
            let pay = create_payment_inner(
                &pool,
                user.id,
                PaymentInput {
                    medical_treatment_id: Some(t.id),
                    paid_at: date.to_string(),
                    payment_type: Some(1),
                    amount: Some(amount),
                    payment_bank_check: Some(PaymentBankCheckInput {
                        account_owner: "Moreau".to_string(),
                        bank_name: "LCL".to_string(),
                        check_number: num.to_string(),
                    }),
                },
            )
            .await
            .unwrap();
            check_ids.push(pay.payment_bank_check.unwrap().id);
        }

        (pool, user.id, office, bank_account.id, check_ids)
    }

    #[tokio::test]
    async fn atomic_bank_deposit_create_show_and_destroy() {
        let (pool, user_id, office, ba_id, check_ids) = setup_deposit_fixture().await;

        // Pending checks as of 2026-04-01 excludes future-dated check and sorts by paid_at ASC
        let pending = list_pending_bank_checks_inner(&pool, user_id, "2026-04-01")
            .await
            .unwrap();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0].id, check_ids[1]); // 2026-03-01 first
        assert_eq!(pending[1].id, check_ids[0]); // 2026-03-05 second

        // Reject invalid check id without mutating bank_account counter or checks
        let invalid_err = create_bank_deposit_inner(
            &pool,
            user_id,
            office.clone(),
            BankDepositInput {
                deposit_type: Some(1),
                bank_account_id: Some(ba_id),
                payment_bank_check_ids: vec![check_ids[0], 999999],
            },
        )
        .await
        .unwrap_err();
        assert!(invalid_err
            .validation_errors()
            .unwrap()
            .iter()
            .any(|f| f.code == "invalid_bank_check"));
        assert_eq!(
            get_bank_account_inner(&pool, user_id, ba_id)
                .await
                .unwrap()
                .bank_check_deposit_number,
            Some(10)
        );

        // Create deposit atomically with both pending checks
        let detail = create_bank_deposit_inner(
            &pool,
            user_id,
            office.clone(),
            BankDepositInput {
                deposit_type: Some(1),
                bank_account_id: Some(ba_id),
                payment_bank_check_ids: vec![check_ids[0], check_ids[1]],
            },
        )
        .await
        .unwrap();

        assert_eq!(detail.deposit.number, Some(11));
        assert_eq!(detail.deposit.amount.unwrap().0, 50.0);
        assert_eq!(detail.deposit.checks_count, 2);
        // Show checks ordered by payments.paid_at ASC
        assert_eq!(detail.checks[0].check_number.as_deref(), Some("C-1"));
        assert_eq!(detail.checks[1].check_number.as_deref(), Some("C-2"));
        assert_eq!(detail.checks[0].status, Some(1));

        // Bank account counter incremented to 11
        assert_eq!(
            get_bank_account_inner(&pool, user_id, ba_id)
                .await
                .unwrap()
                .bank_check_deposit_number,
            Some(11)
        );

        // Pending list now has 0 checks as of 2026-04-01
        assert!(list_pending_bank_checks_inner(&pool, user_id, "2026-04-01")
            .await
            .unwrap()
            .is_empty());

        // Linked payment has bank_deposit_id set and cannot be deleted
        let linked_pid = detail.checks[0].payment_id.unwrap();
        assert_eq!(
            get_payment_inner(&pool, user_id, linked_pid)
                .await
                .unwrap()
                .bank_deposit_id,
            Some(detail.deposit.id)
        );
        assert!(delete_payment_inner(&pool, user_id, linked_pid)
            .await
            .is_err());

        // Double-depositing already cashed check is rejected
        let dup_err = create_bank_deposit_inner(
            &pool,
            user_id,
            office.clone(),
            BankDepositInput {
                deposit_type: Some(1),
                bank_account_id: Some(ba_id),
                payment_bank_check_ids: vec![check_ids[0]],
            },
        )
        .await
        .unwrap_err();
        assert!(dup_err
            .validation_errors()
            .unwrap()
            .iter()
            .any(|f| f.code == "invalid_bank_check"));

        // Filter index by deposit_type and number
        let filtered = list_bank_deposits_inner(
            &pool,
            user_id,
            BankDepositFilter {
                deposit_type: Some(1),
                number: Some(11),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(filtered.filtered_total, 1);

        // Atomic delete resets checks (status = 0, bank_deposit_id = NULL) & payments (bank_deposit_id = NULL)
        delete_bank_deposit_inner(&pool, user_id, detail.deposit.id)
            .await
            .unwrap();
        assert!(
            get_bank_deposit_view_inner(&pool, user_id, detail.deposit.id)
                .await
                .is_err()
        );
        assert_eq!(
            get_payment_inner(&pool, user_id, linked_pid)
                .await
                .unwrap()
                .bank_deposit_id,
            None
        );
        let pending_after_delete = list_pending_bank_checks_inner(&pool, user_id, "2026-04-01")
            .await
            .unwrap();
        assert_eq!(pending_after_delete.len(), 2);
    }
}
