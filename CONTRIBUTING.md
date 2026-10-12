# Contributing to Zabt Kaset

Thank you for contributing. This guide keeps the project consistent and
reviewable.

## Getting started

1. Fork and clone the repository
2. Install [Node.js 20+](https://nodejs.org/), [pnpm 9+](https://pnpm.io/),
   [Rust](https://rustup.rs/), and [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
3. Install system tools used at runtime: `yt-dlp` and `ffmpeg` (or use the
   in-app tool manager once the app runs)
4. From the repository root:

```bash
pnpm install
pnpm tauri:dev
```

See [docs/development.md](docs/development.md) for details.

## Branching

- `main` — stable development branch
- `feat/<short-name>` — features
- `fix/<short-name>` — bug fixes
- `docs/<short-name>` — documentation only
- `chore/<short-name>` — tooling / CI

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add queue priority reordering
fix: handle yt-dlp timeout errors
docs: improve architecture guide
chore: bump tauri to 2.2
test: cover duplicate detector hashing
```

Rules:

- Write messages as a human would — clear, imperative, scoped
- Do **not** add AI tool attribution trailers
- Do **not** add `Co-authored-by: Cursor` or similar tool signatures
- Keep the subject under ~72 characters

## Pull requests

- Keep PRs focused; prefer small reviewable units
- Update docs when behavior or public APIs change
- Add or update tests for logic changes
- Ensure CI passes: TypeScript, ESLint, Rust check, tests
- Fill in the PR template checklist

## Code standards

- **Frontend**: TypeScript strict, React Server/Client boundaries respected,
  Zod for runtime validation, Zustand for client state, TanStack Query for
  async server/IPC data
- **Backend**: idiomatic Rust, `thiserror` for typed errors, no `unwrap()` in
  production paths without justification, Tokio for async
- **Security**: validate all IPC inputs; never shell-interpolate URLs or paths

See [AGENTS.md](AGENTS.md) for naming conventions and agent-oriented guidance.

## Testing

```bash
pnpm typecheck
pnpm lint
pnpm test
pnpm rust:check
pnpm rust:test
pnpm rust:clippy
```

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.
