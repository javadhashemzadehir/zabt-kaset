use crate::database::{DbState, DownloadJob};
use crate::services::queue;
use tauri::State;

#[tauri::command]
pub fn history_search(
    db: State<'_, DbState>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<DownloadJob>, String> {
    queue::search_history(&db, &query, limit).map_err(|e| e.to_command_error())
}
