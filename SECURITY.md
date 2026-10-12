# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |

## Reporting a Vulnerability

Do **not** open a public GitHub issue for security vulnerabilities.

Please report security issues privately by emailing the maintainers or using
GitHub Security Advisories for this repository.

Include:

- A clear description of the issue
- Steps to reproduce
- Affected versions / commit hashes
- Impact assessment if known

We aim to acknowledge reports within 72 hours and provide a remediation plan
within 14 days for confirmed issues.

## Security Principles

This application executes external tools (`yt-dlp`, `ffmpeg`) and handles user
paths, cookies, and network proxies. Contributors must:

- Never pass unsanitized user input to shell commands
- Prefer argument arrays over shell string interpolation
- Validate and canonicalize all filesystem paths
- Reject path traversal (`..`, absolute escapes outside download roots)
- Store cookies and secrets via OS keychain / encrypted storage
- Keep Tauri capabilities minimal (least privilege)
- Treat download URLs and metadata as untrusted input

See [docs/security.md](docs/security.md) for the full threat model and
hardening checklist.
