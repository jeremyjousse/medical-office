use crate::db::AppState;
use crate::error::{AppError, ValidationBuilder};
use crate::models::{now_rails_timestamp, DocumentScan};
use image::{DynamicImage, ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fs;
use std::path::{Component, Path, PathBuf};
use tauri::State;

pub const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "gif", "png", "pdf"];
pub const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "gif", "png"];
pub const THUMB_MAX_DIM: u32 = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentScanView {
    pub id: i64,
    pub patient_id: Option<i64>,
    pub user_id: Option<i64>,
    pub file: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub file_path: Option<String>,
    pub thumb_path: Option<String>,
    pub file_exists: bool,
    pub thumb_exists: bool,
    pub is_pdf: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentScanInput {
    pub patient_id: i64,
    pub title: Option<String>,
    pub source_path: String,
}

#[tauri::command]
pub async fn list_document_scans(
    patient_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<DocumentScanView>, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    list_document_scans_inner(&pool, &state.app_data_dir, user.id, patient_id).await
}

#[tauri::command]
pub async fn get_document_scan(
    id: i64,
    state: State<'_, AppState>,
) -> Result<DocumentScanView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    get_document_scan_inner(&pool, &state.app_data_dir, user.id, id).await
}

#[tauri::command]
pub async fn create_document_scan(
    input: CreateDocumentScanInput,
    state: State<'_, AppState>,
) -> Result<DocumentScanView, AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    create_document_scan_inner(&pool, &state.app_data_dir, user.id, input).await
}

#[tauri::command]
pub async fn open_document_scan_in_viewer(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let (pool, user, _) = state.require_active_context().await?;
    let scan = get_document_scan_inner(&pool, &state.app_data_dir, user.id, id).await?;
    let Some(file_path) = scan.file_path.filter(|_| scan.file_exists) else {
        return Err(AppError::not_found(
            "Document scan file is missing on disk.",
        ));
    };

    open_path_in_macos_viewer(Path::new(&file_path))
}

#[cfg(target_os = "macos")]
fn open_path_in_macos_viewer(path: &Path) -> Result<(), AppError> {
    let status = std::process::Command::new("open")
        .arg(path)
        .status()
        .map_err(|e| AppError::message(format!("Failed to launch macOS viewer: {e}")))?;
    if !status.success() {
        return Err(AppError::message(format!(
            "macOS viewer exited with status {status}."
        )));
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn open_path_in_macos_viewer(_path: &Path) -> Result<(), AppError> {
    Err(AppError::message(
        "Opening in the default viewer is only supported on macOS.",
    ))
}

pub fn scan_storage_dir(app_data_dir: &Path, scan_id: i64) -> PathBuf {
    app_data_dir
        .join("uploads")
        .join("document_scan")
        .join("file")
        .join(scan_id.to_string())
}

fn is_safe_single_filename(name: &str) -> bool {
    if name.is_empty() || name == "." || name == ".." {
        return false;
    }
    if name.contains('/') || name.contains('\\') || name.contains('\0') {
        return false;
    }
    let mut components = Path::new(name).components();
    matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none()
}

fn file_extension_lowercase(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
}

pub fn sanitize_upload_filename(source_path: &Path) -> Result<(String, String), AppError> {
    let raw_name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| AppError::message("Selected file has an invalid filename."))?;

    let mut sanitized: String = raw_name
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || matches!(ch, '.' | '_' | '-') {
                ch
            } else {
                '_'
            }
        })
        .collect();

    while sanitized.starts_with('.') {
        sanitized.remove(0);
    }

    if !is_safe_single_filename(&sanitized) {
        return Err(AppError::message("Selected file has an invalid filename."));
    }

    let ext = file_extension_lowercase(&sanitized).unwrap_or_default();
    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(AppError::message(
            "Unsupported file extension. Allowed types: jpg, jpeg, gif, png, pdf.",
        ));
    }

    let stem = Path::new(&sanitized)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if stem.is_empty() {
        return Err(AppError::message("Selected file has an invalid filename."));
    }

    Ok((sanitized, ext))
}

