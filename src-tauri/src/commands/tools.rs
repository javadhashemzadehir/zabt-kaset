use crate::services::tools::{self, ToolsSnapshot};
use crate::updater;
use std::path::PathBuf;

#[tauri::command]
pub async fn tools_status(
    yt_dlp_path: Option<String>,
    ffmpeg_path: Option<String>,
) -> Result<ToolsSnapshot, String> {
    Ok(tools::status(yt_dlp_path, ffmpeg_path).await)
}

#[tauri::command]
pub async fn tools_update_yt_dlp(path: Option<String>) -> Result<String, String> {
    let bin = PathBuf::from(path.unwrap_or_else(|| "yt-dlp".into()));
    updater::update_yt_dlp(&bin)
        .await
        .map_err(|e| e.to_command_error())
}
