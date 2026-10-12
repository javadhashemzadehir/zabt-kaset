use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub url: String,
    pub output_template: String,
    pub format: Option<String>,
    pub audio_only: bool,
    pub audio_format: Option<String>,
    pub subtitle_langs: Vec<String>,
    pub embed_subtitles: bool,
    pub write_thumbnail: bool,
    pub write_description: bool,
    pub write_info_json: bool,
    pub proxy: Option<String>,
    pub cookies_from_browser: Option<String>,
    pub yt_dlp_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub job_id: String,
    pub percent: f64,
    pub speed: Option<String>,
    pub eta: Option<String>,
    pub status: String,
}

pub fn validate_url(raw: &str) -> AppResult<url::Url> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > 2048 {
        return Err(AppError::validation("url length invalid"));
    }
    let parsed = url::Url::parse(trimmed).map_err(|e| AppError::validation(e.to_string()))?;
    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        other => Err(AppError::validation(format!("unsupported scheme: {other}"))),
    }
}

pub fn build_args(opts: &DownloadOptions) -> AppResult<Vec<String>> {
    let url = validate_url(&opts.url)?;
    let mut args = vec![
        "--newline".into(),
        "--no-colors".into(),
        "--ignore-config".into(),
        "-o".into(),
        opts.output_template.clone(),
    ];

    if opts.audio_only {
        args.push("-x".into());
        if let Some(fmt) = &opts.audio_format {
            args.push("--audio-format".into());
            args.push(sanitize_token(fmt)?);
        }
    } else if let Some(fmt) = &opts.format {
        args.push("-f".into());
        args.push(fmt.clone());
    }

    if !opts.subtitle_langs.is_empty() {
        args.push("--write-subs".into());
        args.push("--sub-langs".into());
        args.push(opts.subtitle_langs.join(","));
        if opts.embed_subtitles {
            args.push("--embed-subs".into());
        }
    }

    if opts.write_thumbnail {
        args.push("--write-thumbnail".into());
    }
    if opts.write_description {
        args.push("--write-description".into());
    }
    if opts.write_info_json {
        args.push("--write-info-json".into());
    }
    if let Some(proxy) = &opts.proxy {
        args.push("--proxy".into());
        args.push(proxy.clone());
    }
    if let Some(browser) = &opts.cookies_from_browser {
        let browser = sanitize_browser(browser)?;
        args.push("--cookies-from-browser".into());
        args.push(browser);
    }

    args.push("--".into());
    args.push(url.as_str().to_string());
    Ok(args)
}

fn sanitize_token(value: &str) -> AppResult<String> {
    if value.is_empty() || value.len() > 32 || !value.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AppError::validation("invalid format token"));
    }
    Ok(value.to_ascii_lowercase())
}

fn sanitize_browser(value: &str) -> AppResult<String> {
    match value.to_ascii_lowercase().as_str() {
        "chrome" | "edge" | "firefox" | "brave" => Ok(value.to_ascii_lowercase()),
        _ => Err(AppError::validation("unsupported browser")),
    }
}

pub async fn probe_version(bin: &Path) -> AppResult<String> {
    let output = Command::new(bin)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| AppError::Process(e.to_string()))?;

    if !output.status.success() {
        return Err(AppError::Process("yt-dlp --version failed".into()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_javascript_url() {
        assert!(validate_url("javascript:alert(1)").is_err());
    }

    #[test]
    fn builds_safe_args() {
        let opts = DownloadOptions {
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".into(),
            output_template: "%(title)s.%(ext)s".into(),
            format: Some("bv*+ba/b".into()),
            audio_only: false,
            audio_format: None,
            subtitle_langs: vec!["en".into(), "fa".into()],
            embed_subtitles: true,
            write_thumbnail: true,
            write_description: true,
            write_info_json: true,
            proxy: None,
            cookies_from_browser: Some("chrome".into()),
            yt_dlp_path: PathBuf::from("yt-dlp"),
        };
        let args = build_args(&opts).unwrap();
        assert!(args.contains(&"--".to_string()));
        assert_eq!(args.last().unwrap().as_str(), opts.url);
    }
}
