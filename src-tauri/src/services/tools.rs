use crate::updater::{self, ToolStatus};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsSnapshot {
    pub yt_dlp: ToolStatus,
    pub ffmpeg: ToolStatus,
}

pub async fn status(yt_dlp_path: Option<String>, ffmpeg_path: Option<String>) -> ToolsSnapshot {
    let yt = updater::yt_dlp_status(yt_dlp_path.map(PathBuf::from)).await;
    let ff = updater::ffmpeg_status(ffmpeg_path.map(PathBuf::from)).await;
    ToolsSnapshot {
        yt_dlp: yt,
        ffmpeg: ff,
    }
}
