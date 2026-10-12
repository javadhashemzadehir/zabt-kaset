use crate::downloader::validate_url;
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSyncRequest {
    pub url: String,
    pub include_shorts: bool,
    pub include_lives: bool,
    pub keyword_filter: Option<String>,
    pub date_after: Option<String>,
    pub date_before: Option<String>,
    pub since_video_id: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSyncPlan {
    pub normalized_url: String,
    pub yt_dlp_args: Vec<String>,
}

pub fn plan_sync(req: &ChannelSyncRequest) -> AppResult<ChannelSyncPlan> {
    let url = validate_url(&req.url)?;
    let host = url.host_str().unwrap_or_default();
    if !host.contains("youtube.com") && host != "youtu.be" {
        // Still allow generic channel URLs supported by yt-dlp, but warn via validation length
        if url.path().is_empty() {
            return Err(AppError::validation("channel url path required"));
        }
    }

    let mut args = vec![
        "--flat-playlist".into(),
        "--dump-json".into(),
        "--ignore-config".into(),
    ];

    if let Some(limit) = req.limit {
        args.push("--playlist-end".into());
        args.push(limit.min(5000).to_string());
    }
    if let Some(after) = &req.date_after {
        args.push("--dateafter".into());
        args.push(sanitize_date(after)?);
    }
    if let Some(before) = &req.date_before {
        args.push("--datebefore".into());
        args.push(sanitize_date(before)?);
    }
    if let Some(kw) = &req.keyword_filter {
        let kw = sanitize_match(kw)?;
        args.push("--match-title".into());
        args.push(kw);
    }
    if !req.include_shorts {
        args.push("--match-filter".into());
        args.push("!original_url*=/shorts/".into());
    }
    if !req.include_lives {
        args.push("--match-filter".into());
        args.push("!is_live".into());
    }

    args.push("--".into());
    args.push(url.as_str().to_string());

    Ok(ChannelSyncPlan {
        normalized_url: url.as_str().to_string(),
        yt_dlp_args: args,
    })
}

fn sanitize_date(value: &str) -> AppResult<String> {
    if value.len() == 8 && value.chars().all(|c| c.is_ascii_digit()) {
        Ok(value.to_string())
    } else {
        Err(AppError::validation("date must be YYYYMMDD"))
    }
}

fn sanitize_match(value: &str) -> AppResult<String> {
    if value.is_empty() || value.len() > 128 {
        return Err(AppError::validation("keyword filter invalid"));
    }
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_channel_args() {
        let plan = plan_sync(&ChannelSyncRequest {
            url: "https://www.youtube.com/@Veritasium".into(),
            include_shorts: false,
            include_lives: false,
            keyword_filter: Some("black hole".into()),
            date_after: Some("20240101".into()),
            date_before: None,
            since_video_id: None,
            limit: Some(50),
        })
        .unwrap();
        assert!(plan.yt_dlp_args.contains(&"--playlist-end".to_string()));
    }
}
