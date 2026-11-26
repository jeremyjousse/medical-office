use crate::db::AppState;
use crate::error::{escape_like_pattern, AppError, ValidationBuilder};
use crate::models::{
    escape_html_rails, normalize_listing_pagination, normalize_rails_date, now_rails_timestamp,
    titlecase_words, PaginatedResult, RailsBool,
};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::State;

pub const DEFAULT_PATIENTS_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PatientView {
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
    pub country_name: Option<String>,
    pub national_insurance_number: Option<String>,
    pub family_doctor_id: Option<i64>,
    pub family_doctor_label: Option<String>,
    pub physical_therapists_id: Option<i64>,
    pub physical_therapists_label: Option<String>,
    pub osteopath_id: Option<i64>,
    pub osteopath_label: Option<String>,
    pub legal_guardian_id: Option<i64>,
    pub note: Option<String>,
    pub diabetic: Option<RailsBool>,
    pub universal_healthcare_coverage: Option<RailsBool>,
    pub long_duration_disease: Option<RailsBool>,
    pub diabete_network_id: Option<i64>,
    pub user_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub treatments_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PatientChoice {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PractitionerOption {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub speciality_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PatientFilter {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub diabetic: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatientInput {
    pub first_name: Option<String>,
    pub last_name: String,
    pub birthdate: Option<String>,
    pub profession: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub city: String,
    pub country_id: Option<i64>,
    pub national_insurance_number: Option<String>,
    pub family_doctor_id: Option<i64>,
    pub physical_therapists_id: Option<i64>,
    pub osteopath_id: Option<i64>,
    pub legal_guardian_id: Option<i64>,
    pub note: Option<String>,
    #[serde(default)]
    pub diabetic: bool,
    #[serde(default)]
    pub universal_healthcare_coverage: bool,
    #[serde(default)]
    pub long_duration_disease: bool,
    pub diabete_network_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPatientsResult {
    pub imported_count: i64,
    pub skipped_count: i64,
    pub total_rows: i64,
}

const PATIENT_SELECT_SQL: &str = "
SELECT
    p.id,
    p.first_name,
    p.last_name,
    p.birthdate,
    p.profession,
    p.phone,
    p.mobile_phone,
    p.email,
    p.address,
    p.postal_code,
    p.city,
    p.country_id,
    c.name AS country_name,
    p.national_insurance_number,
    p.family_doctor_id,
    CASE
        WHEN fd.id IS NOT NULL THEN TRIM(COALESCE(fd.first_name, '') || ' ' || COALESCE(fd.last_name, ''))
            || CASE WHEN COALESCE(fd.city, '') != '' THEN ' (' || fd.city || ')' ELSE '' END
        ELSE NULL
    END AS family_doctor_label,
    p.physical_therapists_id,
    CASE
        WHEN pt.id IS NOT NULL THEN TRIM(COALESCE(pt.first_name, '') || ' ' || COALESCE(pt.last_name, ''))
            || CASE WHEN COALESCE(pt.city, '') != '' THEN ' (' || pt.city || ')' ELSE '' END
        ELSE NULL
    END AS physical_therapists_label,
    p.osteopath_id,
    CASE
        WHEN os.id IS NOT NULL THEN TRIM(COALESCE(os.first_name, '') || ' ' || COALESCE(os.last_name, ''))
            || CASE WHEN COALESCE(os.city, '') != '' THEN ' (' || os.city || ')' ELSE '' END
        ELSE NULL
    END AS osteopath_label,
    p.legal_guardian_id,
    p.note,
    p.diabetic,
    p.universal_healthcare_coverage,
    p.long_duration_disease,
    p.diabete_network_id,
    p.user_id,
    p.created_at,
    p.updated_at,
    COALESCE(mt.cnt, 0) AS treatments_count
FROM patients p
LEFT JOIN countries c ON c.id = p.country_id
LEFT JOIN practitioners fd ON fd.id = p.family_doctor_id
LEFT JOIN practitioners pt ON pt.id = p.physical_therapists_id
LEFT JOIN practitioners os ON os.id = p.osteopath_id
LEFT JOIN (
    SELECT patient_id, COUNT(*) AS cnt
    FROM medical_treatments
    WHERE user_id = ";

#[tauri::command]
pub async fn list_patients(
    filter: PatientFilter,
    state: State<'_, AppState>,
) -> Result<PaginatedResult<PatientView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_patients_inner(&pool, user.id, filter).await
}

#[tauri::command]
pub async fn get_patient(id: i64, state: State<'_, AppState>) -> Result<PatientView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_patient_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn create_patient(
    input: PatientInput,
    state: State<'_, AppState>,
) -> Result<PatientView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_patient_inner(&pool, user.id, input).await
}

#[tauri::command]
pub async fn update_patient(
    id: i64,
    input: PatientInput,
    state: State<'_, AppState>,
) -> Result<PatientView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    update_patient_inner(&pool, user.id, id, input).await
}

#[tauri::command]
pub async fn delete_patient(id: i64, state: State<'_, AppState>) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    delete_patient_inner(&pool, user.id, id).await
}

#[tauri::command]
pub async fn finder_patients(
    query: String,
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<PatientChoice>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    finder_patients_inner(&pool, user.id, &query, limit).await
}

#[tauri::command]
pub async fn list_practitioner_options(
    state: State<'_, AppState>,
) -> Result<Vec<PractitionerOption>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_practitioner_options_inner(&pool, user.id).await
}

#[tauri::command]
pub async fn import_patients_csv(
    path: String,
    state: State<'_, AppState>,
) -> Result<ImportPatientsResult, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    import_patients_csv_inner(&pool, user.id, Path::new(&path)).await
}

pub async fn list_patients_inner(
    pool: &SqlitePool,
    user_id: i64,
    filter: PatientFilter,
) -> Result<PaginatedResult<PatientView>, AppError> {
    let (page, per_page, offset) =
        normalize_listing_pagination(filter.page, filter.per_page, DEFAULT_PATIENTS_PER_PAGE);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count patients: {e}")))?;

    let mut count_qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT COUNT(*) FROM patients p WHERE p.user_id = ");
    count_qb.push_bind(user_id);
    apply_patient_filters(&mut count_qb, &filter);

    let filtered_total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to count filtered patients: {e}")))?;

    let mut list_qb: QueryBuilder<Sqlite> = QueryBuilder::new(PATIENT_SELECT_SQL);
    list_qb.push_bind(user_id);
    list_qb.push(" GROUP BY patient_id) mt ON mt.patient_id = p.id WHERE p.user_id = ");
    list_qb.push_bind(user_id);
    apply_patient_filters(&mut list_qb, &filter);
    list_qb.push(" ORDER BY p.id DESC LIMIT ");
    list_qb.push_bind(per_page);
    list_qb.push(" OFFSET ");
    list_qb.push_bind(offset);

    let items: Vec<PatientView> = list_qb
        .build_query_as()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to list patients: {e}")))?;

    Ok(PaginatedResult {
        items,
        page,
        per_page,
        filtered_total,
        total,
    })
}

fn apply_patient_filters<'a>(qb: &mut QueryBuilder<'a, Sqlite>, filter: &'a PatientFilter) {
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
    if let Some(ref phone) = filter.phone {
        if !phone.trim().is_empty() {
            let pat = escape_like_pattern(phone);
            qb.push(" AND (LOWER(COALESCE(p.phone, '')) LIKE ");
            qb.push_bind(pat.clone());
            qb.push(" ESCAPE '\\' OR LOWER(COALESCE(p.mobile_phone, '')) LIKE ");
            qb.push_bind(pat);
            qb.push(" ESCAPE '\\')");
        }
    }
    if let Some(true) = filter.diabetic {
        qb.push(" AND LOWER(COALESCE(CAST(p.diabetic AS TEXT), '')) IN ('t', 'true', '1')");
    }
}

