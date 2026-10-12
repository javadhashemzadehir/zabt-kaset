use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppNotification {
    pub title: String,
    pub body: String,
}

pub fn download_completed(title: &str) -> AppNotification {
    AppNotification {
        title: "Download complete".into(),
        body: title.to_string(),
    }
}

pub fn download_failed(title: &str, reason: &str) -> AppNotification {
    AppNotification {
        title: "Download failed".into(),
        body: format!("{title}: {reason}"),
    }
}
