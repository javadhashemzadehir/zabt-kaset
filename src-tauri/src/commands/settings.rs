use crate::database::DbState;
use crate::services::settings;
use serde_json::Value;
use tauri::State;

#[tauri::command]
pub fn settings_get(db: State<'_, DbState>, key: String) -> Result<Option<Value>, String> {
    settings::get(&db, &key).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn settings_set(db: State<'_, DbState>, key: String, value: Value) -> Result<(), String> {
    settings::set(&db, &key, value).map_err(|e| e.to_command_error())
}
