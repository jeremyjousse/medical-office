use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef};
use sqlx::{Decode, Encode, Type, TypeInfo, ValueRef};

/// Boolean wrapper compatible with Rails 4.2 SQLite storage (`'t'`/`'f'`, `1`/`0`, `true`/`false`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RailsBool(pub bool);

impl Type<Sqlite> for RailsBool {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(
            ty.name(),
            "BOOLEAN"
                | "INTEGER"
                | "INT"
                | "INT4"
                | "INT8"
                | "BIGINT"
                | "TEXT"
                | "VARCHAR"
                | "NULL"
        )
    }
}

impl<'r> Decode<'r, Sqlite> for RailsBool {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        if value.is_null() {
            return Ok(RailsBool(false));
        }
        match value.type_info().name() {
            "INTEGER" | "INT" | "INT4" | "INT8" | "BIGINT" | "BOOLEAN" => {
                let n = <i64 as Decode<Sqlite>>::decode(value)?;
                Ok(RailsBool(n != 0))
            }
            "REAL" | "FLOAT" | "DOUBLE" => {
                let f = <f64 as Decode<Sqlite>>::decode(value)?;
                Ok(RailsBool(f != 0.0))
            }
            _ => {
                let s = <String as Decode<Sqlite>>::decode(value)?;
                Ok(RailsBool(parse_rails_bool_str(&s)))
            }
        }
    }
}

impl<'q> Encode<'q, Sqlite> for RailsBool {
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'q>>) -> Result<IsNull, BoxDynError> {
        buf.push(SqliteArgumentValue::Text(if self.0 {
            "t".into()
        } else {
            "f".into()
        }));
        Ok(IsNull::No)
    }
}

pub fn parse_rails_bool_str(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "t" | "true" | "1" | "yes" | "y" | "on"
    )
}

/// Decimal/Numeric wrapper tolerant of SQLite `REAL`, `INTEGER`, and `TEXT` storage.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RailsDecimal(pub f64);

impl Type<Sqlite> for RailsDecimal {
    fn type_info() -> SqliteTypeInfo {
        <f64 as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(
            ty.name(),
            "REAL"
                | "FLOAT"
                | "DOUBLE"
                | "NUMERIC"
                | "DECIMAL"
                | "INTEGER"
                | "INT"
                | "INT8"
                | "TEXT"
                | "NULL"
        )
    }
}

impl<'r> Decode<'r, Sqlite> for RailsDecimal {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        if value.is_null() {
            return Ok(RailsDecimal(0.0));
        }
        match value.type_info().name() {
            "INTEGER" | "INT" | "INT4" | "INT8" | "BIGINT" => {
                let n = <i64 as Decode<Sqlite>>::decode(value)?;
                Ok(RailsDecimal(n as f64))
            }
            "TEXT" | "VARCHAR" => {
                let s = <String as Decode<Sqlite>>::decode(value)?;
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    Ok(RailsDecimal(0.0))
                } else {
                    Ok(RailsDecimal(trimmed.parse::<f64>()?))
                }
            }
            _ => {
                let f = <f64 as Decode<Sqlite>>::decode(value)?;
                Ok(RailsDecimal(f))
            }
        }
    }
}

impl<'q> Encode<'q, Sqlite> for RailsDecimal {
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'q>>) -> Result<IsNull, BoxDynError> {
        buf.push(SqliteArgumentValue::Double(self.0));
        Ok(IsNull::No)
    }
}

/// Parse a Rails SQLite date or datetime string into a `NaiveDate`.
/// Handles `"YYYY-MM-DD"`, `"YYYY-MM-DD HH:MM:SS..."`, `"YYYY-MM-DDTHH:MM:SS..."`, and truncates >10 char `paid_at`.
pub fn parse_rails_date(raw: &str) -> Option<NaiveDate> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        return Some(d);
    }
    if let Some(dt) = parse_rails_datetime(trimmed) {
        return Some(dt.date());
    }
    if trimmed.len() >= 10 {
        return NaiveDate::parse_from_str(&trimmed[..10], "%Y-%m-%d").ok();
    }
    None
}

/// Normalize a Rails date string to `"YYYY-MM-DD"` (used for `paid_at` / date fields).
pub fn normalize_rails_date(raw: &str) -> Option<String> {
    parse_rails_date(raw).map(|d| d.format("%Y-%m-%d").to_string())
}