fn build_scan_view(app_data_dir: &Path, row: DocumentScan) -> DocumentScanView {
    let safe_filename = row
        .file
        .as_deref()
        .map(str::trim)
        .filter(|name| is_safe_single_filename(name));

    let is_pdf = safe_filename
        .and_then(file_extension_lowercase)
        .map(|ext| ext == "pdf")
        .unwrap_or(false);

    let (file_exists, file_path, thumb_exists, thumb_path) = if let Some(filename) = safe_filename {
        let dir = scan_storage_dir(app_data_dir, row.id);
        let original = dir.join(filename);
        let orig_ok = original.is_file();

        let thumb = dir.join(format!("thumb_{filename}"));
        let thumb_ok = !is_pdf && thumb.is_file();

        (
            orig_ok,
            orig_ok.then(|| original.to_string_lossy().to_string()),
            thumb_ok,
            thumb_ok.then(|| thumb.to_string_lossy().to_string()),
        )
    } else {
        (false, None, false, None)
    };

    DocumentScanView {
        id: row.id,
        patient_id: row.patient_id,
        user_id: row.user_id,
        file: row.file,
        title: row.title,
        created_at: row.created_at,
        updated_at: row.updated_at,
        file_path,
        thumb_path,
        file_exists,
        thumb_exists,
        is_pdf,
    }
}

async fn ensure_patient_owned(
    pool: &SqlitePool,
    user_id: i64,
    patient_id: i64,
) -> Result<(), AppError> {
    let exists: Option<i64> =
        sqlx::query_scalar("SELECT id FROM patients WHERE id = ? AND user_id = ?")
            .bind(patient_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                AppError::message(format!("Failed to verify patient {patient_id}: {e}"))
            })?;

    if exists.is_none() {
        return Err(AppError::not_found(format!(
            "Patient {patient_id} not found."
        )));
    }
    Ok(())
}

