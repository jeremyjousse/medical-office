use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use medical_office_lib::commands::bank_accounts::{
    create_bank_account_inner, list_bank_accounts_inner, update_bank_account_inner,
    BankAccountFilter, BankAccountInput,
};
use medical_office_lib::commands::bank_deposits::{
    create_bank_deposit_inner, get_bank_deposit_inner, list_bank_deposits_inner,
    list_pending_bank_checks_inner, BankDepositFilter, BankDepositInput,
};
use medical_office_lib::commands::document_scans::{
    create_document_scan_inner, get_document_scan_inner, list_document_scans_inner,
    CreateDocumentScanInput,
};
use medical_office_lib::commands::office::{
    get_home_summary_inner, list_countries_inner, update_office_inner, OfficeInput,
};
use medical_office_lib::commands::patients::{
    create_patient_inner, finder_patients_inner, get_patient_inner, list_patients_inner,
    update_patient_inner, PatientFilter, PatientInput,
};
use medical_office_lib::commands::payments::{
    create_payment_inner, list_payment_listing_inner, list_payments_inner, update_payment_inner,
    PaymentBankCheckInput, PaymentFilter, PaymentInput, PaymentListingFilter,
};
use medical_office_lib::commands::practitioners::{
    create_practitioner_inner, list_practitioners_inner, update_practitioner_inner,
    PractitionerFilter, PractitionerInput,
};
use medical_office_lib::commands::treatment_types::{
    create_treatment_type_inner, list_treatment_types_inner, update_treatment_type_inner,
    TreatmentTypeFilter, TreatmentTypeInput,
};
use medical_office_lib::commands::treatments::{
    create_treatment_inner, list_treatments_inner, update_treatment_inner, MedicalTreatmentInput,
    TreatmentFilter,
};
use medical_office_lib::db::AppState;
use medical_office_lib::seed::{COUNTRIES_SEED, SCHEMA_SQL};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Row;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