/// Parse Rails 4.2 SQLite timestamp formats (`"YYYY-MM-DD HH:MM:SS.ffffff"`, ISO8601, or date-only).
pub fn parse_rails_datetime(raw: &str) -> Option<NaiveDateTime> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    const FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.fZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
    ];
    for fmt in FORMATS {
        if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, fmt) {
            return Some(dt);
        }
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        return d.and_hms_opt(0, 0, 0);
    }
    None
}

/// Current UTC timestamp formatted as Rails 4.2 SQLite `datetime` (`YYYY-MM-DD HH:MM:SS.ffffff`).
pub fn now_rails_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// Current local date formatted as `"YYYY-MM-DD"`.
pub fn today_date_str() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Exact integer-cent conversion for decimal comparisons (`sum(payments) == price`).
pub fn to_cents(amount: f64) -> i64 {
    (amount * 100.0).round() as i64
}

pub fn from_cents(cents: i64) -> f64 {
    cents as f64 / 100.0
}

pub fn sum_cents(amounts: &[f64]) -> i64 {
    amounts.iter().map(|&a| to_cents(a)).sum()
}

pub fn amounts_equal(a: f64, b: f64) -> bool {
    to_cents(a) == to_cents(b)
}

pub const DEFAULT_PER_PAGE: i64 = 8;
pub const MAX_LISTING_PER_PAGE: i64 = 10_000_000;

pub fn normalize_pagination(page: Option<i64>, per_page: Option<i64>) -> (i64, i64, i64) {
    let per_page = per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, 100);
    let page = page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;
    (page, per_page, offset)
}

pub fn normalize_listing_pagination(
    page: Option<i64>,
    per_page: Option<i64>,
    default_per_page: i64,
) -> (i64, i64, i64) {
    let per_page = per_page
        .unwrap_or(default_per_page)
        .clamp(1, MAX_LISTING_PER_PAGE);
    let page = page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;
    (page, per_page, offset)
}

/// Word capitalization matching Rails `Patient#set_first_name_and_last_name_uppercase` (`gsub(/\b(\w)/)`).
pub fn titlecase_words(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut at_word_start = true;
    for ch in raw.chars() {
        if ch.is_alphanumeric() {
            if at_word_start {
                for upper in ch.to_uppercase() {
                    out.push(upper);
                }
                at_word_start = false;
            } else {
                out.push(ch);
            }
        } else {
            out.push(ch);
            at_word_start = true;
        }
    }
    out
}

