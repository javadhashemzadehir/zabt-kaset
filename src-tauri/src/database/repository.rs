use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadJob {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub source_type: String,
    pub state: String,
    pub priority: i32,
    pub preset_id: Option<String>,
    pub progress: f64,
    pub error_message: Option<String>,
    pub video_id: Option<String>,
    pub created_at: String,
}

pub struct Database;

impl Database {
    pub fn list_jobs(conn: &Connection, limit: i64) -> AppResult<Vec<DownloadJob>> {
        let mut stmt = conn.prepare(
            "SELECT id, url, title, source_type, state, priority, preset_id, progress,
                    error_message, video_id, created_at
             FROM download_jobs
             ORDER BY priority DESC, created_at DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], |row| {
            Ok(DownloadJob {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                source_type: row.get(3)?,
                state: row.get(4)?,
                priority: row.get(5)?,
                preset_id: row.get(6)?,
                progress: row.get(7)?,
                error_message: row.get(8)?,
                video_id: row.get(9)?,
                created_at: row.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn insert_job(
        conn: &Connection,
        id: &str,
        url: &str,
        source_type: &str,
        priority: i32,
        preset_id: Option<&str>,
    ) -> AppResult<()> {
        conn.execute(
            "INSERT INTO download_jobs (id, url, source_type, state, priority, preset_id)
             VALUES (?1, ?2, ?3, 'waiting', ?4, ?5)",
            params![id, url, source_type, priority, preset_id],
        )?;
        Ok(())
    }

    pub fn update_job_state(
        conn: &Connection,
        id: &str,
        state: &str,
        error: Option<&str>,
    ) -> AppResult<()> {
        conn.execute(
            "UPDATE download_jobs
             SET state = ?2,
                 error_message = COALESCE(?3, error_message),
                 finished_at = CASE WHEN ?2 IN ('completed','failed','cancelled')
                                    THEN datetime('now') ELSE finished_at END
             WHERE id = ?1",
            params![id, state, error],
        )?;
        Ok(())
    }

    pub fn search_history(
        conn: &Connection,
        query: &str,
        limit: i64,
    ) -> AppResult<Vec<DownloadJob>> {
        let like = format!("%{query}%");
        let mut stmt = conn.prepare(
            "SELECT id, url, title, source_type, state, priority, preset_id, progress,
                    error_message, video_id, created_at
             FROM download_jobs
             WHERE state IN ('completed','failed','cancelled')
               AND (IFNULL(title,'') LIKE ?1 OR url LIKE ?1 OR IFNULL(video_id,'') LIKE ?1)
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![like, limit], |row| {
            Ok(DownloadJob {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                source_type: row.get(3)?,
                state: row.get(4)?,
                priority: row.get(5)?,
                preset_id: row.get(6)?,
                progress: row.get(7)?,
                error_message: row.get(8)?,
                video_id: row.get(9)?,
                created_at: row.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn get_setting(conn: &Connection, key: &str) -> AppResult<Option<String>> {
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let mut rows = stmt.query([key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(conn: &Connection, key: &str, value: &str) -> AppResult<()> {
        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
            params![key, value],
        )?;
        Ok(())
    }
}
