# Hook: pre-commit

Before committing:

1. `git status`
2. `git diff`
3. `git log -5 --oneline`
4. Ensure no secrets, cookies, `*.db`, or `.cursor/private/` files
5. Ensure commit message is Conventional Commit style
6. Strip any `Co-authored-by: Cursor` or AI attribution trailers
7. Prefer `pnpm typecheck` / `pnpm rust:check` for touched areas