pub async fn get_patient_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<PatientView, AppError> {
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(PATIENT_SELECT_SQL);
    qb.push_bind(user_id);
    qb.push(" GROUP BY patient_id) mt ON mt.patient_id = p.id WHERE p.id = ");
    qb.push_bind(id);
    qb.push(" AND p.user_id = ");
    qb.push_bind(user_id);

    qb.build_query_as::<PatientView>()
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to load patient {id}: {e}")))?
        .ok_or_else(|| AppError::not_found(format!("Patient {id} not found.")))
}

struct ValidatedPatient {
    first_name: Option<String>,
    last_name: String,
    birthdate: Option<String>,
    profession: Option<String>,
    phone: Option<String>,
    mobile_phone: Option<String>,
    email: Option<String>,
    address: Option<String>,
    postal_code: Option<String>,
    city: String,
    country_id: i64,
    national_insurance_number: Option<String>,
    family_doctor_id: Option<i64>,
    physical_therapists_id: Option<i64>,
    osteopath_id: Option<i64>,
    legal_guardian_id: Option<i64>,
    note: Option<String>,
    diabetic: RailsBool,
    universal_healthcare_coverage: RailsBool,
    long_duration_disease: RailsBool,
    diabete_network_id: Option<i64>,
}

async fn validate_practitioner_ref(
    pool: &SqlitePool,
    user_id: i64,
    field: &str,
    practitioner_id: Option<i64>,
    v: &mut ValidationBuilder,
) -> Result<Option<i64>, AppError> {
    match practitioner_id {
        Some(pid) if pid > 0 => {
            let exists: Option<i64> =
                sqlx::query_scalar("SELECT id FROM practitioners WHERE id = ? AND user_id = ?")
                    .bind(pid)
                    .bind(user_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| AppError::message(format!("Failed to verify {field}: {e}")))?;
            if exists.is_none() {
                v.push(
                    field,
                    "invalid_practitioner",
                    format!("Selected {field} is invalid."),
                );
                Ok(None)
            } else {
                Ok(Some(pid))
            }
        }
        Some(_) => {
            v.push(
                field,
                "invalid_practitioner",
                format!("Selected {field} is invalid."),
            );
            Ok(None)
        }
        None => Ok(None),
    }
}

