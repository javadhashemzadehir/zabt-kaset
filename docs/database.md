# Database

## Engine

SQLite via Rust (`rusqlite` + migrations). One file per user profile in the
application data directory.

## Migrations

Location: `src-tauri/src/database/migrations/`

Naming: `NNNN_description.sql` (e.g. `0001_init.sql`)

Applied in lexicographic order at startup inside a transaction. Schema version
tracked in `schema_migrations`.

## Core tables

| Table | Purpose |
| ----- | ------- |
| `download_jobs` | Queue + history of download jobs |
| `media_files` | Produced files linked to jobs |
| `channels` | Tracked channels + sync cursors |
| `playlists` | Known playlists |
| `duplicates` | Hash / ID index for dedupe |
| `presets` | Built-in and user presets |
| `proxy_profiles` | Proxy configurations |
| `settings` | Key/value settings |
| `cookie_profiles` | Metadata for stored cookie vault entries |

## Job states

`waiting` → `analyzing` → `downloading` → `processing` → `completed`  
Any active state may transition to `failed` (retryable) or `cancelled`.

## Backup

Copy the SQLite file while the app is closed, or use online backup API in a
future maintenance command.
