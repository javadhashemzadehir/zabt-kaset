# AGENTS.md — Zabet Kaset

Instructions for human contributors and AI coding agents working in this
repository.

## Project identity

**Zabet Kaset** is a Tauri v2 + Next.js desktop media manager. Treat it as a
production open-source product: prefer maintainability, security, and clear
boundaries over clever shortcuts.

## Architecture map

| Area | Path | Notes |
| ---- | ---- | ----- |
| Frontend app | `apps/desktop/` | Next.js App Router, static export for Tauri |
| UI primitives | `apps/desktop/components/ui/` | shadcn/Radix only |
| Feature modules | `apps/desktop/features/` | Domain UI + hooks colocated |
| Client state | `apps/desktop/stores/` | Zustand |
| Shared lib | `apps/desktop/lib/` | IPC wrappers, utils, validators |
| Locales | `apps/desktop/locales/{en,fa}/` | next-intl messages |
| Tauri/Rust | `src-tauri/src/` | Commands thin; logic in services/modules |
| Database | `src-tauri/src/database/` | SQLite + SQL migrations |
| Docs | `docs/` | Source of truth for architecture |

IPC flow: **UI → `lib/tauri.ts` → Tauri command → service → SQLite / yt-dlp**.

## Coding standards

### TypeScript / React

- Strict TypeScript; no `any` unless justified and narrowly scoped
- Prefer server components only when they add value; most desktop UI is client
- Validate IPC payloads with Zod on the frontend boundary
- Use TanStack Query for async IPC reads; Zustand for ephemeral UI state
- Feature folders own their components; keep `components/ui` generic
- Support RTL: use logical CSS (`ps-`, `pe-`, `ms-`, `me-`) not left/right

### Rust

- Modules: `commands/` (IPC), `services/` (orchestration), domain crates-as-mods
- Errors: `thiserror` types; convert to `Result<T, String>` (or structured
  error) at the command boundary
- Async: Tokio; never block the main thread on network or process I/O
- No shell string concatenation — always `Command::new(bin).args([...])`
- Paths: canonicalize and ensure they remain under configured roots

### Naming

- Files: `kebab-case.ts`, `PascalCase.tsx` for components, `snake_case.rs`
- Types: `PascalCase`; constants `SCREAMING_SNAKE`
- Tauri commands: `snake_case` (e.g. `queue_add`, `tools_yt_dlp_version`)
- DB tables: plural `snake_case` (`download_jobs`, `media_files`)

## Testing rules

- Unit-test pure logic (organizer templates, duplicate hashing, URL parsing)
- Rust: `cargo test` for database and domain modules
- Frontend: Vitest for stores/utils; keep UI tests focused
- Do not hit real YouTube in CI — mock process runners
- New commands need at least one happy-path and one validation-failure test when feasible

## Commit rules

- Conventional Commits: `feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`
- Human-written subjects; no tool branding in messages
- **Never** add `Co-authored-by: Cursor` or AI attribution trailers
- Before commit: `git status`, `git diff`, `git log -5` — scrub accidental junk
- Do not commit secrets, cookies, local DBs, or `.cursor/private/`

## Documentation rules

- User-facing behavior changes → update `README.md` and relevant `docs/*`
- New IPC commands → document in `docs/architecture.md` (IPC section)
- Schema changes → `docs/database.md` + a numbered SQL migration
- Keep docs accurate; prefer short concrete examples over essays

## Security rules

- Validate every command argument (URL scheme, path, enum, length)
- Deny path traversal; resolve symlinks carefully
- Cookies/secrets → encrypted store / OS keychain APIs only
- Least-privilege Tauri capabilities; no broad FS/shell scopes without need
- See `docs/security.md` and `.cursor/rules/security.mdc`

## What not to touch casually

- `LICENSE`, Code of Conduct, and security policy wording without maintainer review
- Release signing secrets and GitHub Actions deploy keys
- Public `.cursor/rules` — keep them generic; put personal prompts in
  `.cursor/private/` (gitignored)

## Agent hygiene

- Public agent docs: `AGENTS.md`, `.cursor/rules`, `.cursor/skills`, `.agents/`
- Private/local only: `.cursor/private/`, `.agents/private/` (ignored)
- Do not write Cursor brand metadata into the product UI or commits
- Prefer editing existing patterns over introducing parallel abstractions
