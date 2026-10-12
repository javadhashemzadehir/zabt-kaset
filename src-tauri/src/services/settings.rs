use crate::database::{Database, DbState};
use crate::error::AppResult;
use serde_json::Value;

pub fn get(db: &DbState, key: &str) -> AppResult<Option<Value>> {
    db.with_conn(|conn| {
        let raw = Database::get_setting(conn, key)?;
        Ok(raw.map(|s| serde_json::from_str(&s).unwrap_or(Value::String(s))))
    })
}

pub fn set(db: &DbState, key: &str, value: Value) -> AppResult<()> {
    let serialized = serde_json::to_string(&value).unwrap_or_else(|_| "null".into());
    db.with_conn(|conn| Database::set_setting(conn, key, &serialized))
}