async fn snapshot_sqlite_master(db_path: &Path) -> BTreeMap<(String, String), String> {
    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .read_only(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await
        .unwrap();

    let rows = sqlx::query(
        "SELECT type, name, COALESCE(sql, '') AS sql
         FROM sqlite_master
         WHERE name NOT LIKE 'sqlite_%'
         ORDER BY type, name",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    pool.close().await;

    let mut map = BTreeMap::new();
    for r in rows {
        let typ: String = r.get("type");
        let name: String = r.get("name");
        let sql: String = r.get("sql");
        map.insert((typ, name), sql);
    }
    map
}

#[tokio::test]
async fn e2e_rails_sqlite_import_full_crud_and_zero_schema_drift() {
    let tmp = TempDir::new().unwrap();
    let legacy_db = tmp.path().join("rails_production_copy.sqlite3");

    // 1. Build a legacy Rails 4.2 SQLite database copy with schema_migrations + legacy rows
    {
        let opts = SqliteConnectOptions::new()
            .filename(&legacy_db)
            .create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
            .unwrap();

        sqlx::raw_sql(SCHEMA_SQL).execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE schema_migrations (version varchar NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE UNIQUE INDEX unique_schema_migrations ON schema_migrations (version)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO schema_migrations (version) VALUES ('20150130165215')")
            .execute(&pool)
            .await
            .unwrap();

        for &(id, iso, name) in COUNTRIES_SEED {
            sqlx::query("INSERT INTO countries (id, iso, name) VALUES (?, ?, ?)")
                .bind(id)
                .bind(iso)
                .bind(name)
                .execute(&pool)
                .await
                .unwrap();
        }

        sqlx::query(
            "INSERT INTO offices (id, name, address, city, postal_code, country_id, phone, email, created_at, updated_at)
             VALUES (1, 'Cabinet Podologie', '10 Rue Pasteur', 'Lyon', '69007', 75, '0472000000', 'contact@cabinet.fr', '2015-01-10 09:00:00.000000', '2015-01-10 09:00:00.000000')",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO users (id, email, encrypted_password, sign_in_count, office_id, created_at, updated_at)
             VALUES (1, 'praticien@cabinet.fr', '$2a$10$legacyhash', 12, 1, '2015-01-10 09:00:00.000000', '2015-01-10 09:00:00.000000')",
        )
        .execute(&pool)
        .await
        .unwrap();

        // Legacy patient with Rails 't'/'f' booleans and legacy document_scan row (missing on disk)
        sqlx::query(
            "INSERT INTO patients (id, first_name, last_name, city, country_id, diabetic, universal_healthcare_coverage, long_duration_disease, user_id, created_at, updated_at)
             VALUES (1, 'Jean', 'Valjean', 'Paris', 75, 't', 'f', 't', 1, '2015-01-11 10:00:00.000000', '2015-01-11 10:00:00.000000')",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO document_scans (id, patient_id, user_id, file, title, created_at, updated_at)
             VALUES (1, 1, 1, 'legacy_radio.jpg', 'Legacy Radio', '2015-01-12 11:00:00.000000', '2015-01-12 11:00:00.000000')",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool.close().await;
    }

    let schema_before = snapshot_sqlite_master(&legacy_db).await;

    // 2. Import legacy DB into AppState
    let app_dir = tmp.path().join("app_data");
    let state = AppState::new(app_dir.clone());
    let status = state.import_sqlite_db(&legacy_db).await.unwrap();
    assert!(status.initialized);
    assert!(!status.needs_office_setup);
    assert_eq!(status.active_user.as_ref().unwrap().id, 1);

    let (pool, user, office) = state.require_active_context().await.unwrap();

    // 3. Exercise all screens / commands end-to-end
    let countries = list_countries_inner(&pool).await.unwrap();
    assert_eq!(countries.len(), 251);

    assert_eq!(office.name.as_deref(), Some("Cabinet Podologie"));
    let updated_office = update_office_inner(
        &pool,
        office.id,
        OfficeInput {
            name: "Cabinet Podologie Lyon".to_string(),
            address: "10 Rue Pasteur".to_string(),
            postal_code: "69007".to_string(),
            city: "Lyon".to_string(),
            country_id: Some(75),
            phone: "0472000000".to_string(),
            website: Some("https://cabinet.fr".to_string()),
            email: "contact@cabinet.fr".to_string(),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        updated_office.name.as_deref(),
        Some("Cabinet Podologie Lyon")
    );

    // Practitioners
    let prac = create_practitioner_inner(
        &pool,
        user.id,
        PractitionerInput {
            speciality_id: Some(1),
            first_name: "Paul".to_string(),
            last_name: "Martin".to_string(),
            phone: Some("0472111111".to_string()),
            mobile_phone: None,
            email: None,
            address: "2 Place Bellecour".to_string(),
            postal_code: "69002".to_string(),
            city: "Lyon".to_string(),
            country_id: Some(75),
            note: None,
        },
    )
    .await
    .unwrap();
    update_practitioner_inner(
        &pool,
        user.id,
        prac.id,
        PractitionerInput {
            speciality_id: Some(1),
            first_name: "Paul".to_string(),
            last_name: "Martin".to_string(),
            phone: Some("0472111112".to_string()),
            mobile_phone: None,
            email: None,
            address: "2 Place Bellecour".to_string(),
            postal_code: "69002".to_string(),
            city: "Lyon".to_string(),
            country_id: Some(75),
            note: Some("Updated".to_string()),
        },
    )
    .await
    .unwrap();
    let pracs = list_practitioners_inner(&pool, user.id, PractitionerFilter::default())
        .await
        .unwrap();
    assert_eq!(pracs.total, 1);

    // Treatment types
    let tt = create_treatment_type_inner(
        &pool,
        user.id,
        TreatmentTypeInput {
            name: "Bilan podologique".to_string(),
            price: Some(55.0),
        },
    )
    .await
    .unwrap();
    update_treatment_type_inner(
        &pool,
        user.id,
        tt.id,
        TreatmentTypeInput {
            name: "Bilan podologique complet".to_string(),
            price: Some(60.0),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        list_treatment_types_inner(&pool, user.id, TreatmentTypeFilter::default())
            .await
            .unwrap()
            .total,
        1
    );

    // Bank accounts
    let ba = create_bank_account_inner(
        &pool,
        user.id,
        BankAccountInput {
            bank_name: "Credit Mutuel".to_string(),
            account_number: Some(123456789),
            bank_check_deposit_number: Some(41),
        },
    )
    .await
    .unwrap();
    update_bank_account_inner(
        &pool,
        user.id,
        ba.id,
        BankAccountInput {
            bank_name: "Credit Mutuel Lyon".to_string(),
            account_number: Some(123456789),
            bank_check_deposit_number: Some(41),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        list_bank_accounts_inner(&pool, user.id, BankAccountFilter::default())
            .await
            .unwrap()
            .total,
        1
    );

    // Patients (read legacy patient + create + filter + update + finder)
    let legacy_patient = get_patient_inner(&pool, user.id, 1).await.unwrap();
    assert_eq!(legacy_patient.diabetic.map(|b| b.0), Some(true));
    assert_eq!(
        legacy_patient.universal_healthcare_coverage.map(|b| b.0),
        Some(false)
    );

    let new_patient = create_patient_inner(
        &pool,
        user.id,
        PatientInput {
            first_name: Some("sophie".to_string()),
            last_name: "moreau".to_string(),
            birthdate: Some("1990-05-14".to_string()),
            profession: None,
            phone: None,
            mobile_phone: Some("0612345678".to_string()),
            email: None,
            address: None,
            postal_code: None,
            city: "lyon".to_string(),
            country_id: Some(75),
            national_insurance_number: None,
            family_doctor_id: Some(prac.id),
            physical_therapists_id: None,
            osteopath_id: None,
            legal_guardian_id: None,
            note: None,
            diabetic: true,
            universal_healthcare_coverage: false,
            long_duration_disease: false,
            diabete_network_id: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(new_patient.first_name.as_deref(), Some("Sophie"));
    assert_eq!(new_patient.last_name.as_deref(), Some("Moreau"));

    update_patient_inner(
        &pool,
        user.id,
        new_patient.id,
        PatientInput {
            first_name: Some("sophie".to_string()),
            last_name: "moreau".to_string(),
            birthdate: Some("1990-05-14".to_string()),
            profession: Some("Designer".to_string()),
            phone: None,
            mobile_phone: Some("0612345678".to_string()),
            email: None,
            address: None,
            postal_code: None,
            city: "lyon".to_string(),
            country_id: Some(75),
            national_insurance_number: None,
            family_doctor_id: Some(prac.id),
            physical_therapists_id: None,
            osteopath_id: None,
            legal_guardian_id: None,
            note: None,
            diabetic: true,
            universal_healthcare_coverage: false,
            long_duration_disease: false,
            diabete_network_id: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        list_patients_inner(
            &pool,
            user.id,
            PatientFilter {
                diabetic: Some(true),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .filtered_total,
        2
    );
    assert_eq!(
        finder_patients_inner(&pool, user.id, "mor", None)
            .await
            .unwrap()
            .len(),
        1
    );

    // Medical treatments
    let treatment = create_treatment_inner(
        &pool,
        user.id,
        MedicalTreatmentInput {
            patient_id: Some(new_patient.id),
            date: "2026-04-01".to_string(),
            location_id: Some(1),
            medical_treatment_type_id: Some(tt.id),
            price: Some(60.0),
            note: Some("Semelles".to_string()),
        },
    )
    .await
    .unwrap();
    assert_eq!(treatment.status, Some(0));
    assert_eq!(
        get_home_summary_inner(&pool, user.id, updated_office.clone())
            .await
            .unwrap()
            .unpaid_medical_treatments_count,
        1
    );

    update_treatment_inner(
        &pool,
        user.id,
        treatment.id,
        MedicalTreatmentInput {
            patient_id: Some(new_patient.id),
            date: "2026-04-01".to_string(),
            location_id: Some(2),
            medical_treatment_type_id: Some(tt.id),
            price: Some(60.0),
            note: Some("Visite domicile".to_string()),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        list_treatments_inner(&pool, user.id, TreatmentFilter::default())
            .await
            .unwrap()
            .total,
        1
    );

    // Payments (check payment -> updates treatment to paid -> payment listing)
    let pay = create_payment_inner(
        &pool,
        user.id,
        PaymentInput {
            medical_treatment_id: Some(treatment.id),
            paid_at: "2026-04-01".to_string(),
            payment_type: Some(1),
            amount: Some(60.0),
            payment_bank_check: Some(PaymentBankCheckInput {
                account_owner: "Sophie Moreau".to_string(),
                bank_name: "BNP".to_string(),
                check_number: "998877".to_string(),
            }),
        },
    )
    .await
    .unwrap();
    update_payment_inner(
        &pool,
        user.id,
        pay.id,
        PaymentInput {
            medical_treatment_id: Some(treatment.id),
            paid_at: "2026-04-02".to_string(),
            payment_type: Some(1),
            amount: Some(60.0),
            payment_bank_check: Some(PaymentBankCheckInput {
                account_owner: "Sophie Moreau".to_string(),
                bank_name: "BNP Paribas".to_string(),
                check_number: "998877".to_string(),
            }),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        list_payments_inner(&pool, user.id, PaymentFilter::default())
            .await
            .unwrap()
            .total,
        1
    );
    let listing = list_payment_listing_inner(&pool, user.id, PaymentListingFilter::default())
        .await
        .unwrap();
    assert_eq!(listing.grand_total_amount, 60.0);

    // Bank deposit
    let pending = list_pending_bank_checks_inner(&pool, user.id, "2026-04-02")
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    let deposit_detail = create_bank_deposit_inner(
        &pool,
        user.id,
        updated_office.clone(),
        BankDepositInput {
            deposit_type: Some(1),
            bank_account_id: Some(ba.id),
            payment_bank_check_ids: vec![pending[0].id],
        },
    )
    .await
    .unwrap();
    assert_eq!(deposit_detail.deposit.number, Some(42));
    assert_eq!(
        get_bank_deposit_inner(&pool, user.id, updated_office, deposit_detail.deposit.id)
            .await
            .unwrap()
            .checks
            .len(),
        1
    );
    assert_eq!(
        list_bank_deposits_inner(&pool, user.id, BankDepositFilter::default())
            .await
            .unwrap()
            .total,
        1
    );

    // Document scans: legacy missing file + new image upload + new PDF upload
    let legacy_scans = list_document_scans_inner(&pool, &app_dir, user.id, legacy_patient.id)
        .await
        .unwrap();
    assert_eq!(legacy_scans.len(), 1);
    assert!(!legacy_scans[0].file_exists);

    let upload_src = tmp.path().join("src_files");
    fs::create_dir_all(&upload_src).unwrap();
    let img_file = upload_src.join("scan_test.png");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
        200,
        100,
        Rgba([10u8, 20u8, 30u8, 255u8]),
    ))
    .save_with_format(&img_file, ImageFormat::Png)
    .unwrap();
    let pdf_file = upload_src.join("report.pdf");
    fs::write(&pdf_file, b"%PDF-1.4\n%e2e\n").unwrap();

    let uploaded_img = create_document_scan_inner(
        &pool,
        &app_dir,
        user.id,
        CreateDocumentScanInput {
            patient_id: new_patient.id,
            title: Some("Podoscope".to_string()),
            source_path: img_file.to_string_lossy().to_string(),
        },
    )
    .await
    .unwrap();
    assert!(uploaded_img.file_exists);
    assert!(uploaded_img.thumb_exists);

    let uploaded_pdf = create_document_scan_inner(
        &pool,
        &app_dir,
        user.id,
        CreateDocumentScanInput {
            patient_id: new_patient.id,
            title: Some("Ordonnance PDF".to_string()),
            source_path: pdf_file.to_string_lossy().to_string(),
        },
    )
    .await
    .unwrap();
    assert!(uploaded_pdf.file_exists);
    assert!(uploaded_pdf.is_pdf);
    assert!(!uploaded_pdf.thumb_exists);
    assert!(
        get_document_scan_inner(&pool, &app_dir, user.id, uploaded_pdf.id)
            .await
            .is_ok()
    );

    // 4. Export DB and verify zero schema drift against pre-import Rails DB
    let exported_db = tmp.path().join("exported.sqlite3");
    state.export_sqlite_db(&exported_db).await.unwrap();
    let schema_after = snapshot_sqlite_master(&exported_db).await;
    assert_eq!(
        schema_before, schema_after,
        "Schema drift detected between imported Rails DB and exported DB"
    );
}
