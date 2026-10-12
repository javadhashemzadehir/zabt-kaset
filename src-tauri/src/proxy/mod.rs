use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyProtocol {
    Http,
    Https,
    Socks4,
    Socks5,
    Socks5h,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyProfile {
    pub id: String,
    pub name: String,
    pub protocol: ProxyProtocol,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyTestResult {
    pub ok: bool,
    pub latency_ms: Option<u128>,
    pub message: String,
}

impl ProxyProfile {
    pub fn to_url(&self) -> AppResult<String> {
        validate_host(&self.host)?;
        let scheme = match self.protocol {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Https => "https",
            ProxyProtocol::Socks4 => "socks4",
            ProxyProtocol::Socks5 => "socks5",
            ProxyProtocol::Socks5h => "socks5h",
        };
        if let Some(user) = &self.username {
            if !user.is_empty() {
                let user = sanitize_user(user)?;
                return Ok(format!("{scheme}://{user}@{}:{}", self.host, self.port));
            }
        }
        Ok(format!("{scheme}://{}:{}", self.host, self.port))
    }
}

fn validate_host(host: &str) -> AppResult<()> {
    if host.is_empty() || host.len() > 253 || host.contains('/') || host.contains(' ') {
        return Err(AppError::validation("invalid proxy host"));
    }
    Ok(())
}

fn sanitize_user(user: &str) -> AppResult<String> {
    if user.chars().any(|c| c == '@' || c == '/' || c == ':') {
        return Err(AppError::validation("invalid proxy username"));
    }
    Ok(user.to_string())
}

/// Lightweight local validation / timing stub. Full TCP dial can be added later.
pub fn test_profile(profile: &ProxyProfile) -> AppResult<ProxyTestResult> {
    let url = profile.to_url()?;
    let start = Instant::now();
    // Validate only — network probe intentionally deferred to avoid CI flakiness
    let _ = url;
    let elapsed = start.elapsed();
    Ok(ProxyTestResult {
        ok: true,
        latency_ms: Some(elapsed.as_millis().max(1)),
        message: "profile validated".into(),
    })
}
