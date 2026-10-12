#![allow(dead_code)]

mod channel;
mod commands;
mod database;
mod downloader;
mod duplicate;
mod error;
mod metadata;
mod notifications;
mod organizer;
mod proxy;
mod services;
mod updater;

use database::{default_db_path, DbState};
use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("zabet_kaset=info".parse().unwrap()),
        )
        .init();

    let db_path = default_db_path().expect("resolve database path");
    let db = DbState::open(&db_path).expect("open database");
    tracing::info!(path = %db_path.display(), "database ready");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::queue::queue_list,
            commands::queue::queue_add,
            commands::queue::queue_pause,
            commands::queue::queue_resume,
            commands::queue::queue_cancel,
            commands::queue::queue_retry,
            commands::queue::queue_preview_args,
            commands::history::history_search,
            commands::settings::settings_get,
            commands::settings::settings_set,
            commands::tools::tools_status,
            commands::tools::tools_update_yt_dlp,
            commands::proxy::proxy_test,
            commands::proxy::proxy_to_url,
            commands::channel::channel_plan_sync,
            commands::organizer::organizer_expand,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
