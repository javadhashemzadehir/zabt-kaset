# Development

## Prerequisites

- Node.js 20+
- pnpm 9+ (`corepack enable` recommended)
- Rust stable + Cargo
- Platform-specific [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
- Optional: yt-dlp, ffmpeg on `PATH`

## Setup

```bash
git clone https://github.com/javadhashemzadehir/zabt-kaset.git
cd zabt-kaset
cp .env.example .env
pnpm install
```

## Run

```bash
# Web UI only (limited — no Tauri APIs)
pnpm dev

# Full desktop stack
pnpm tauri:dev
```

## Quality gates

```bash
pnpm typecheck
pnpm lint
pnpm test
pnpm rust:fmt
pnpm rust:clippy
pnpm rust:test
```

## Frontend notes

- Package: `@zabt-kaset/desktop` in `apps/desktop`
- Path alias `@/` → `apps/desktop`
- shadcn components live in `components/ui`
- Add features under `features/<domain>`

## Backend notes

- Entrypoint: `src-tauri/src/lib.rs` / `main.rs`
- Register commands in `lib.rs`
- Add SQL migrations under `src-tauri/src/database/migrations/`

## Debugging IPC

1. Confirm command name matches exactly (`snake_case`)
2. Log on Rust side with `tracing`
3. Validate payload with Zod before `invoke`
4. Check Tauri capability allows the command

## Local agent files

Put personal prompts and scratch notes in `.cursor/private/` or
`.agents/private/` — both are gitignored and must never be committed.