async fn validate_patient_input(
    pool: &SqlitePool,
    user_id: i64,
    input: PatientInput,
) -> Result<ValidatedPatient, AppError> {
    let mut v = ValidationBuilder::new();

    let first_name = v
        .optional_max_len("first_name", input.first_name.as_deref(), 255)
        .map(|s| titlecase_words(&s));
    let last_name = titlecase_words(&v.required_len("last_name", &input.last_name, 2, 255));
    let birthdate_raw = v.optional_max_len("birthdate", input.birthdate.as_deref(), 100);
    let birthdate = birthdate_raw.map(|raw| normalize_rails_date(&raw).unwrap_or(raw));
    let profession = v.optional_max_len("profession", input.profession.as_deref(), 255);
    let phone = v.optional_max_len("phone", input.phone.as_deref(), 255);
    let mobile_phone = v.optional_max_len("mobile_phone", input.mobile_phone.as_deref(), 255);
    let email = v.optional_max_len("email", input.email.as_deref(), 255);
    let address = v
        .optional_max_len("address", input.address.as_deref(), 255)
        .map(|s| titlecase_words(&s));
    let postal_code = v.optional_max_len("postal_code", input.postal_code.as_deref(), 255);
    let city = titlecase_words(&v.required_len("city", &input.city, 2, 255));
    let national_insurance_number = v.optional_max_len(
        "national_insurance_number",
        input.national_insurance_number.as_deref(),
        255,
    );
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

    let family_doctor_id = validate_practitioner_ref(
        pool,
        user_id,
        "family_doctor_id",
        input.family_doctor_id,
        &mut v,
    )
    .await?;
    let physical_therapists_id = validate_practitioner_ref(
        pool,
        user_id,
        "physical_therapists_id",
        input.physical_therapists_id,
        &mut v,
    )
    .await?;
    let osteopath_id =
        validate_practitioner_ref(pool, user_id, "osteopath_id", input.osteopath_id, &mut v)
            .await?;

    v.finish()?;

    Ok(ValidatedPatient {
        first_name,
        last_name,
        birthdate,
        profession,
        phone,
        mobile_phone,
        email,
        address,
        postal_code,
        city,
        country_id,
        national_insurance_number,
        family_doctor_id,
        physical_therapists_id,
        osteopath_id,
        legal_guardian_id: input.legal_guardian_id,
        note,
        diabetic: RailsBool(input.diabetic),
        universal_healthcare_coverage: RailsBool(input.universal_healthcare_coverage),
        long_duration_disease: RailsBool(input.long_duration_disease),
        diabete_network_id: input.diabete_network_id,
    })
}

