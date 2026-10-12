use crate::error::{AppError, AppResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizerContext {
    pub channel: Option<String>,
    pub playlist: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub year: Option<String>,
    pub resolution: Option<String>,
    pub format: Option<String>,
    pub ext: Option<String>,
}

pub fn expand_template(template: &str, ctx: &OrganizerContext) -> AppResult<String> {
    if template.is_empty() || template.len() > 512 {
        return Err(AppError::validation("template length invalid"));
    }
    if template.contains("..") {
        return Err(AppError::validation("template must not contain '..'"));
    }

    let mut out = template.to_string();
    let replacements = [
        (
            "{channel}",
            sanitize_segment(ctx.channel.as_deref().unwrap_or("Unknown")),
        ),
        (
            "{playlist}",
            sanitize_segment(ctx.playlist.as_deref().unwrap_or("Playlist")),
        ),
        (
            "{title}",
            sanitize_segment(ctx.title.as_deref().unwrap_or("Untitled")),
        ),
        (
            "{date}",
            sanitize_segment(ctx.date.as_deref().unwrap_or("unknown-date")),
        ),
        (
            "{year}",
            sanitize_segment(ctx.year.as_deref().unwrap_or("0000")),
        ),
        (
            "{resolution}",
            sanitize_segment(ctx.resolution.as_deref().unwrap_or("unknown")),
        ),
        (
            "{format}",
            sanitize_segment(ctx.format.as_deref().unwrap_or("media")),
        ),
        (
            "{ext}",
            sanitize_segment(ctx.ext.as_deref().unwrap_or("mp4")),
        ),
    ];

    for (key, value) in replacements {
        out = out.replace(key, &value);
    }

    if Regex::new(r"\{[a-zA-Z_]+\}").unwrap().is_match(&out) {
        return Err(AppError::validation("unknown template variable"));
    }

    Ok(out)
}

pub fn join_under_root(root: &Path, relative: &str) -> AppResult<PathBuf> {
    let root = dunce::canonicalize(root).map_err(|e| AppError::Path(e.to_string()))?;
    let candidate = root.join(relative);
    let normalized = normalize_relative(Path::new(relative))?;
    let full = root.join(normalized);
    let full_str = full.to_string_lossy();
    let root_str = root.to_string_lossy();
    if !full_str.starts_with(root_str.as_ref()) {
        return Err(AppError::Path("path escapes download root".into()));
    }
    let _ = candidate;
    Ok(full)
}

fn normalize_relative(path: &Path) -> AppResult<PathBuf> {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::Normal(seg) => out.push(seg),
            Component::CurDir => {}
            _ => return Err(AppError::Path("invalid path component".into())),
        }
    }
    Ok(out)
}

fn sanitize_segment(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    let trimmed = cleaned.trim().trim_matches('.');
    if trimmed.is_empty() {
        "untitled".into()
    } else {
        trimmed.chars().take(120).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_channel_title() {
        let ctx = OrganizerContext {
            channel: Some("Veritasium".into()),
            title: Some("Spinning Black Holes".into()),
            ext: Some("mp4".into()),
            ..Default::default()
        };
        let path = expand_template("YouTube/{channel}/Videos/{title}.{ext}", &ctx).unwrap();
        assert_eq!(path, "YouTube/Veritasium/Videos/Spinning Black Holes.mp4");
    }

    #[test]
    fn rejects_traversal_in_template() {
        assert!(expand_template("../{title}", &OrganizerContext::default()).is_err());
    }
}
