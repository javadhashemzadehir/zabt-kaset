mod migrations;
mod repository;

pub use repository::{Database, DownloadJob};

use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct DbState {
    pub conn: Mutex<Connection>,
    pub path: PathBuf,
}

impl DbState {
    pub fn open(path: impl AsRef<Path>) -> AppResult<Self> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path.as_ref())?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;
        migrations::run(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            path: path.as_ref().to_path_buf(),
        })
    }

    pub fn with_conn<T>(&self, f: impl FnOnce(&Connection) -> AppResult<T>) -> AppResult<T> {
        let guard = self
            .conn
            .lock()
            .map_err(|_| AppError::Internal("database lock poisoned".into()))?;
        f(&guard)
    }
}

pub fn default_db_path() -> AppResult<PathBuf> {
    let base = dirs::data_dir().ok_or_else(|| AppError::Internal("no data dir".into()))?;
    Ok(base.join("zabet-kaset").join("zabet-kaset.db"))
}
