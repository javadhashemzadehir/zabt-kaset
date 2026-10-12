# Troubleshooting

## App won't start

- Confirm Rust and Tauri OS prerequisites are installed
- Delete `apps/desktop/.next` and re-run `pnpm tauri:dev`
- Check `src-tauri/tauri.conf.json` `build.devUrl` matches Next dev port

## yt-dlp not found

- Install yt-dlp and ensure it is on `PATH`
- Or set an absolute path in Settings → Tools
- Use `tools_status` / Tools page to verify detection

## FFmpeg errors during merge

- Verify `ffmpeg -version` works in a terminal
- Prefer full builds with common codecs
- Check the job log for the exact ffmpeg argv

## Downloads fail with 403 / age restriction

- Import cookies from a logged-in browser profile
- Confirm cookie profile is selected on the job
- Re-export cookies if the session expired

## Persian UI direction wrong

- Ensure locale is `fa` and root `dir="rtl"`
- Prefer logical Tailwind classes (`ps-4` not `pl-4`)

## Database locked

- Only one app instance should open the DB
- Ensure previous process exited cleanly
- Avoid opening the DB file with external writers while running

## CI failures

- Run local gates: `pnpm typecheck && pnpm lint && pnpm rust:clippy`
- Ensure `Cargo.lock` and `pnpm-lock.yaml` are committed when present