pub async fn create_patient_inner(
    pool: &SqlitePool,
    user_id: i64,
    input: PatientInput,
) -> Result<PatientView, AppError> {
    let data = validate_patient_input(pool, user_id, input).await?;
    let now = now_rails_timestamp();

    let res = sqlx::query(
        "INSERT INTO patients (first_name, last_name, birthdate, profession, phone, mobile_phone, email, address, postal_code, city, country_id, national_insurance_number, family_doctor_id, physical_therapists_id, osteopath_id, legal_guardian_id, note, diabetic, universal_healthcare_coverage, long_duration_disease, diabete_network_id, user_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.birthdate)
    .bind(&data.profession)
    .bind(&data.phone)
    .bind(&data.mobile_phone)
    .bind(&data.email)
    .bind(&data.address)
    .bind(&data.postal_code)
    .bind(&data.city)
    .bind(data.country_id)
    .bind(&data.national_insurance_number)
    .bind(data.family_doctor_id)
    .bind(data.physical_therapists_id)
    .bind(data.osteopath_id)
    .bind(data.legal_guardian_id)
    .bind(&data.note)
    .bind(data.diabetic)
    .bind(data.universal_healthcare_coverage)
    .bind(data.long_duration_disease)
    .bind(data.diabete_network_id)
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to create patient: {e}")))?;

    get_patient_inner(pool, user_id, res.last_insert_rowid()).await
}

pub async fn update_patient_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
    input: PatientInput,
) -> Result<PatientView, AppError> {
    let _existing = get_patient_inner(pool, user_id, id).await?;
    let data = validate_patient_input(pool, user_id, input).await?;
    let now = now_rails_timestamp();

    sqlx::query(
        "UPDATE patients SET first_name = ?, last_name = ?, birthdate = ?, profession = ?, phone = ?, mobile_phone = ?, email = ?, address = ?, postal_code = ?, city = ?, country_id = ?, national_insurance_number = ?, family_doctor_id = ?, physical_therapists_id = ?, osteopath_id = ?, legal_guardian_id = ?, note = ?, diabetic = ?, universal_healthcare_coverage = ?, long_duration_disease = ?, diabete_network_id = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.birthdate)
    .bind(&data.profession)
    .bind(&data.phone)
    .bind(&data.mobile_phone)
    .bind(&data.email)
    .bind(&data.address)
    .bind(&data.postal_code)
    .bind(&data.city)
    .bind(data.country_id)
    .bind(&data.national_insurance_number)
    .bind(data.family_doctor_id)
    .bind(data.physical_therapists_id)
    .bind(data.osteopath_id)
    .bind(data.legal_guardian_id)
    .bind(&data.note)
    .bind(data.diabetic)
    .bind(data.universal_healthcare_coverage)
    .bind(data.long_duration_disease)
    .bind(data.diabete_network_id)
    .bind(&now)
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to update patient {id}: {e}")))?;

    get_patient_inner(pool, user_id, id).await
}

pub async fn delete_patient_inner(
    pool: &SqlitePool,
    user_id: i64,
    id: i64,
) -> Result<(), AppError> {
    let _existing = get_patient_inner(pool, user_id, id).await?;

    let treatments_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM medical_treatments WHERE patient_id = ? AND user_id = ?",
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to verify patient treatments: {e}")))?;

    if treatments_count > 0 {
        return Err(AppError::blocked(
            "patient_has_treatments",
            "Cannot delete patient because they have existing medical treatments.",
        ));
    }

    sqlx::query("DELETE FROM patients WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to delete patient {id}: {e}")))?;

    Ok(())
}

