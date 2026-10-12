# Skill: database-migration

Add a SQLite migration.

## Steps

1. Create `src-tauri/src/database/migrations/NNNN_name.sql`.
2. Register it in `migrations.rs` `MIGRATIONS` array.
3. Keep migrations idempotent where possible (`IF NOT EXISTS`).
4. Update `docs/database.md`.
5. Add/adjust repository methods.
6. Run `cargo test --manifest-path src-tauri/Cargo.toml`.
