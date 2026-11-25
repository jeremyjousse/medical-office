use crate::db::AppState;
use crate::error::{escape_like_pattern, AppError, ValidationBuilder};
use crate::models::{normalize_pagination, now_rails_timestamp, BankAccount, PaginatedResult};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tauri::State;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BankAccountFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub bank_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BankAccountInput {
    pub bank_name: String,
    pub account_number: Option<i64>,
    pub bank_check_deposit_number: Option<i64>,
}

#[tauri::command]
pub async fn list_bank_accounts(
    filter: BankAccountFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<BankAccount>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_bank_accounts_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_bank_account(
    id: i64,
    state: State<'_, AppState>,
) -> Result<BankAccount, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_bank_account_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_bank_account(
    input: BankAccountInput,
    state: State<'_, AppState>,
) -> Result<BankAccount, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_bank_account_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_bank_account(
    id: i64,
    input: BankAccountInput,
    state: State<'_, AppState>,
) -> Result<BankAccount, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_bank_account_inner(&pool, user.id, id, input).await
}

pub async fn list_bank_accounts_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: BankAccountFilter,
) -> Result<PaginatedResult<BankAccount>, AppError> {
    let (page, per_page, offset) = normalize_pagination(filter.page, filter.per_page);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bank_accounts WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count bank accounts: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM bank_accounts WHERE user_id = ");
    count_qb.push_bind(user_id);
    apply_bank_account_filter(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered bank accounts: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, bank_name, bank_check_deposit_number, account_number, user_id, created_at, updated_at FROM bank_accounts WHERE user_id = ",
    );
    list_qb.push_bind(user_id);
    apply_bank_account_filter(&mut list_qb, &filter);
    list_qb.push(" ORDER BY id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<BankAccount> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list bank accounts: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_bank_account_filter<'a>(qb: &mut QueryBuilder<'a, Sqlite>, filter: &'a BankAccountFilter) {
    if let Some(ref bank_name) = filter.bank_name {
        if !bank_name.trim().is_empty() {
            qb.push(" AND LOWER(COALESCE(bank_name, '')) LIKE ");
            qb.push_bind(escape_like_pattern(bank_name));
            qb.push(" ESCAPE '\\'");
        }
    }
}

pub async fn get_bank_account_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<BankAccount, AppError> {
    sqlx::query_as::<_, BankAccount>(
        "SELECT id, bank_name, bank_check_deposit_number, account_number, user_id, created_at, updated_at FROM bank_accounts WHERE id = ? AND user_id = ?",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load bank account {id}: {e}")))?
    .ok_or_else(|| AppError::not_found(format!("Bank account {id} not found.")))
}

fn validate_bank_account_input(
    input: BankAccountInput,
) -> Result<(String, Option<i64>, Option<i64>), AppError> {
    let mut v = ValidationBuilder::new();
    let bank_name = v.required_len("bank_name", &input.bank_name, 2, 255);

    if let Some(num) = input.account_number {
        if num < 0 {
            v.push(
                "account_number",
                "invalid_integer",
                "account_number must be non-negative.",
            );
        }
    }

    if let Some(num) = input.bank_check_deposit_number {
        if num < 0 {
            v.push(
                "bank_check_deposit_number",
                "invalid_integer",
                "bank_check_deposit_number must be non-negative.",
            );
        }
    }

    v.finish()?;
    Ok((
        bank_name,
        input.account_number,
        input.bank_check_deposit_number,
    ))
}

pub async fn create_bank_account_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: BankAccountInput,
) -> Result<BankAccount, AppError> {
    let (bank_name, account_number, bank_check_deposit_number) =
        validate_bank_account_input(input)?;
    let now = now_rails_timestamp();

    let res = sqlx::query(
        "INSERT INTO bank_accounts (bank_name, account_number, bank_check_deposit_number, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&bank_name)
    .bind(account_number)
    .bind(bank_check_deposit_number)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to create bank account: {e}")))?;

    get_bank_account_inner(pool, user_id, res.last_insert_rowid()).await
}

pub async fn update_bank_account_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: BankAccountInput,
) -> Result<BankAccount, AppError> {
    let _existing = get_bank_account_inner(pool, user_id, id).await?;
    let (bank_name, account_number, bank_check_deposit_number) =
        validate_bank_account_input(input)?;
    let now = now_rails_timestamp();

    sqlx::query(
        "UPDATE bank_accounts SET bank_name = ?, account_number = ?, bank_check_deposit_number = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(&bank_name)
    .bind(account_number)
    .bind(bank_check_deposit_number)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update bank account {id}: {e}")))?;

    get_bank_account_inner(pool, user_id, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn validates_creates_updates_and_paginates_bank_accounts() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        let err = create_bank_account_inner(
            &pool,
            user.id,
            BankAccountInput {
                bank_name: " ".to_string(),
                account_number: Some(-1),
                bank_check_deposit_number: Some(-2),
            },
        )
        .await
        .unwrap_err();
        let fields = err.validation_errors().unwrap();
        assert!(fields
            .iter()
            .any(|f| f.field == "bank_name" && f.code == "required"));
        assert!(fields
            .iter()
            .any(|f| f.field == "account_number" && f.code == "invalid_integer"));
        assert!(fields
            .iter()
            .any(|f| f.field == "bank_check_deposit_number" && f.code == "invalid_integer"));

        for i in 1..=9 {
            create_bank_account_inner(
                &pool,
                user.id,
                BankAccountInput {
                    bank_name: format!("Banque {i}"),
                    account_number: Some(1000 + i),
                    bank_check_deposit_number: Some(i),
                },
            )
            .await
            .unwrap();
        }

        let page1 = list_bank_accounts_inner(&pool, user.id, BankAccountFilter::default())
            .await
            .unwrap();
        assert_eq!(page1.total, 9);
        assert_eq!(page1.items.len(), 8);

        let target_id = page1.items[0].id;
        let updated = update_bank_account_inner(
            &pool,
            user.id,
            target_id,
            BankAccountInput {
                bank_name: "BNP Paribas".to_string(),
                account_number: Some(987654),
                bank_check_deposit_number: Some(42),
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.bank_name.as_deref(), Some("BNP Paribas"));
        assert_eq!(updated.account_number, Some(987654));
        assert_eq!(updated.bank_check_deposit_number, Some(42));
    }
}