pub async fn finder_patients_inner(
    pool: &SqlitePool,
    user_id: i64,
    query: &str,
    limit: Option<i64>,
) -> Result<Vec<PatientChoice>, AppError> {
    let limit = limit.unwrap_or(25).clamp(1, 100);
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, first_name, last_name, city, TRIM(COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')) || CASE WHEN COALESCE(city, '') != '' THEN ' (' || city || ')' ELSE '' END AS label FROM patients WHERE user_id = ",
    );
    qb.push_bind(user_id);

    if !query.trim().is_empty() {
        let pat = escape_like_pattern(query);
        qb.push(" AND (LOWER(COALESCE(last_name, '')) LIKE ");
        qb.push_bind(pat.clone());
        qb.push(" ESCAPE '\\' OR LOWER(COALESCE(first_name, '')) LIKE ");
        qb.push_bind(pat);
        qb.push(" ESCAPE '\\')");
    }

    qb.push(
        " ORDER BY LOWER(COALESCE(last_name, '')) ASC, LOWER(COALESCE(first_name, '')) ASC LIMIT ",
    );
    qb.push_bind(limit);

    qb.build_query_as::<PatientChoice>()
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::message(format!("Failed to search patients: {e}")))
}

pub async fn list_practitioner_options_inner(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<PractitionerOption>, AppError> {
    sqlx::query_as::<_, PractitionerOption>(
        "SELECT id, first_name, last_name, city, speciality_id FROM practitioners WHERE user_id = ? ORDER BY LOWER(COALESCE(last_name, '')) ASC, LOWER(COALESCE(first_name, '')) ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list practitioner options: {e}")))
}

fn decode_google_contacts_csv_bytes(bytes: &[u8]) -> Result<String, AppError> {
    if bytes.is_empty() {
        return Err(AppError::message("Selected CSV file is empty."));
    }

    let text = if bytes.starts_with(&[0xFF, 0xFE]) {
        let (decoded, _, _) = encoding_rs::UTF_16LE.decode(&bytes[2..]);
        decoded.into_owned()
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        let (decoded, _, _) = encoding_rs::UTF_16BE.decode(&bytes[2..]);
        decoded.into_owned()
    } else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        let (decoded, _, had_errors) = encoding_rs::UTF_8.decode(&bytes[3..]);
        if had_errors {
            return Err(AppError::message("CSV file is not valid UTF-8."));
        }
        decoded.into_owned()
    } else if bytes.len() >= 2 && bytes.len().is_multiple_of(2) && bytes[1] == 0 {
        let (decoded, _, _) = encoding_rs::UTF_16LE.decode(bytes);
        decoded.into_owned()
    } else {
        let (decoded, _, had_errors) = encoding_rs::UTF_8.decode(bytes);
        if !had_errors {
            decoded.into_owned()
        } else {
            let (fallback, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
            fallback.into_owned()
        }
    };

    Ok(text)
}

