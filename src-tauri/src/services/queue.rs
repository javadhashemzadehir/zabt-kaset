use crate::database::{Database, DbState, DownloadJob};
use crate::downloader::{self, DownloadOptions};
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueAddRequest {
    pub urls: Vec<String>,
    pub source_type: Option<String>,
    pub priority: Option<i32>,
    pub preset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueAddResponse {
    pub job_ids: Vec<String>,
}

pub fn list(db: &DbState, limit: Option<i64>) -> AppResult<Vec<DownloadJob>> {
    db.with_conn(|conn| Database::list_jobs(conn, limit.unwrap_or(200)))
}

pub fn add(db: &DbState, req: QueueAddRequest) -> AppResult<QueueAddResponse> {
    if req.urls.is_empty() {
        return Err(AppError::validation("at least one url required"));
    }
    if req.urls.len() > 100 {
        return Err(AppError::validation("too many urls (max 100)"));
    }

    let source_type = req.source_type.unwrap_or_else(|| "video".into());
    validate_source_type(&source_type)?;
    let priority = req.priority.unwrap_or(0).clamp(-100, 100);

    let mut job_ids = Vec::new();
    db.with_conn(|conn| {
        for raw in &req.urls {
            let url = downloader::validate_url(raw)?;
            let id = Uuid::new_v4().to_string();
            Database::insert_job(
                conn,
                &id,
                url.as_str(),
                &source_type,
                priority,
                req.preset_id.as_deref(),
            )?;
            job_ids.push(id);
        }
        Ok(())
    })?;

    Ok(QueueAddResponse { job_ids })
}

pub fn set_state(db: &DbState, id: &str, state: &str, error: Option<&str>) -> AppResult<()> {
    validate_state(state)?;
    db.with_conn(|conn| Database::update_job_state(conn, id, state, error))
}

pub fn search_history(
    db: &DbState,
    query: &str,
    limit: Option<i64>,
) -> AppResult<Vec<DownloadJob>> {
    if query.len() > 200 {
        return Err(AppError::validation("query too long"));
    }
    db.with_conn(|conn| Database::search_history(conn, query, limit.unwrap_or(100)))
}

pub fn preview_args(opts: DownloadOptions) -> AppResult<Vec<String>> {
    downloader::build_args(&opts)
}

fn validate_source_type(value: &str) -> AppResult<()> {
    match value {
        "video" | "playlist" | "channel" | "shorts" | "batch" | "community" => Ok(()),
        _ => Err(AppError::validation("invalid source_type")),
    }
}

fn validate_state(value: &str) -> AppResult<()> {
    match value {
        "waiting" | "analyzing" | "downloading" | "processing" | "completed" | "failed"
        | "cancelled" => Ok(()),
        _ => Err(AppError::validation("invalid state")),
    }
}
