use crate::database::DbState;
use crate::database::DownloadJob;
use crate::downloader::DownloadOptions;
use crate::services::queue::{self, QueueAddRequest, QueueAddResponse};
use tauri::State;

#[tauri::command]
pub fn queue_list(db: State<'_, DbState>, limit: Option<i64>) -> Result<Vec<DownloadJob>, String> {
    queue::list(&db, limit).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_add(
    db: State<'_, DbState>,
    request: QueueAddRequest,
) -> Result<QueueAddResponse, String> {
    queue::add(&db, request).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_pause(db: State<'_, DbState>, id: String) -> Result<(), String> {
    queue::set_state(&db, &id, "waiting", None).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_resume(db: State<'_, DbState>, id: String) -> Result<(), String> {
    queue::set_state(&db, &id, "waiting", None).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_cancel(db: State<'_, DbState>, id: String) -> Result<(), String> {
    queue::set_state(&db, &id, "cancelled", None).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_retry(db: State<'_, DbState>, id: String) -> Result<(), String> {
    queue::set_state(&db, &id, "waiting", None).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn queue_preview_args(options: DownloadOptions) -> Result<Vec<String>, String> {
    queue::preview_args(options).map_err(|e| e.to_command_error())
}
