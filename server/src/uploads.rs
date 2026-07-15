// server/src/uploads.rs
//
// File upload + serving.
//
// - POST   /upload      — authenticated. Accepts multipart/form-data with a
//                          `file` field (and optional `public` bool field).
//                          Streams the file to disk under UPLOAD_DIR and
//                          records metadata in the `file` SurrealDB table.
// - GET    /files/<id>  — serves the file's bytes if it is public, or the
//                          caller is the owner/an admin. 404 otherwise (to
//                          avoid leaking existence of private files).
// - DELETE /files/<id>  — authenticated. Removes the DB record and the file
//                          on disk. Owner or admin only.

use rocket::{get, post, delete, State};
use rocket::form::{Form, FromForm};
use rocket::fs::{TempFile, NamedFile};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb_types::{SurrealValue, ToSql};
use surrealdb::types::RecordId;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::env;
use std::path::PathBuf;
use log::{error, info};

use crate::auth::{Db, AuthUser};
use crate::error::Error;

/// Directory on disk where uploaded files are persisted.
/// Defaults to `./uploads` locally; set `UPLOAD_DIR=/uploads` in Docker to
/// match the `uploads-volume` mount in `docker-compose.yml`.
fn upload_dir() -> PathBuf {
    let dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
    PathBuf::from(dir)
}

/// Derives a safe, lowercase file extension (no leading dot) from the
/// upload's Content-Type, if any. Returns `None` if the content type has no
/// known extension.
fn safe_extension(file: &TempFile<'_>) -> Option<String> {
    file.content_type()
        .and_then(|ct| ct.extension())
        .map(|ext| ext.as_str().to_lowercase())
}

/// Metadata persisted for every uploaded file.
#[derive(Debug, Deserialize, SurrealValue)]
struct FileRecord {
    id: RecordId,
    filename: String,
    original_name: String,
    content_type: String,
    size: i64,
    owner: RecordId,
    public: bool,
    #[allow(dead_code)]
    created_at: DateTime<Utc>,
}

/// Input used to create a new `file` record.
#[derive(Debug, Serialize, SurrealValue)]
struct FileCreate {
    filename: String,
    original_name: String,
    content_type: String,
    size: i64,
    owner: RecordId,
    public: bool,
}

#[derive(FromForm)]
pub struct UploadForm<'f> {
    file: TempFile<'f>,
    public: Option<bool>,
}

/// Looks up a single `file` record by its record-id key (the part after the
/// `:`). Returns `Ok(None)` if no such record exists.
async fn find_file(db: &Db, id: &str) -> Result<Option<FileRecord>, Error> {
    let mut records: Vec<FileRecord> = db.client
        .query("SELECT * FROM file WHERE id = type::thing('file', $id) LIMIT 1")
        .bind(("id", id.to_string()))
        .await
        .map_err(|e| {
            error!("Failed to query file record {}: {:?}", id, e);
            Error::Internal("Database error".to_string())
        })?
        .take(0)
        .map_err(|e| {
            error!("Failed to take file query result: {:?}", e);
            Error::Internal("Database error".to_string())
        })?;

    Ok(if records.is_empty() { None } else { Some(records.remove(0)) })
}

fn is_authorized(record: &FileRecord, user: &Option<AuthUser>) -> bool {
    if record.public {
        return true;
    }
    match user {
        Some(u) => u.id.to_sql() == record.owner.to_sql() || u.role == "admin",
        None => false,
    }
}

#[post("/upload", data = "<form>")]
pub async fn upload(
    user: AuthUser,
    db: &State<Db>,
    mut form: Form<UploadForm<'_>>,
) -> Result<Json<Value>, Error> {
    let ext = safe_extension(&form.file);
    let stored_filename = match ext {
        Some(ext) => format!("{}.{}", Uuid::new_v4(), ext),
        None => Uuid::new_v4().to_string(),
    };

    let content_type = form.file
        .content_type()
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let original_name = form.file
        .raw_name()
        .and_then(|n| n.as_str())
        .unwrap_or("upload")
        .to_string();

    let size = form.file.len() as i64;

    let dir = upload_dir();
    tokio::fs::create_dir_all(&dir).await.map_err(Error::from)?;
    let dest = dir.join(&stored_filename);

    form.file.persist_to(&dest).await.map_err(|e| {
        error!("Failed to persist uploaded file: {:?}", e);
        Error::Internal("Failed to save uploaded file".to_string())
    })?;

    let file_create = FileCreate {
        filename: stored_filename.clone(),
        original_name,
        content_type,
        size,
        owner: user.id.clone(),
        public: form.public.unwrap_or(false),
    };

    let created: Option<FileRecord> = db.client
        .create("file")
        .content(file_create)
        .await
        .map_err(|e| {
            error!("Failed to create file record: {:?}", e);
            Error::Internal("Failed to record uploaded file".to_string())
        })?;

    let created = created.ok_or_else(|| Error::BadRequest("Couldn't create file record".to_string()))?;

    let id_key = created.id.to_sql().split(':').last().unwrap_or_default().to_string();

    info!("File uploaded: id={} owner={}", id_key, user.email);

    Ok(Json(json!({
        "id": id_key,
        "url": format!("/files/{}", id_key),
        "filename": created.filename,
        "size": created.size,
        "public": created.public,
    })))
}

#[get("/files/<id>")]
pub async fn get_file(
    id: String,
    db: &State<Db>,
    user: Option<AuthUser>,
) -> Result<NamedFile, Error> {
    let record = find_file(db.inner(), &id).await?.ok_or(Error::NotFound)?;

    if !is_authorized(&record, &user) {
        // 404 rather than 403 to avoid leaking existence of private files.
        return Err(Error::NotFound);
    }

    let path = upload_dir().join(&record.filename);
    NamedFile::open(path).await.map_err(|e| {
        error!("Failed to open file {} on disk: {:?}", record.filename, e);
        Error::NotFound
    })
}

#[delete("/files/<id>")]
pub async fn delete_file(
    id: String,
    db: &State<Db>,
    user: AuthUser,
) -> Result<Json<Value>, Error> {
    let record = find_file(db.inner(), &id).await?.ok_or(Error::NotFound)?;

    let is_owner_or_admin = record.owner.to_sql() == user.id.to_sql() || user.role == "admin";
    if !is_owner_or_admin {
        return Err(Error::Forbidden);
    }

    db.client

        .query("DELETE file WHERE id = type::thing('file', $id)")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| {
            error!("Failed to delete file record {}: {:?}", id, e);
            Error::Internal("Failed to delete file record".to_string())
        })?;

    let path = upload_dir().join(&record.filename);
    if let Err(e) = tokio::fs::remove_file(&path).await {
        // Don't fail the request if the DB row is already gone and disk cleanup
        // fails (e.g. already removed); just log it.
        error!("Failed to remove file {} from disk: {:?}", path.display(), e);
    }

    info!("File deleted: id={} by={}", id, user.email);
    Ok(Json(json!({"success": true})))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_dir_defaults_to_uploads() {
        // SAFETY: test-only mutation of process env; no other test reads UPLOAD_DIR
        // concurrently within this crate's test binary at this exact moment.
        unsafe { env::remove_var("UPLOAD_DIR"); }
        assert_eq!(upload_dir(), PathBuf::from("uploads"));
    }

    #[test]
    fn upload_dir_respects_env_var() {
        unsafe { env::set_var("UPLOAD_DIR", "/tmp/custom-uploads"); }
        assert_eq!(upload_dir(), PathBuf::from("/tmp/custom-uploads"));
        unsafe { env::remove_var("UPLOAD_DIR"); }
    }
}
