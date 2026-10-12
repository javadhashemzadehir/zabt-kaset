# Security

## Threat model (summary)

| Asset | Risk | Mitigation |
| ----- | ---- | ---------- |
| Filesystem | Path traversal / overwrite | Canonicalize; confine to download root |
| Process spawn | Shell injection | Argv arrays only; fixed binary paths |
| Cookies | Credential theft | OS keychain / encrypted store; never log |
| Network | SSRF / malicious URLs | Allowlist schemes (`http`, `https`); length limits |
| Webview | IPC abuse | Validate inputs; minimal capabilities |

## Command hardening

```rust
// Good
Command::new(&yt_dlp_path).args(["-f", &format, "--", &url])

// Bad — never
Command::new("sh").arg("-c").arg(format!("yt-dlp {url}"))
```

## Path rules

1. Resolve with `dunce` / `canonicalize`
2. Ensure result starts with configured download root
3. Reject NUL and control characters in path segments
4. Template variables are sanitized (strip `../`, path separators)

## Tauri capabilities

Grant only required permissions. Prefer scoped FS APIs. Do not enable
arbitrary shell unless wrapped behind validated commands (preferred: no shell
plugin).

## Reporting

See [SECURITY.md](../SECURITY.md).
