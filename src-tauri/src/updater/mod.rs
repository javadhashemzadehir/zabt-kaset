use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    pub name: String,
    pub found: bool,
    pub path: Option<String>,
    pub version: Option<String>,
}

pub async fn which_tool(candidates: &[&str]) -> Option<PathBuf> {
    for name in candidates {
        if let Ok(output) = Command::new(name)
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
        {
            if output.success() || output.code().is_some() {
                return Some(PathBuf::from(name));
            }
        }
        // yt-dlp uses --version
        if let Ok(output) = Command::new(name)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
        {
            if output.success() {
                return Some(PathBuf::from(name));
            }
        }
    }
    None
}

pub async fn read_version(bin: &Path, flag: &str) -> AppResult<String> {
    let output = Command::new(bin)
        .arg(flag)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| AppError::Process(e.to_string()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let text = if stdout.trim().is_empty() {
        stderr
    } else {
        stdout
    };
    let line = text.lines().next().unwrap_or("").trim();
    if line.is_empty() {
        Err(AppError::Process(format!(
            "unable to read version for {}",
            bin.display()
        )))
    } else {
        Ok(line.to_string())
    }
}

pub async fn yt_dlp_status(configured: Option<PathBuf>) -> ToolStatus {
    let path = if let Some(p) = configured {
        Some(p)
    } else {
        which_tool(&["yt-dlp", "yt-dlp.exe"]).await
    };
    match path {
        Some(p) => match read_version(&p, "--version").await {
            Ok(version) => ToolStatus {
                name: "yt-dlp".into(),
                found: true,
                path: Some(p.display().to_string()),
                version: Some(version),
            },
            Err(_) => ToolStatus {
                name: "yt-dlp".into(),
                found: false,
                path: Some(p.display().to_string()),
                version: None,
            },
        },
        None => ToolStatus {
            name: "yt-dlp".into(),
            found: false,
            path: None,
            version: None,
        },
    }
}

pub async fn ffmpeg_status(configured: Option<PathBuf>) -> ToolStatus {
    let path = if let Some(p) = configured {
        Some(p)
    } else {
        which_tool(&["ffmpeg", "ffmpeg.exe"]).await
    };
    match path {
        Some(p) => match read_version(&p, "-version").await {
            Ok(version) => ToolStatus {
                name: "ffmpeg".into(),
                found: true,
                path: Some(p.display().to_string()),
                version: Some(version),
            },
            Err(_) => ToolStatus {
                name: "ffmpeg".into(),
                found: false,
                path: Some(p.display().to_string()),
                version: None,
            },
        },
        None => ToolStatus {
            name: "ffmpeg".into(),
            found: false,
            path: None,
            version: None,
        },
    }
}

/// Self-update yt-dlp via its built-in updater when available.
pub async fn update_yt_dlp(bin: &Path) -> AppResult<String> {
    let output = Command::new(bin)
        .args(["-U"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| AppError::Process(e.to_string()))?;
    let msg = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        Ok(msg.trim().to_string())
    } else {
        Err(AppError::Process(msg))
    }
}
