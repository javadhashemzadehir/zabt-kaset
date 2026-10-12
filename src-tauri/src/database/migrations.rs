use crate::error::{AppError, AppResult};
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[("0001_init", include_str!("migrations/0001_init.sql"))];

pub fn run(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY NOT NULL,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )?;

    for (version, sql) in MIGRATIONS {
        let already: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?1)",
            [version],
            |row| row.get(0),
        )?;

        if already {
            continue;
        }

        conn.execute_batch("BEGIN;")?;
        match conn.execute_batch(sql) {
            Ok(()) => {
                conn.execute(
                    "INSERT INTO schema_migrations (version) VALUES (?1)",
                    [version],
                )?;
                conn.execute_batch("COMMIT;")?;
                tracing::info!(version, "applied migration");
            }
            Err(err) => {
                let _ = conn.execute_batch("ROLLBACK;");
                return Err(AppError::Database(err));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn applies_init_migration() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let conn = Connection::open(path).unwrap();
        run(&conn).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM presets", [], |r| r.get(0))
            .unwrap();
        assert!(count >= 5);
    }
}
