# Skill: create-tauri-command

Add a new Tauri IPC command.

## Steps

1. Define request/response types with `serde` + `camelCase`.
2. Implement domain logic in `services/` or a domain module.
3. Add a thin function in `src-tauri/src/commands/`.
4. Register in `src-tauri/src/lib.rs` invoke handler.
5. Wrap in `apps/desktop/lib/tauri.ts`.
6. Add Zod/types on the frontend if needed.
7. Document in `docs/architecture.md`.
8. Validate all inputs; return `Result<T, String>` via `AppError::to_command_error()`.
