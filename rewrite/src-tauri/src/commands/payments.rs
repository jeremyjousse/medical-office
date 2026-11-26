use crate::db::AppState;
use crate::error::{AppError, ValidationBuilder};
use crate::models::{
    from_cents, normalize_listing_pagination, normalize_rails_date, now_rails_timestamp, to_cents,
    PaginatedResult, PaymentBankCheck, RailsDecimal,
};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqliteConnection, SqlitePool};
use std::collections::BTreeMap;
use tauri::State;

pub const DEFAULT_PAYMENTS_PER_PAGE: i64 = 10;
pub const VALID_PAYMENT_TYPES: &[i64] = &[1, 2, 3, 4];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentView {
    pub id: i64,
    pub medical_treatment_id: Option<i64>,
    pub paid_at: Option<String>,
    pub payment_type: Option<i64>,
    pub amount: Option<RailsDecimal>,
    pub user_id: Option<i64>,
    pub bank_deposit_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub patient_id: Option<i64>,
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub treatment_type_name: Option<String>,
    pub treatment_date: Option<String>,
    pub treatment_price: Option<RailsDecimal>,
    pub payment_bank_check: Option<PaymentBankCheck>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PaymentJoinedRow {
    id: i64,
    medical_treatment_id: Option<i64>,
    paid_at: Option<String>,
    payment_type: Option<i64>,
    amount: Option<RailsDecimal>,
    user_id: Option<i64>,
    bank_deposit_id: Option<i64>,
    created_at: Option<String>,
    updated_at: Option<String>,
    patient_id: Option<i64>,
    patient_first_name: Option<String>,
    patient_last_name: Option<String>,
    treatment_type_name: Option<String>,
    treatment_date: Option<String>,
    treatment_price: Option<RailsDecimal>,
    check_id: Option<i64>,
    check_account_owner: Option<String>,
    check_bank_name: Option<String>,
    check_number: Option<String>,
    check_amount: Option<RailsDecimal>,
    check_status: Option<i64>,
    check_bank_deposit_id: Option<i64>,
    check_payment_id: Option<i64>,
    check_user_id: Option<i64>,
    check_created_at: Option<String>,
    check_updated_at: Option<String>,
}

impl From<PaymentJoinedRow> for PaymentView {
    fn from(r: PaymentJoinedRow) -> Self {
        let payment_bank_check = r.check_id.map(|cid| PaymentBankCheck {
            id: cid,
            account_owner: r.check_account_owner,
            bank_name: r.check_bank_name,
            check_number: r.check_number,
            amount: r.check_amount,
            status: r.check_status,
            bank_deposit_id: r.check_bank_deposit_id,
            payment_id: r.check_payment_id,
            user_id: r.check_user_id,
            created_at: r.check_created_at,
            updated_at: r.check_updated_at,
        });

        Self {
            id: r.id,
            medical_treatment_id: r.medical_treatment_id,
            paid_at: r.paid_at,
            payment_type: r.payment_type,
            amount: r.amount,
            user_id: r.user_id,
            bank_deposit_id: r.bank_deposit_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
            patient_id: r.patient_id,
            patient_first_name: r.patient_first_name,
            patient_last_name: r.patient_last_name,
            treatment_type_name: r.treatment_type_name,
            treatment_date: r.treatment_date,
            treatment_price: r.treatment_price,
            payment_bank_check,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaymentFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub paid_at: Option<String>,
    pub payment_type: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PaymentListingFilter {
    pub paid_at_gteq: Option<String>,
    pub paid_at_lteq: Option<String>,
    pub payment_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentTypeTotal {
    pub payment_type: i64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListingDayGroup {
    pub date: String,
    pub payments: Vec<PaymentView>,
    pub total_amount: f64,
    pub totals_by_type: Vec<PaymentTypeTotal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListingResult {
    pub days: Vec<PaymentListingDayGroup>,
    pub grand_total_amount: f64,
    pub grand_totals_by_type: Vec<PaymentTypeTotal>,
    pub filtered_total: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentBankCheckInput {
    pub account_owner: String,
    pub bank_name: String,
    pub check_number: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentInput {
    pub medical_treatment_id: Option<i64>,
    pub paid_at: String,
    pub payment_type: Option<i64>,
    pub amount: Option<f64>,
    pub payment_bank_check: Option<PaymentBankCheckInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MedicalTreatmentOption {
    pub id: i64,
    pub patient_id: Option<i64>,
    pub patient_first_name: Option<String>,
    pub patient_last_name: Option<String>,
    pub treatment_type_name: Option<String>,
    pub date: Option<String>,
    pub price: Option<RailsDecimal>,
    pub status: Option<i64>,
}

const PAYMENT_SELECT_SQL: &str = "
SELECT
    pay.id,
    pay.medical_treatment_id,
    pay.paid_at,
    pay.payment_type,
    pay.amount,
    pay.user_id,
    pay.bank_deposit_id,
    pay.created_at,
    pay.updated_at,
    mt.patient_id,
    p.first_name AS patient_first_name,
    p.last_name AS patient_last_name,
    mtt.name AS treatment_type_name,
    mt.date AS treatment_date,
    mt.price AS treatment_price,
    pbc.id AS check_id,
    pbc.account_owner AS check_account_owner,
    pbc.bank_name AS check_bank_name,
    pbc.check_number AS check_number,
    pbc.amount AS check_amount,
    pbc.status AS check_status,
    pbc.bank_deposit_id AS check_bank_deposit_id,
    pbc.payment_id AS check_payment_id,
    pbc.user_id AS check_user_id,
    pbc.created_at AS check_created_at,
    pbc.updated_at AS check_updated_at
FROM payments pay
LEFT JOIN medical_treatments mt ON mt.id = pay.medical_treatment_id
LEFT JOIN patients p ON p.id = mt.patient_id
LEFT JOIN medical_treatment_types mtt ON mtt.id = mt.medical_treatment_type_id
LEFT JOIN payment_bank_checks pbc ON pbc.payment_id = pay.id
WHERE pay.user_id = ";

#[tauri::command]
pub async fn list_payments(
    filter: PaymentFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<PaymentView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_payments_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn list_payment_listing(
    filter: PaymentListingFilter,
    state: State<'_, AppState>,
) -> Result<PaymentListingResult, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_payment_listing_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_payment(id: i64, state: State<'_, AppState>) -> Result<PaymentView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_payment_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_payment(
    input: PaymentInput,
    state: State<'_, AppState>,
) -> Result<PaymentView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_payment_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_payment(
    id: i64,
    input: PaymentInput,
    state: State<'_, AppState>,
) -> Result<PaymentView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_payment_inner(&pool, user.id, id, input).await
}

#[tauri::command]
pub async fn delete_payment(id: i64, state: State<'_, AppState>) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    delete_payment_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn list_treatment_options_for_payment(
    state: State<'_, AppState>,
) -> Result<Vec<MedicalTreatmentOption>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_treatment_options_for_payment_inner(&pool, user.id).await
}

pub async fn list_payments_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: PaymentFilter,
) -> Result<PaginatedResult<PaymentView>, AppError> {
    let (page, per_page, offset) =
        normalize_listing_pagination(filter.page, filter.per_page, DEFAULT_PAYMENTS_PER_PAGE);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM payments pay WHERE pay.user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count payments: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM payments pay WHERE pay.user_id = ");
    count_qb.push_bind(user_id);
    apply_payment_filters(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered payments: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(PAYMENT_SELECT_SQL);
    list_qb.push_bind(user_id);
    apply_payment_filters(&mut list_qb, &filter);
    list_qb.push(" ORDER BY SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) DESC, pay.id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let rows: Vec<PaymentJoinedRow> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list payments: {e}")))?;

    Ok(PaginatedResult {
        items: rows.into_iter().map(PaymentView::from).collect(),
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_payment_filters<'a>(qb: &mut QueryBuilder<'a, Sqlite>, filter: &'a PaymentFilter) {
    if let Some(ref paid_at) = filter.paid_at {
        if let Some(norm) = normalize_rails_date(paid_at) {
            qb.push(" AND SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) = ");
            qb.push_bind(norm);
        }
    }
    if let Some(payment_type) = filter.payment_type {
        qb.push(" AND pay.payment_type = ");
        qb.push_bind(payment_type);
    }
}

pub async fn list_payment_listing_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: PaymentListingFilter,
) -> Result<PaymentListingResult, AppError> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM payments pay WHERE pay.user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count payments: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(PAYMENT_SELECT_SQL);
    list_qb.push_bind(user_id);
    if let Some(ref gteq) = filter.paid_at_gteq {
        if let Some(norm) = normalize_rails_date(gteq) {
            list_qb.push(" AND SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) >= ");
            list_qb.push_bind(norm);
        }
    }
    if let Some(ref lteq) = filter.paid_at_lteq {
        if let Some(norm) = normalize_rails_date(lteq) {
            list_qb.push(" AND SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) <= ");
            list_qb.push_bind(norm);
        }
    }
    if let Some(payment_type) = filter.payment_type {
        list_qb.push(" AND pay.payment_type = ");
        list_qb.push_bind(payment_type);
    }
    list_qb.push(
        " ORDER BY SUBSTR(COALESCE(pay.paid_at, ''), 1, 10) ASC, COALESCE(pay.payment_type, 0) ASC, pay.id ASC",
    );

    let rows: Vec<PaymentJoinedRow> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to load payment listing: {e}")))?;

    let items: Vec<PaymentView> = rows.into_iter().map(PaymentView::from).collect();
    Ok(group_payment_listing(items, total))
}

fn group_payment_listing(items: Vec<PaymentView>, total: i64) -> PaymentListingResult {
    let filtered_total = items.len() as i64;
    let mut days: Vec<PaymentListingDayGroup> = Vec::new();
    let mut grand_total_cents: i64 = 0;
    let mut grand_by_type_cents: BTreeMap<i64, i64> = BTreeMap::new();

    let mut current_date: Option<String> = None;
    let mut current_payments: Vec<PaymentView> = Vec::new();
    let mut day_total_cents: i64 = 0;
    let mut day_by_type_cents: BTreeMap<i64, i64> = BTreeMap::new();

    let flush_day = |days: &mut Vec<PaymentListingDayGroup>,
                     date: String,
                     payments: Vec<PaymentView>,
                     day_total_cents: i64,
                     day_by_type_cents: BTreeMap<i64, i64>| {
        let totals_by_type = day_by_type_cents
            .into_iter()
            .map(|(payment_type, cents)| PaymentTypeTotal {
                payment_type,
                amount: from_cents(cents),
            })
            .collect();
        days.push(PaymentListingDayGroup {
            date,
            payments,
            total_amount: from_cents(day_total_cents),
            totals_by_type,
        });
    };

    for item in items {
        let day_key = item
            .paid_at
            .as_deref()
            .and_then(normalize_rails_date)
            .unwrap_or_else(|| item.paid_at.clone().unwrap_or_default());

        if current_date.as_ref() != Some(&day_key) {
            if let Some(prev_date) = current_date.take() {
                flush_day(
                    &mut days,
                    prev_date,
                    std::mem::take(&mut current_payments),
                    day_total_cents,
                    std::mem::take(&mut day_by_type_cents),
                );
                day_total_cents = 0;
            }
            current_date = Some(day_key);
        }

        let amount_cents = item.amount.map(|a| to_cents(a.0)).unwrap_or(0);
        let ptype = item.payment_type.unwrap_or(0);

        day_total_cents += amount_cents;
        *day_by_type_cents.entry(ptype).or_insert(0) += amount_cents;

        grand_total_cents += amount_cents;
        *grand_by_type_cents.entry(ptype).or_insert(0) += amount_cents;

        current_payments.push(item);
    }

    if let Some(last_date) = current_date {
        flush_day(
            &mut days,
            last_date,
            current_payments,
            day_total_cents,
            day_by_type_cents,
        );
    }

    let grand_totals_by_type = grand_by_type_cents
        .into_iter()
        .map(|(payment_type, cents)| PaymentTypeTotal {
            payment_type,
            amount: from_cents(cents),
        })
        .collect();

    PaymentListingResult {
        days,
        grand_total_amount: from_cents(grand_total_cents),
        grand_totals_by_type,
        filtered_total,
        total,
    }
}

pub async fn get_payment_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<PaymentView, AppError> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(PAYMENT_SELECT_SQL);
    qb.push_bind(user_id);
    qb.push(" AND pay.id = ");
    qb.push_bind(id);

    qb.build_query_as::<PaymentJoinedRow>()
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to load payment {id}: {e}")))?
        .map(PaymentView::from)
        .ok_or_else(|| AppError::not_found(format!("Payment {id} not found.")))
}

pub async fn list_treatment_options_for_payment_inner(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<MedicalTreatmentOption>, AppError> {
    sqlx::query_as::<_, MedicalTreatmentOption>(
        "SELECT
            mt.id,
            mt.patient_id,
            p.first_name AS patient_first_name,
            p.last_name AS patient_last_name,
            mtt.name AS treatment_type_name,
            mt.date,
            mt.price,
            mt.status
         FROM medical_treatments mt
         LEFT JOIN patients p ON p.id = mt.patient_id
         LEFT JOIN medical_treatment_types mtt ON mtt.id = mt.medical_treatment_type_id
         WHERE mt.user_id = ?
         ORDER BY SUBSTR(COALESCE(mt.date, ''), 1, 10) DESC, mt.id DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list treatments for payment: {e}")))
}

struct ValidatedPayment {
    medical_treatment_id: i64,
    paid_at: String,
    payment_type: i64,
    amount: f64,
    amount_cents: i64,
    check: Option<ValidatedBankCheck>,
}

struct ValidatedBankCheck {
    account_owner: String,
    bank_name: String,
    check_number: String,
}

fn validate_payment_fields(input: PaymentInput) -> Result<ValidatedPayment, AppError> {
    let mut v = ValidationBuilder::new();

    let medical_treatment_id = match input.medical_treatment_id {
        Some(tid) if tid > 0 => tid,
        _ => {
            v.push(
                "medical_treatment_id",
                "required",
                "medical_treatment_id is required.",
            );
            0
        }
    };

    let paid_at = if input.paid_at.trim().is_empty() {
        v.push("paid_at", "required", "paid_at is required.");
        String::new()
    } else if let Some(norm) = normalize_rails_date(&input.paid_at) {
        norm
    } else {
        v.push(
            "paid_at",
            "invalid_date",
            "paid_at must be a valid YYYY-MM-DD date.",
        );
        String::new()
    };

    let payment_type = match input.payment_type {
        Some(pt) if VALID_PAYMENT_TYPES.contains(&pt) => pt,
        Some(_) => {
            v.push(
                "payment_type",
                "invalid_payment_type",
                "payment_type must be 1 (Check), 2 (Credit card), 3 (Cash), or 4 (Bank account transfer).",
            );
            0
        }
        None => {
            v.push("payment_type", "required", "payment_type is required.");
            0
        }
    };

    let (amount, amount_cents) = match input.amount {
        Some(a) if a.is_finite() && to_cents(a) > 0 => {
            let cents = to_cents(a);
            (from_cents(cents), cents)
        }
        Some(_) => {
            v.push(
                "amount",
                "invalid_amount",
                "amount must be a valid positive number.",
            );
            (0.0, 0)
        }
        None => {
            v.push("amount", "required", "amount is required.");
            (0.0, 0)
        }
    };

    let check = if payment_type == 1 {
        match input.payment_bank_check {
            Some(c) => {
                let account_owner = v.required_len("account_owner", &c.account_owner, 1, 255);
                let bank_name = v.required_len("bank_name", &c.bank_name, 1, 255);
                let check_number = v.required_len("check_number", &c.check_number, 1, 255);
                Some(ValidatedBankCheck {
                    account_owner,
                    bank_name,
                    check_number,
                })
            }
            None => {
                v.push(
                    "account_owner",
                    "required",
                    "Check details are required for check payments.",
                );
                None
            }
        }
    } else {
        None
    };

    v.finish()?;

    Ok(ValidatedPayment {
        medical_treatment_id,
        paid_at,
        payment_type,
        amount,
        amount_cents,
        check,
    })
}

async fn load_treatment_price_cents(
    conn: &mut SqliteConnection,
    user_id: i64,
    treatment_id: i64,
) -> Result<Option<i64>, AppError> {
    let price: Option<Option<RailsDecimal>> =
        sqlx::query_scalar("SELECT price FROM medical_treatments WHERE id = ? AND user_id = ?")
            .bind(treatment_id)
            .bind(user_id)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| {
                AppError::message(format!("Failed to load treatment {treatment_id}: {e}"))
            })?;

    Ok(price.map(|p| p.map(|d| to_cents(d.0)).unwrap_or(0)))
}

async fn sum_treatment_payments_cents(
    conn: &mut SqliteConnection,
    treatment_id: i64,
    exclude_payment_id: Option<i64>,
) -> Result<i64, AppError> {
    let amounts: Vec<RailsDecimal> = if let Some(pid) = exclude_payment_id {
        sqlx::query_scalar("SELECT amount FROM payments WHERE medical_treatment_id = ? AND id != ?")
            .bind(treatment_id)
            .bind(pid)
            .fetch_all(&mut *conn)
            .await
    } else {
        sqlx::query_scalar("SELECT amount FROM payments WHERE medical_treatment_id = ?")
            .bind(treatment_id)
            .fetch_all(&mut *conn)
            .await
    }
    .map_err(|e| AppError::message(format!("Failed to sum treatment payments: {e}")))?;

    Ok(amounts.iter().map(|a| to_cents(a.0)).sum())
}

async fn sync_treatment_status_tx(
    conn: &mut SqliteConnection,
    user_id: i64,
    treatment_id: i64,
    now: &str,
) -> Result<(), AppError> {
    let Some(price_cents) = load_treatment_price_cents(conn, user_id, treatment_id).await? else {
        return Ok(());
    };
    let total_cents = sum_treatment_payments_cents(conn, treatment_id, None).await?;
    let status = if total_cents == price_cents { 1 } else { 0 };

    sqlx::query(
        "UPDATE medical_treatments SET status = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(status)
    .bind(now)
    .bind(treatment_id)
    .bind(user_id)
    .execute(&mut *conn)
    .await
    .map_err(|e| AppError::message(format!("Failed to update treatment status: {e}")))?;

    Ok(())
}

pub async fn create_payment_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: PaymentInput,
) -> Result<PaymentView, AppError> {
    let data = validate_payment_fields(input)?;
    let now = now_rails_timestamp();

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::message(format!("Failed to begin payment transaction: {e}")))?;

    let Some(price_cents) =
        load_treatment_price_cents(&mut tx, user_id, data.medical_treatment_id).await?
    else {
        let mut v = ValidationBuilder::new();
        v.push(
            "medical_treatment_id",
            "invalid_treatment",
            "Selected medical treatment is invalid.",
        );
        v.finish()?;
        unreachable!();
    };

    let existing_cents =
        sum_treatment_payments_cents(&mut tx, data.medical_treatment_id, None).await?;
    if existing_cents + data.amount_cents > price_cents {
        let mut v = ValidationBuilder::new();
        v.push(
            "amount",
            "payments_exceed_price",
            "Total payments cannot exceed treatment price.",
        );
        v.finish()?;
    }

    let res = sqlx::query(
        "INSERT INTO payments (medical_treatment_id, paid_at, payment_type, amount, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(data.medical_treatment_id)
    .bind(&data.paid_at)
    .bind(data.payment_type)
    .bind(data.amount)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to create payment: {e}")))?;

    let payment_id = res.last_insert_rowid();

    if let Some(check) = data.check {
        sqlx::query(
            "INSERT INTO payment_bank_checks (account_owner, bank_name, check_number, amount, status, payment_id, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, 0, ?, ?, ?, ?)",
        )
        .bind(&check.account_owner)
        .bind(&check.bank_name)
        .bind(&check.check_number)
        .bind(data.amount)
        .bind(payment_id)
        .bind(user_id)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to create bank check: {e}")))?;
    }

    sync_treatment_status_tx(&mut tx, user_id, data.medical_treatment_id, &now).await?;

    tx.commit()
        .await
        .map_err(|e| AppError::message(format!("Failed to commit payment transaction: {e}")))?;

    get_payment_inner(pool, user_id, payment_id).await
}

pub async fn update_payment_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: PaymentInput,
) -> Result<PaymentView, AppError> {
    let existing = get_payment_inner(pool, user_id, id).await?;
    let data = validate_payment_fields(input)?;
    let now = now_rails_timestamp();

    if existing.bank_deposit_id.is_some() {
        let old_amount_cents = existing.amount.map(|a| to_cents(a.0)).unwrap_or(0);
        if existing.payment_type != Some(data.payment_type)
            || old_amount_cents != data.amount_cents
            || existing.medical_treatment_id != Some(data.medical_treatment_id)
        {
            return Err(AppError::blocked(
                "payment_in_bank_deposit",
                "Cannot modify payment type, amount, or treatment because payment is included in a bank deposit.",
            ));
        }
    }

    let mut tx = pool.begin().await.map_err(|e| {
        AppError::message(format!("Failed to begin payment update transaction: {e}"))
    })?;

    let Some(price_cents) =
        load_treatment_price_cents(&mut tx, user_id, data.medical_treatment_id).await?
    else {
        let mut v = ValidationBuilder::new();
        v.push(
            "medical_treatment_id",
            "invalid_treatment",
            "Selected medical treatment is invalid.",
        );
        v.finish()?;
        unreachable!();
    };

    let other_cents =
        sum_treatment_payments_cents(&mut tx, data.medical_treatment_id, Some(id)).await?;
    if other_cents + data.amount_cents > price_cents {
        let mut v = ValidationBuilder::new();
        v.push(
            "amount",
            "payments_exceed_price",
            "Total payments cannot exceed treatment price.",
        );
        v.finish()?;
    }

    sqlx::query(
        "UPDATE payments SET medical_treatment_id = ?, paid_at = ?, payment_type = ?, amount = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(data.medical_treatment_id)
    .bind(&data.paid_at)
    .bind(data.payment_type)
    .bind(data.amount)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to update payment {id}: {e}")))?;

    if let Some(check) = data.check {
        let existing_check_id: Option<i64> =
            sqlx::query_scalar("SELECT id FROM payment_bank_checks WHERE payment_id = ?")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| AppError::message(format!("Failed to check bank check: {e}")))?;

        if let Some(check_id) = existing_check_id {
            sqlx::query(
                "UPDATE payment_bank_checks SET account_owner = ?, bank_name = ?, check_number = ?, amount = ?, updated_at = ? WHERE id = ?",
            )
            .bind(&check.account_owner)
            .bind(&check.bank_name)
            .bind(&check.check_number)
            .bind(data.amount)
            .bind(&now)
            .bind(check_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::message(format!("Failed to update bank check: {e}")))?;
        } else {
            sqlx::query(
                "INSERT INTO payment_bank_checks (account_owner, bank_name, check_number, amount, status, payment_id, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, 0, ?, ?, ?, ?)",
            )
            .bind(&check.account_owner)
            .bind(&check.bank_name)
            .bind(&check.check_number)
            .bind(data.amount)
            .bind(id)
            .bind(user_id)
            .bind(&now)
            .bind(&now)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::message(format!("Failed to create bank check: {e}")))?;
        }
    } else {
        sqlx::query("DELETE FROM payment_bank_checks WHERE payment_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::message(format!("Failed to delete bank check: {e}")))?;
    }

    sync_treatment_status_tx(&mut tx, user_id, data.medical_treatment_id, &now).await?;
    if let Some(old_tid) = existing.medical_treatment_id {
        if old_tid != data.medical_treatment_id {
            sync_treatment_status_tx(&mut tx, user_id, old_tid, &now).await?;
        }
    }

    tx.commit()
        .await
        .map_err(|e| AppError::message(format!("Failed to commit payment update: {e}")))?;

    get_payment_inner(pool, user_id, id).await
}

pub async fn delete_payment_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let existing = get_payment_inner(pool, user_id, id).await?;

    if existing.bank_deposit_id.is_some() {
        return Err(AppError::blocked(
            "payment_in_bank_deposit",
            "Cannot delete payment because it is included in a bank deposit.",
        ));
    }

    let now = now_rails_timestamp();
    let mut tx = pool.begin().await.map_err(|e| {
        AppError::message(format!("Failed to begin payment delete transaction: {e}"))
    })?;

    sqlx::query("DELETE FROM payment_bank_checks WHERE payment_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete bank check: {e}")))?;

    sqlx::query("DELETE FROM payments WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete payment {id}: {e}")))?;

    if let Some(tid) = existing.medical_treatment_id {
        sync_treatment_status_tx(&mut tx, user_id, tid, &now).await?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::message(format!("Failed to commit payment delete: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::patients::{create_patient_inner, PatientInput};
    use crate::commands::treatment_types::{create_treatment_type_inner, TreatmentTypeInput};
    use crate::commands::treatments::{
        create_treatment_inner, get_treatment_view_inner, MedicalTreatmentInput,
    };
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
                first_name: Some("Claire".to_string()),
                last_name: "Dupont".to_string(),
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
                name: "Bilan".to_string(),
                price: Some(60.0),
            },
        )
        .await
        .unwrap();

        let treatment = create_treatment_inner(
            &pool,
            user.id,
            MedicalTreatmentInput {
                patient_id: Some(patient.id),
                date: "2026-04-01".to_string(),
                location_id: Some(1),
                medical_treatment_type_id: Some(tt.id),
                price: Some(60.0),
                note: None,
            },
        )
        .await
        .unwrap();

        (pool, user.id, patient.id, treatment.id)
    }

    #[tokio::test]
    async fn validates_payment_totals_updates_status_and_handles_bank_checks() {
        let (pool, user_id, _patient_id, tid) = setup_fixture().await;

        // 1. Check payment requires check fields
        let err = create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-01".to_string(),
                payment_type: Some(1),
                amount: Some(25.0),
                payment_bank_check: Some(PaymentBankCheckInput {
                    account_owner: "".to_string(),
                    bank_name: "".to_string(),
                    check_number: "".to_string(),
                }),
            },
        )
        .await
        .unwrap_err();
        let fields = err.validation_errors().unwrap();
        assert!(fields.iter().any(|f| f.field == "account_owner"));
        assert!(fields.iter().any(|f| f.field == "bank_name"));
        assert!(fields.iter().any(|f| f.field == "check_number"));

        // 2. Partial check payment (25.00 of 60.00) -> check status 0, treatment status 0
        let p1 = create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-01".to_string(),
                payment_type: Some(1),
                amount: Some(25.0),
                payment_bank_check: Some(PaymentBankCheckInput {
                    account_owner: "Dupont".to_string(),
                    bank_name: "Credit Agricole".to_string(),
                    check_number: "CHK-001".to_string(),
                }),
            },
        )
        .await
        .unwrap();
        assert_eq!(p1.amount.unwrap().0, 25.0);
        let chk = p1.payment_bank_check.as_ref().unwrap();
        assert_eq!(chk.status, Some(0));
        assert_eq!(chk.amount.unwrap().0, 25.0);
        assert_eq!(
            get_treatment_view_inner(&pool, user_id, tid)
                .await
                .unwrap()
                .status,
            Some(0)
        );

        // 3. Exceeding treatment price (25 + 40 > 60) is rejected atomically
        let exceed_err = create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-02".to_string(),
                payment_type: Some(1),
                amount: Some(40.0),
                payment_bank_check: Some(PaymentBankCheckInput {
                    account_owner: "Dupont".to_string(),
                    bank_name: "BNP".to_string(),
                    check_number: "CHK-002".to_string(),
                }),
            },
        )
        .await
        .unwrap_err();
        assert!(exceed_err
            .validation_errors()
            .unwrap()
            .iter()
            .any(|f| f.field == "amount" && f.code == "payments_exceed_price"));
        // Verify no orphan check or payment created
        let chk_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM payment_bank_checks")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(chk_count, 1);

        // 4. Exact remaining payment (35.00 cash) -> treatment status becomes 1 (paid)
        let p2 = create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-02".to_string(),
                payment_type: Some(3),
                amount: Some(35.0),
                payment_bank_check: None,
            },
        )
        .await
        .unwrap();
        assert!(p2.payment_bank_check.is_none());
        assert_eq!(
            get_treatment_view_inner(&pool, user_id, tid)
                .await
                .unwrap()
                .status,
            Some(1)
        );

        // 5. Updating p2 amount down to 30.00 reverts treatment status to 0
        let p2_updated = update_payment_inner(
            &pool,
            user_id,
            p2.id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-02".to_string(),
                payment_type: Some(2),
                amount: Some(30.0),
                payment_bank_check: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(p2_updated.payment_type, Some(2));
        assert_eq!(
            get_treatment_view_inner(&pool, user_id, tid)
                .await
                .unwrap()
                .status,
            Some(0)
        );

        // Restore p2 to 35.00 (paid)
        update_payment_inner(
            &pool,
            user_id,
            p2.id,
            PaymentInput {
                medical_treatment_id: Some(tid),
                paid_at: "2026-04-02".to_string(),
                payment_type: Some(2),
                amount: Some(35.0),
                payment_bank_check: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(
            get_treatment_view_inner(&pool, user_id, tid)
                .await
                .unwrap()
                .status,
            Some(1)
        );

        // 6. safe_destroy deletes p2 and reverts treatment status to 0
        delete_payment_inner(&pool, user_id, p2.id).await.unwrap();
        assert_eq!(
            get_treatment_view_inner(&pool, user_id, tid)
                .await
                .unwrap()
                .status,
            Some(0)
        );

        // 7. safe_destroy is blocked when payment is linked to a bank deposit
        sqlx::query("UPDATE payments SET bank_deposit_id = 99 WHERE id = ?")
            .bind(p1.id)
            .execute(&pool)
            .await
            .unwrap();
        let blocked = delete_payment_inner(&pool, user_id, p1.id)
            .await
            .unwrap_err();
        assert!(matches!(
            blocked,
            AppError::Blocked { ref code, .. } if code == "payment_in_bank_deposit"
        ));
    }

    #[tokio::test]
    async fn computes_payment_listing_day_subtotals_and_grand_totals() {
        let (pool, user_id, patient_id, tid1) = setup_fixture().await;

        let tt2 = create_treatment_type_inner(
            &pool,
            user_id,
            TreatmentTypeInput {
                name: "Semelles".to_string(),
                price: Some(120.0),
            },
        )
        .await
        .unwrap();
        let tid2 = create_treatment_inner(
            &pool,
            user_id,
            MedicalTreatmentInput {
                patient_id: Some(patient_id),
                date: "2026-04-02".to_string(),
                location_id: Some(1),
                medical_treatment_type_id: Some(tt2.id),
                price: Some(120.0),
                note: None,
            },
        )
        .await
        .unwrap()
        .id;

        // Day 1: 2026-04-01 -> Check 20.10 + Cash 10.20 + Check 15.05 = 45.35
        create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid1),
                paid_at: "2026-04-01".to_string(),
                payment_type: Some(3),
                amount: Some(10.20),
                payment_bank_check: None,
            },
        )
        .await
        .unwrap();
        create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid1),
                paid_at: "2026-04-01".to_string(),
                payment_type: Some(1),
                amount: Some(20.10),
                payment_bank_check: Some(PaymentBankCheckInput {
                    account_owner: "Dupont".to_string(),
                    bank_name: "LCL".to_string(),
                    check_number: "101".to_string(),
                }),
            },
        )
        .await
        .unwrap();
        create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid1),
                paid_at: "2026-04-01".to_string(),
                payment_type: Some(1),
                amount: Some(15.05),
                payment_bank_check: Some(PaymentBankCheckInput {
                    account_owner: "Dupont".to_string(),
                    bank_name: "LCL".to_string(),
                    check_number: "102".to_string(),
                }),
            },
        )
        .await
        .unwrap();

        // Day 2: 2026-04-03 -> Credit card 50.00
        create_payment_inner(
            &pool,
            user_id,
            PaymentInput {
                medical_treatment_id: Some(tid2),
                paid_at: "2026-04-03".to_string(),
                payment_type: Some(2),
                amount: Some(50.00),
                payment_bank_check: None,
            },
        )
        .await
        .unwrap();

        let res = list_payment_listing_inner(&pool, user_id, PaymentListingFilter::default())
            .await
            .unwrap();
        assert_eq!(res.total, 4);
        assert_eq!(res.filtered_total, 4);
        assert_eq!(res.days.len(), 2);

        // Day 1 sorted by paid_at ASC, payment_type ASC
        assert_eq!(res.days[0].date, "2026-04-01");
        assert_eq!(res.days[0].payments.len(), 3);
        assert_eq!(res.days[0].payments[0].payment_type, Some(1));
        assert_eq!(res.days[0].payments[1].payment_type, Some(1));
        assert_eq!(res.days[0].payments[2].payment_type, Some(3));
        assert_eq!(res.days[0].total_amount, 45.35);
        assert_eq!(
            res.days[0].totals_by_type,
            vec![
                PaymentTypeTotal {
                    payment_type: 1,
                    amount: 35.15,
                },
                PaymentTypeTotal {
                    payment_type: 3,
                    amount: 10.20,
                },
            ]
        );

        assert_eq!(res.days[1].date, "2026-04-03");
        assert_eq!(res.days[1].total_amount, 50.00);

        assert_eq!(res.grand_total_amount, 95.35);
        assert_eq!(
            res.grand_totals_by_type,
            vec![
                PaymentTypeTotal {
                    payment_type: 1,
                    amount: 35.15,
                },
                PaymentTypeTotal {
                    payment_type: 2,
                    amount: 50.00,
                },
                PaymentTypeTotal {
                    payment_type: 3,
                    amount: 10.20,
                },
            ]
        );

        // Filter by date range and payment_type
        let filtered = list_payment_listing_inner(
            &pool,
            user_id,
            PaymentListingFilter {
                paid_at_gteq: Some("2026-04-01".to_string()),
                paid_at_lteq: Some("2026-04-01".to_string()),
                payment_type: Some(1),
            },
        )
        .await
        .unwrap();
        assert_eq!(filtered.total, 4);
        assert_eq!(filtered.filtered_total, 2);
        assert_eq!(filtered.grand_total_amount, 35.15);

        // Paginated payments index & filters (paid_at + payment_type)
        let page1 = list_payments_inner(
            &pool,
            user_id,
            PaymentFilter {
                page: Some(1),
                per_page: Some(2),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(page1.total, 4);
        assert_eq!(page1.filtered_total, 4);
        assert_eq!(page1.items.len(), 2);
        assert_eq!(page1.items[0].paid_at.as_deref(), Some("2026-04-03"));

        let by_filter = list_payments_inner(
            &pool,
            user_id,
            PaymentFilter {
                paid_at: Some("2026-04-01 09:00:00".to_string()),
                payment_type: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(by_filter.total, 4);
        assert_eq!(by_filter.filtered_total, 2);
        assert!(by_filter
            .items
            .iter()
            .all(|p| p.payment_bank_check.is_some()));
    }
}
