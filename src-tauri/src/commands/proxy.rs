use crate::proxy::{self, ProxyProfile, ProxyTestResult};

#[tauri::command]
pub fn proxy_test(profile: ProxyProfile) -> Result<ProxyTestResult, String> {
    proxy::test_profile(&profile).map_err(|e| e.to_command_error())
}

#[tauri::command]
pub fn proxy_to_url(profile: ProxyProfile) -> Result<String, String> {
    profile.to_url().map_err(|e| e.to_command_error())
}
