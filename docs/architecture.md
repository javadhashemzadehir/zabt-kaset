# Architecture

## Overview

Zabt Kaset is a desktop application with a web frontend embedded in a Tauri
webview and a Rust backend that owns persistence, process orchestration, and
security-sensitive operations.

```
apps/desktop (Next.js)
        │  invoke / listen
        ▼
src-tauri commands
        │
        ▼
   services layer
   ├── downloader (yt-dlp)
   ├── organizer
   ├── duplicate
   ├── channel
   ├── metadata
   ├── proxy
   ├── updater (tools)
   ├── database (SQLite)
   └── notifications
```

## Frontend

- **Next.js 15 App Router** with `output: "export"` for Tauri bundling
- **Feature modules** under `features/` encapsulate screens and domain hooks
- **Zustand** for UI/session state (theme prefs mirror, queue selection, modals)
- **TanStack Query** for IPC reads/mutations with cache keys per domain
- **Zod** schemas mirror Rust DTOs for runtime validation at the boundary
- **next-intl** for English / Persian; document `dir` flips with locale
- **next-themes** for light / dark / system

Routing lives under `app/(app)/` for authenticated-shell style pages (local app
shell, not cloud auth).

## Tauri & IPC

Commands are thin adapters:

1. Deserialize / validate inputs
2. Call a service
3. Map errors to a stable string or structured error payload
4. Optionally emit events (`download://progress`, `queue://updated`)

Frontend access goes through `apps/desktop/lib/tauri.ts` — never scatter raw
`invoke` calls across components.

### Example commands

| Command | Purpose |
| ------- | ------- |
| `queue_list` | List jobs |
| `queue_add` | Enqueue URL(s) with preset |
| `queue_pause` / `queue_resume` / `queue_cancel` | Control |
| `history_search` | Query completed downloads |
| `settings_get` / `settings_set` | App settings |
| `tools_status` | yt-dlp / FFmpeg versions |
| `proxy_test` | Latency check |
| `channel_sync` | Incremental channel fetch |

## Rust backend

Domain modules own business rules:

- **downloader** — builds argv for yt-dlp, parses progress, maps exit codes
- **organizer** — template expansion and safe path joins
- **duplicate** — SHA-256 and ID matching
- **channel** — listing + incremental watermark
- **metadata** — sidecar JSON / description / thumbnails
- **proxy** — profile application to child env
- **updater** — tool binary detect/update
- **database** — migrations + repositories
- **notifications** — desktop notifications + tray hints

Tokio drives async work. Long jobs run as supervised tasks; UI receives events.

## SQLite

Single app database in the platform app-data directory. Migrations are numbered
SQL files applied at startup. See [database.md](database.md).

## Media engine

`yt-dlp` performs network download and format selection. `ffmpeg` merges,
remuxes, extracts audio, and embeds subtitles. Both are treated as external
trusted binaries located via settings or `PATH`, never via user-controlled
relative paths without validation.

## Security boundaries

- Webview cannot spawn arbitrary processes
- All FS writes go through Rust after root checks
- Cookie material never logged
- Capabilities in `src-tauri/capabilities/` are least privilege

See [security.md](security.md).