pub async fn list_document_scans_inner(
    pool: &SqlitePool,
    app_data_dir: &Path,
    user_id: i64,
    patient_id: i64,
) -> Result<Vec<DocumentScanView>, AppError> {
    ensure_patient_owned(pool, user_id, patient_id).await?;

    let rows = sqlx::query_as::<_, DocumentScan>(
        "SELECT id, patient_id, user_id, file, title, created_at, updated_at
         FROM document_scans
         WHERE patient_id = ? AND (user_id = ? OR user_id IS NULL)
         ORDER BY created_at DESC, id DESC",
    )
    .bind(patient_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to list document scans: {e}")))?;

    Ok(rows
        .into_iter()
        .map(|row| build_scan_view(app_data_dir, row))
        .collect())
}

pub async fn get_document_scan_inner(
    pool: &SqlitePool,
    app_data_dir: &Path,
    user_id: i64,
    id: i64,
) -> Result<DocumentScanView, AppError> {
    let row = sqlx::query_as::<_, DocumentScan>(
        "SELECT ds.id, ds.patient_id, ds.user_id, ds.file, ds.title, ds.created_at, ds.updated_at
         FROM document_scans ds
         INNER JOIN patients p ON p.id = ds.patient_id
         WHERE ds.id = ? AND p.user_id = ? AND (ds.user_id = ? OR ds.user_id IS NULL)",
    )
    .bind(id)
    .bind(user_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::message(format!("Failed to load document scan {id}: {e}")))?
    .ok_or_else(|| AppError::not_found(format!("Document scan {id} not found.")))?;

    Ok(build_scan_view(app_data_dir, row))
}

fn image_format_for_ext(ext: &str) -> Option<ImageFormat> {
    match ext {
        "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
        "png" => Some(ImageFormat::Png),
        "gif" => Some(ImageFormat::Gif),
        _ => None,
    }
}

fn generate_thumbnail_image(source_path: &Path) -> Result<DynamicImage, AppError> {
    let img = ImageReader::open(source_path)
        .map_err(|e| AppError::message(format!("Failed to read image file: {e}")))?
        .with_guessed_format()
        .map_err(|e| AppError::message(format!("Failed to detect image format: {e}")))?
        .decode()
        .map_err(|e| AppError::message(format!("Invalid or corrupted image file: {e}")))?;

    Ok(img.thumbnail(THUMB_MAX_DIM, THUMB_MAX_DIM))
}

fn write_thumbnail_file(
    thumb: &DynamicImage,
    dest_path: &Path,
    format: ImageFormat,
) -> Result<(), AppError> {
    match format {
        // JPEG encoder does not support RGBA/alpha channels; flatten to RGB8 first.
        ImageFormat::Jpeg => thumb
            .to_rgb8()
            .save_with_format(dest_path, ImageFormat::Jpeg)
            .map_err(|e| AppError::message(format!("Failed to write JPEG thumbnail: {e}"))),
        _ => thumb
            .save_with_format(dest_path, format)
            .map_err(|e| AppError::message(format!("Failed to write thumbnail: {e}"))),
    }
}

pub async fn create_document_scan_inner(
    pool: &SqlitePool,
    app_data_dir: &Path,
    user_id: i64,
    input: CreateDocumentScanInput,
) -> Result<DocumentScanView, AppError> {
    ensure_patient_owned(pool, user_id, input.patient_id).await?;

    let mut v = ValidationBuilder::new();
    let title = v.optional_max_len("title", input.title.as_deref(), 255);
    let source_raw = input.source_path.trim();
    if source_raw.is_empty() {
        v.push("source_path", "required", "source_path is required.");
    }
    v.finish()?;

    let source_path = Path::new(source_raw);
    if !source_path.exists() || !source_path.is_file() {
        return Err(AppError::message("Selected scan file does not exist."));
    }

    let (filename, ext) = sanitize_upload_filename(source_path)?;

    let thumb_data = if IMAGE_EXTENSIONS.contains(&ext.as_str()) {
        let format = image_format_for_ext(&ext)
            .ok_or_else(|| AppError::message("Unsupported image format for thumbnail."))?;
        let src_owned = source_path.to_path_buf();
        let thumb =
            tauri::async_runtime::spawn_blocking(move || generate_thumbnail_image(&src_owned))
                .await
                .map_err(|e| AppError::message(format!("Thumbnail task failed: {e}")))??;
        Some((thumb, format))
    } else {
        None
    };

    let now = now_rails_timestamp();
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::message(format!("Failed to start transaction: {e}")))?;

    let res = sqlx::query(
        "INSERT INTO document_scans (patient_id, user_id, file, title, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(input.patient_id)
    .bind(user_id)
    .bind(&filename)
    .bind(&title)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::message(format!("Failed to save document scan record: {e}")))?;

    let scan_id = res.last_insert_rowid();
    let target_dir = scan_storage_dir(app_data_dir, scan_id);
    let src_copy = source_path.to_path_buf();
    let target_dir_clone = target_dir.clone();
    let filename_clone = filename.clone();

    let fs_res = tauri::async_runtime::spawn_blocking(move || -> Result<(), AppError> {
        fs::create_dir_all(&target_dir_clone)
            .map_err(|e| AppError::message(format!("Failed to create upload directory: {e}")))?;

        let dest_file = target_dir_clone.join(&filename_clone);
        if let Err(e) = fs::copy(&src_copy, &dest_file) {
            let _ = fs::remove_dir_all(&target_dir_clone);
            return Err(AppError::message(format!("Failed to copy scan file: {e}")));
        }

        if let Some((thumb, format)) = thumb_data {
            let thumb_dest = target_dir_clone.join(format!("thumb_{filename_clone}"));
            if let Err(err) = write_thumbnail_file(&thumb, &thumb_dest, format) {
                let _ = fs::remove_dir_all(&target_dir_clone);
                return Err(err);
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| AppError::message(format!("File upload task failed: {e}")));

    match fs_res {
        Ok(Ok(())) => {}
        Ok(Err(err)) => return Err(err),
        Err(err) => {
            let _ = fs::remove_dir_all(&target_dir);
            return Err(err);
        }
    }

    if let Err(e) = tx.commit().await {
        let _ = fs::remove_dir_all(&target_dir);
        return Err(AppError::message(format!(
            "Failed to commit document scan: {e}"
        )));
    }

    get_document_scan_inner(pool, app_data_dir, user_id, scan_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::patients::{create_patient_inner, PatientInput};
    use crate::db::{initialize_empty_schema_and_seed, resolve_active_user};
    use image::{ImageBuffer, Rgba};
    use sqlx::sqlite::SqlitePoolOptions;
    use tempfile::TempDir;

    async fn setup_patient_fixture() -> (SqlitePool, TempDir, i64, i64) {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        initialize_empty_schema_and_seed(&pool).await.unwrap();
        let (user, _) = resolve_active_user(&pool, None).await.unwrap();
        let app_dir = TempDir::new().unwrap();

        let patient = create_patient_inner(
            &pool,
            user.id,
            PatientInput {
                first_name: Some("Clara".to_string()),
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

        (pool, app_dir, user.id, patient.id)
    }

    #[tokio::test]
    async fn uploads_image_creates_carrierwave_path_and_100x100_aspect_fit_thumb() {
        let (pool, app_dir, user_id, patient_id) = setup_patient_fixture().await;
        let src_dir = TempDir::new().unwrap();

        // Create a 400x200 RGBA PNG and also save a JPEG copy (tests RGBA -> RGB8 flattening for JPEG thumbs)
        let rgba = ImageBuffer::from_pixel(400, 200, Rgba([20u8, 120u8, 220u8, 255u8]));
        let dyn_img = DynamicImage::ImageRgba8(rgba);

        let png_src = src_dir.path().join("my radio scan.png");
        dyn_img
            .save_with_format(&png_src, ImageFormat::Png)
            .unwrap();

        let created_png = create_document_scan_inner(
            &pool,
            app_dir.path(),
            user_id,
            CreateDocumentScanInput {
                patient_id,
                title: Some("Foot X-Ray".to_string()),
                source_path: png_src.to_string_lossy().to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(created_png.file.as_deref(), Some("my_radio_scan.png"));
        assert_eq!(created_png.title.as_deref(), Some("Foot X-Ray"));
        assert!(created_png.file_exists);
        assert!(created_png.thumb_exists);
        assert!(!created_png.is_pdf);

        let expected_orig = app_dir
            .path()
            .join("uploads/document_scan/file")
            .join(created_png.id.to_string())
            .join("my_radio_scan.png");
        let expected_thumb = app_dir
            .path()
            .join("uploads/document_scan/file")
            .join(created_png.id.to_string())
            .join("thumb_my_radio_scan.png");

        assert!(expected_orig.is_file());
        assert!(expected_thumb.is_file());

        let thumb_img = image::open(&expected_thumb).unwrap();
        assert_eq!((thumb_img.width(), thumb_img.height()), (100, 50));

        // Test JPEG upload from an RGB image
        let jpg_src = src_dir.path().join("Ordonnance.JPG");
        dyn_img
            .to_rgb8()
            .save_with_format(&jpg_src, ImageFormat::Jpeg)
            .unwrap();
        let created_jpg = create_document_scan_inner(
            &pool,
            app_dir.path(),
            user_id,
            CreateDocumentScanInput {
                patient_id,
                title: None,
                source_path: jpg_src.to_string_lossy().to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(created_jpg.file.as_deref(), Some("Ordonnance.JPG"));
        assert!(created_jpg.file_exists);
        assert!(created_jpg.thumb_exists);
        let jpg_thumb = image::open(created_jpg.thumb_path.as_ref().unwrap()).unwrap();
        assert_eq!((jpg_thumb.width(), jpg_thumb.height()), (100, 50));
    }

    #[tokio::test]
    async fn uploads_pdf_without_thumbnail_and_handles_missing_legacy_file_gracefully() {
        let (pool, app_dir, user_id, patient_id) = setup_patient_fixture().await;
        let src_dir = TempDir::new().unwrap();

        let pdf_src = src_dir.path().join("prescription.pdf");
        fs::write(&pdf_src, b"%PDF-1.4\n%mock\n").unwrap();

        let created_pdf = create_document_scan_inner(
            &pool,
            app_dir.path(),
            user_id,
            CreateDocumentScanInput {
                patient_id,
                title: Some("Prescription".to_string()),
                source_path: pdf_src.to_string_lossy().to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(created_pdf.file.as_deref(), Some("prescription.pdf"));
        assert!(created_pdf.file_exists);
        assert!(!created_pdf.thumb_exists);
        assert!(created_pdf.thumb_path.is_none());
        assert!(created_pdf.is_pdf);

        // Insert a legacy DB row with no file on disk and another with a traversal attempt
        sqlx::query(
            "INSERT INTO document_scans (patient_id, user_id, file, title, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(patient_id)
        .bind(user_id)
        .bind("legacy_scan.jpg")
        .bind("Legacy Scan")
        .bind("2015-02-01 10:00:00.000000")
        .bind("2015-02-01 10:00:00.000000")
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO document_scans (patient_id, user_id, file, title, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(patient_id)
        .bind(user_id)
        .bind("../../etc/passwd")
        .bind("Traversal")
        .bind("2015-01-01 10:00:00.000000")
        .bind("2015-01-01 10:00:00.000000")
        .execute(&pool)
        .await
        .unwrap();

        let scans = list_document_scans_inner(&pool, app_dir.path(), user_id, patient_id)
            .await
            .unwrap();
        assert_eq!(scans.len(), 3);
        assert_eq!(scans[0].id, created_pdf.id);

        let legacy = scans
            .iter()
            .find(|s| s.title.as_deref() == Some("Legacy Scan"))
            .unwrap();
        assert!(!legacy.file_exists);
        assert!(legacy.file_path.is_none());
        assert!(!legacy.thumb_exists);

        let traversal = scans
            .iter()
            .find(|s| s.title.as_deref() == Some("Traversal"))
            .unwrap();
        assert!(!traversal.file_exists);
        assert!(traversal.file_path.is_none());
    }

    #[tokio::test]
    async fn rejects_disallowed_extensions_and_corrupt_images_atomically() {
        let (pool, app_dir, user_id, patient_id) = setup_patient_fixture().await;
        let src_dir = TempDir::new().unwrap();

        let txt_src = src_dir.path().join("notes.txt");
        fs::write(&txt_src, b"not allowed").unwrap();

        let err = create_document_scan_inner(
            &pool,
            app_dir.path(),
            user_id,
            CreateDocumentScanInput {
                patient_id,
                title: Some("Invalid".to_string()),
                source_path: txt_src.to_string_lossy().to_string(),
            },
        )
        .await
        .unwrap_err();
        assert!(
            matches!(err, AppError::Message { ref message } if message.contains("Unsupported file extension"))
        );

        let corrupt_jpg = src_dir.path().join("corrupt.jpg");
        fs::write(&corrupt_jpg, b"not-actually-a-jpeg").unwrap();

        let err2 = create_document_scan_inner(
            &pool,
            app_dir.path(),
            user_id,
            CreateDocumentScanInput {
                patient_id,
                title: Some("Corrupt".to_string()),
                source_path: corrupt_jpg.to_string_lossy().to_string(),
            },
        )
        .await
        .unwrap_err();
        assert!(matches!(err2, AppError::Message { .. }));

        // No rows were persisted for either failed upload
        let scans = list_document_scans_inner(&pool, app_dir.path(), user_id, patient_id)
            .await
            .unwrap();
        assert!(scans.is_empty());
    }
}
