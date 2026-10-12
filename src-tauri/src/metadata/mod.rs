use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSidecars {
    pub description_path: Option<String>,
    pub info_json_path: Option<String>,
    pub thumbnail_path: Option<String>,
}

pub fn write_description(path: &Path, body: &str) -> AppResult<()> {
    if body.len() > 2_000_000 {
        return Err(AppError::validation("description too large"));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, body)?;
    Ok(())
}

pub fn write_info_json(path: &Path, value: &serde_json::Value) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let pretty =
        serde_json::to_string_pretty(value).map_err(|e| AppError::Internal(e.to_string()))?;
    fs::write(path, pretty)?;
    Ok(())
}