/// HTML entity escaping matching Ruby `CGI::escapeHTML`.
pub fn escape_html_rails(raw: &str) -> String {
    html_escape::encode_double_quoted_attribute(raw).replace('\'', "&#39;")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub filtered_total: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub office_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Office {
    pub id: i64,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country_id: Option<i64>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Country {
    pub id: i64,
    pub name: Option<String>,
    pub iso: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Patient {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthdate: Option<String>,
    pub profession: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub country_id: Option<i64>,
    pub national_insurance_number: Option<String>,
    pub family_doctor_id: Option<i64>,
    pub physical_therapists_id: Option<i64>,
    pub osteopath_id: Option<i64>,
    pub legal_guardian_id: Option<i64>,
    pub note: Option<String>,
    pub diabetic: Option<RailsBool>,
    pub universal_healthcare_coverage: Option<RailsBool>,
    pub long_duration_disease: Option<RailsBool>,
    pub diabete_network_id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Practitioner {
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
    pub note: Option<String>,
    pub speciality_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MedicalTreatmentType {
    pub id: i64,
    pub name: Option<String>,
    pub price: Option<RailsDecimal>,
    pub user_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MedicalTreatment {
    pub id: i64,
    pub user_id: Option<i64>,
    pub patient_id: Option<i64>,
    pub date: Option<String>,
    pub location_id: Option<i64>,
    pub medical_treatment_type_id: Option<i64>,
    pub price: Option<RailsDecimal>,
    pub note: Option<String>,
    pub status: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: i64,
    pub medical_treatment_id: Option<i64>,
    pub paid_at: Option<String>,
    pub payment_type: Option<i64>,
    pub amount: Option<RailsDecimal>,
    pub user_id: Option<i64>,
    pub bank_deposit_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaymentBankCheck {
    pub id: i64,
    pub account_owner: Option<String>,
    pub bank_name: Option<String>,
    pub check_number: Option<String>,
    pub amount: Option<RailsDecimal>,
    pub status: Option<i64>,
    pub bank_deposit_id: Option<i64>,
    pub payment_id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankAccount {
    pub id: i64,
    pub bank_name: Option<String>,
    pub bank_check_deposit_number: Option<i64>,
    pub account_number: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankDeposit {
    pub id: i64,
    pub deposit_type: Option<i64>,
    pub number: Option<i64>,
    pub amount: Option<RailsDecimal>,
    pub user_id: Option<i64>,
    pub deposit_date: Option<String>,
    pub bank_account_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DocumentScan {
    pub id: i64,
    pub patient_id: Option<i64>,
    pub user_id: Option<i64>,
    pub file: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[test]
    fn parses_rails_dates_and_datetimes() {
        assert_eq!(
            normalize_rails_date("2014-04-07"),
            Some("2014-04-07".to_string())
        );
        assert_eq!(
            normalize_rails_date("2014-04-07 14:22:05.123456"),
            Some("2014-04-07".to_string())
        );
        assert_eq!(
            normalize_rails_date("2014-04-07T09:10:00Z"),
            Some("2014-04-07".to_string())
        );
        assert_eq!(normalize_rails_date("   "), None);

        assert!(parse_rails_datetime("2014-05-30 08:03:54.123456").is_some());
        assert!(parse_rails_datetime("2014-05-30T08:03:54Z").is_some());
        assert!(parse_rails_datetime("2014-05-30").is_some());
        assert!(parse_rails_datetime("").is_none());
    }

    #[test]
    fn handles_cents_arithmetic_without_float_drift() {
        let payments = [19.99, 20.01, 10.00];
        assert_eq!(sum_cents(&payments), 5000);
        assert!(amounts_equal(0.1 + 0.2, 0.3));
        assert!(!amounts_equal(49.99, 50.00));
        assert_eq!(from_cents(5025), 50.25);
    }

    #[test]
    fn titlecases_words_and_escapes_html_rails_style() {
        assert_eq!(titlecase_words("jean-pierre"), "Jean-Pierre");
        assert_eq!(titlecase_words("10 rue de la paix"), "10 Rue De La Paix");
        assert_eq!(titlecase_words("école d'art"), "École D'Art");
        assert_eq!(titlecase_words("McDonald"), "McDonald");
        assert_eq!(titlecase_words(""), "");

        assert_eq!(
            escape_html_rails(r#"A & B <tag> "q" 's' 1980/05/12"#),
            "A &amp; B &lt;tag&gt; &quot;q&quot; &#39;s&#39; 1980/05/12"
        );
    }

    #[test]
    fn normalizes_listing_pagination_including_all() {
        assert_eq!(normalize_listing_pagination(None, None, 10), (1, 10, 0));
        assert_eq!(
            normalize_listing_pagination(Some(3), Some(20), 10),
            (3, 20, 40)
        );
        assert_eq!(
            normalize_listing_pagination(Some(1), Some(10_000_000), 10),
            (1, 10_000_000, 0)
        );
    }

    #[tokio::test]
    async fn decodes_rails_sqlite_booleans_and_decimals() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE t (id INTEGER PRIMARY KEY, b1 TEXT, b2 INTEGER, b3 TEXT, d1 NUMERIC, d2 TEXT, d3 INTEGER)",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO t (id, b1, b2, b3, d1, d2, d3) VALUES (1, 't', 1, NULL, 55.5, '42.75', 60)",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO t (id, b1, b2, b3, d1, d2, d3) VALUES (2, 'f', 0, 'true', NULL, '', 0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        #[derive(sqlx::FromRow)]
        struct Row {
            id: i64,
            b1: Option<RailsBool>,
            b2: Option<RailsBool>,
            b3: Option<RailsBool>,
            d1: Option<RailsDecimal>,
            d2: Option<RailsDecimal>,
            d3: Option<RailsDecimal>,
        }

        let rows: Vec<Row> = sqlx::query_as("SELECT * FROM t ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows[0].id, 1);
        assert_eq!(rows[0].b1, Some(RailsBool(true)));
        assert_eq!(rows[0].b2, Some(RailsBool(true)));
        assert_eq!(rows[0].b3, None);
        assert_eq!(rows[0].d1, Some(RailsDecimal(55.5)));
        assert_eq!(rows[0].d2, Some(RailsDecimal(42.75)));
        assert_eq!(rows[0].d3, Some(RailsDecimal(60.0)));

        assert_eq!(rows[1].id, 2);
        assert_eq!(rows[1].b1, Some(RailsBool(false)));
        assert_eq!(rows[1].b2, Some(RailsBool(false)));
        assert_eq!(rows[1].b3, Some(RailsBool(true)));
        assert_eq!(rows[1].d1, None);
        assert_eq!(rows[1].d2, Some(RailsDecimal(0.0)));
        assert_eq!(rows[1].d3, Some(RailsDecimal(0.0)));
    }
}