pub async fn import_patients_csv_inner(
    pool: &SqlitePool,
    user_id: i64,
    path: &Path,
) -> Result<ImportPatientsResult, AppError> {
    if !path.exists() || !path.is_file() {
        return Err(AppError::message("Selected CSV file does not exist."));
    }

    let bytes =
        fs::read(path).map_err(|e| AppError::message(format!("Failed to read CSV file: {e}")))?;
    let decoded = decode_google_contacts_csv_bytes(&bytes)?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(decoded.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| AppError::message(format!("Failed to parse CSV headers: {e}")))?
        .clone();

    let header_map: HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim(), i))
        .collect();

    if !header_map.contains_key("Family Name") {
        return Err(AppError::message(
            "Invalid Google Contacts CSV: missing 'Family Name' column.",
        ));
    }

    let mut total_rows: i64 = 0;
    let mut imported_count: i64 = 0;
    let mut skipped_count: i64 = 0;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::message(format!("Failed to begin import transaction: {e}")))?;

    let now = now_rails_timestamp();

    for result in reader.records() {
        let record =
            result.map_err(|e| AppError::message(format!("Failed to parse CSV row: {e}")))?;
        total_rows += 1;

        let raw_col = |name: &str| -> Option<String> {
            let idx = *header_map.get(name)?;
            let val = record.get(idx)?.trim();
            if val.is_empty() {
                None
            } else {
                Some(val.to_string())
            }
        };

        let Some(raw_last_name) = raw_col("Family Name") else {
            skipped_count += 1;
            continue;
        };

        let raw_first_name = raw_col("Given Name");
        let raw_phone = raw_col("Phone 1 - Value");

        let last_name_escaped = escape_html_rails(&titlecase_words(&raw_last_name));
        let first_name_escaped = raw_first_name
            .as_deref()
            .map(|s| escape_html_rails(&titlecase_words(s)));
        let phone_escaped = raw_phone.as_deref().map(escape_html_rails);

        // Deduplicate on (user_id, last_name, first_name, phone) matching both raw and escaped forms.
        let exists: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM patients WHERE user_id = ?
             AND (LOWER(COALESCE(last_name, '')) = LOWER(?) OR LOWER(COALESCE(last_name, '')) = LOWER(?))
             AND (LOWER(COALESCE(first_name, '')) = LOWER(?) OR LOWER(COALESCE(first_name, '')) = LOWER(?))
             AND (LOWER(COALESCE(phone, '')) = LOWER(?) OR LOWER(COALESCE(phone, '')) = LOWER(?))
             LIMIT 1",
        )
        .bind(user_id)
        .bind(&last_name_escaped)
        .bind(&raw_last_name)
        .bind(first_name_escaped.as_deref().unwrap_or(""))
        .bind(raw_first_name.as_deref().unwrap_or(""))
        .bind(phone_escaped.as_deref().unwrap_or(""))
        .bind(raw_phone.as_deref().unwrap_or(""))
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to check duplicate patient: {e}")))?;

        if exists.is_some() {
            skipped_count += 1;
            continue;
        }

        let birthdate = raw_col("Birthday").map(|s| escape_html_rails(&s));
        let profession = raw_col("Occupation").map(|s| escape_html_rails(&s));
        let email = raw_col("E-mail 1 - Value").map(|s| escape_html_rails(&s));
        let mobile_phone = raw_col("Phone 2 - Value").map(|s| escape_html_rails(&s));
        let address =
            raw_col("Address 1 - Street").map(|s| escape_html_rails(&titlecase_words(&s)));
        let postal_code = raw_col("Address 1 - Postal Code").map(|s| escape_html_rails(&s));
        let city = raw_col("Address 1 - City").map(|s| escape_html_rails(&titlecase_words(&s)));
        let note = raw_col("Notes").map(|s| escape_html_rails(&s));

        sqlx::query(
            "INSERT INTO patients (user_id, first_name, last_name, birthdate, profession, email, phone, mobile_phone, address, postal_code, city, note, diabetic, universal_healthcare_coverage, long_duration_disease, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'f', 'f', 'f', ?, ?)",
        )
        .bind(user_id)
        .bind(&first_name_escaped)
        .bind(&last_name_escaped)
        .bind(&birthdate)
        .bind(&profession)
        .bind(&email)
        .bind(&phone_escaped)
        .bind(&mobile_phone)
        .bind(&address)
        .bind(&postal_code)
        .bind(&city)
        .bind(&note)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::message(format!("Failed to insert imported patient: {e}")))?;

        imported_count += 1;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::message(format!("Failed to commit import transaction: {e}")))?;

    Ok(ImportPatientsResult {
        imported_count,
        skipped_count,
        total_rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::practitioners::{create_practitioner_inner, PractitionerInput};
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use sqlx::sqlite::SqlitePoolOptions;
    use tempfile::NamedTempFile;

    fn sample_patient(first_name: Option<&str>, last_name: &str) -> PatientInput {
        PatientInput {
            first_name: first_name.map(str::to_string),
            last_name: last_name.to_string(),
            birthdate: Some("1985-04-12".to_string()),
            profession: Some("Architect".to_string()),
            phone: Some("0142000001".to_string()),
            mobile_phone: Some("0600000001".to_string()),
            email: Some("patient@example.fr".to_string()),
            address: Some("12 rue de la paix".to_string()),
            postal_code: Some("75002".to_string()),
            city: "paris".to_string(),
            country_id: Some(75),
            national_insurance_number: Some("1850475123456".to_string()),
            family_doctor_id: None,
            physical_therapists_id: None,
            osteopath_id: None,
            legal_guardian_id: None,
            note: Some("Allergy check".to_string()),
            diabetic: false,
            universal_healthcare_coverage: false,
            long_duration_disease: false,
            diabete_network_id: None,
        }
    }

    #[tokio::test]
    async fn crud_titlecase_filters_pagination_and_delete_guard_for_patients() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        // Validation rejects short last_name/city and missing country
        let mut bad = sample_patient(Some("jean"), "A");
        bad.city = "P".to_string();
        bad.country_id = None;
        let err = create_patient_inner(&pool, user.id, bad).await.unwrap_err();
        let fields = err.validation_errors().unwrap();
        assert!(fields
            .iter()
            .any(|f| f.field == "last_name" && f.code == "too_short"));
        assert!(fields
            .iter()
            .any(|f| f.field == "city" && f.code == "too_short"));
        assert!(fields
            .iter()
            .any(|f| f.field == "country_id" && f.code == "required"));

        // Create practitioner for family_doctor_id
        let doc = create_practitioner_inner(
            &pool,
            user.id,
            PractitionerInput {
                speciality_id: Some(1),
                first_name: "Marc".to_string(),
                last_name: "House".to_string(),
                phone: None,
                mobile_phone: None,
                email: None,
                address: "1 Rue Santé".to_string(),
                postal_code: "69001".to_string(),
                city: "Lyon".to_string(),
                country_id: Some(75),
                note: None,
            },
        )
        .await
        .unwrap();

        let mut p1 = sample_patient(Some("jean-luc"), "picard");
        p1.diabetic = true;
        p1.family_doctor_id = Some(doc.id);
        let created = create_patient_inner(&pool, user.id, p1).await.unwrap();
        assert_eq!(created.first_name.as_deref(), Some("Jean-Luc"));
        assert_eq!(created.last_name.as_deref(), Some("Picard"));
        assert_eq!(created.address.as_deref(), Some("12 Rue De La Paix"));
        assert_eq!(created.city.as_deref(), Some("Paris"));
        assert_eq!(
            created.family_doctor_label.as_deref(),
            Some("Marc House (Lyon)")
        );

        let mut p2 = sample_patient(Some("marie"), "curie");
        p2.phone = Some("0199999999".to_string());
        p2.mobile_phone = Some("0788888888".to_string());
        let p2_created = create_patient_inner(&pool, user.id, p2).await.unwrap();

        // Filter by diabetic
        let diabetic_only = list_patients_inner(
            &pool,
            user.id,
            PatientFilter {
                diabetic: Some(true),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(diabetic_only.total, 2);
        assert_eq!(diabetic_only.filtered_total, 1);
        assert_eq!(diabetic_only.items[0].id, created.id);

        // Filter by phone OR mobile_phone
        let by_mobile = list_patients_inner(
            &pool,
            user.id,
            PatientFilter {
                phone: Some("078888".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(by_mobile.filtered_total, 1);
        assert_eq!(by_mobile.items[0].id, p2_created.id);

        // Pagination with per_page = All (10_000_000) and per_page = 1
        let per_one = list_patients_inner(
            &pool,
            user.id,
            PatientFilter {
                page: Some(2),
                per_page: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(per_one.items.len(), 1);
        assert_eq!(per_one.page, 2);

        // Finder autocomplete
        let found = finder_patients_inner(&pool, user.id, "pic", None)
            .await
            .unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].label, "Jean-Luc Picard (Paris)");

        // Block delete when patient has a treatment
        sqlx::query(
            "INSERT INTO medical_treatments (user_id, patient_id, status) VALUES (?, ?, 0)",
        )
        .bind(user.id)
        .bind(created.id)
        .execute(&pool)
        .await
        .unwrap();

        let blocked = delete_patient_inner(&pool, user.id, created.id)
            .await
            .unwrap_err();
        assert!(matches!(
            blocked,
            AppError::Blocked { ref code, .. } if code == "patient_has_treatments"
        ));

        // Delete works when treatments_count == 0
        delete_patient_inner(&pool, user.id, p2_created.id)
            .await
            .unwrap();
        let remaining = list_patients_inner(&pool, user.id, PatientFilter::default())
            .await
            .unwrap();
        assert_eq!(remaining.total, 1);
    }

    #[tokio::test]
    async fn imports_utf16le_bom_google_contacts_csv_with_escaping_and_deduplication() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();

        let csv_content = concat!(
            "Given Name,Family Name,Birthday,Occupation,E-mail 1 - Value,Phone 1 - Value,Phone 2 - Value,Address 1 - Street,Address 1 - Postal Code,Address 1 - City,Notes\n",
            "jean & luc,o'connor,1980-02-01,Engineer,jean@test.fr,0102030405,0602030405,5 <rue> victor,75001,paris,\"Note with \"\"quotes\"\"\"\n",
            ",MissingFamily,,Teacher,,0100000000,,,,,\n",
            "NoFamily,,,,,,,,,,\n"
        );

        let mut utf16_bytes = vec![0xFFu8, 0xFEu8];
        for unit in csv_content.encode_utf16() {
            utf16_bytes.extend_from_slice(&unit.to_le_bytes());
        }

        let tmp = NamedTempFile::new().unwrap();
        fs::write(tmp.path(), &utf16_bytes).unwrap();

        let first_run = import_patients_csv_inner(&pool, user.id, tmp.path())
            .await
            .unwrap();
        assert_eq!(first_run.total_rows, 3);
        assert_eq!(first_run.imported_count, 2);
        assert_eq!(first_run.skipped_count, 1);

        let listed = list_patients_inner(&pool, user.id, PatientFilter::default())
            .await
            .unwrap();
        assert_eq!(listed.total, 2);

        let jean = listed
            .items
            .iter()
            .find(|p| p.phone.as_deref() == Some("0102030405"))
            .unwrap();
        assert_eq!(jean.first_name.as_deref(), Some("Jean &amp; Luc"));
        assert_eq!(jean.last_name.as_deref(), Some("O&#39;Connor"));
        assert_eq!(jean.address.as_deref(), Some("5 &lt;Rue&gt; Victor"));
        assert_eq!(jean.city.as_deref(), Some("Paris"));
        assert_eq!(jean.note.as_deref(), Some("Note with &quot;quotes&quot;"));

        // Re-importing the exact same file deduplicates all rows
        let second_run = import_patients_csv_inner(&pool, user.id, tmp.path())
            .await
            .unwrap();
        assert_eq!(second_run.total_rows, 3);
        assert_eq!(second_run.imported_count, 0);
        assert_eq!(second_run.skipped_count, 3);
    }
}
