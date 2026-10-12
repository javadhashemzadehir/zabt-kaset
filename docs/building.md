# Building

## Development build

```bash
pnpm tauri:dev
```

## Production build

```bash
pnpm tauri:build
```

This:

1. Builds the Next.js static export (`apps/desktop/out`)
2. Compiles the Rust binary
3. Packages installers under `src-tauri/target/release/bundle/`

## Platform notes

### Windows

- Needs WebView2 (usually preinstalled on Windows 10/11)
- NSIS / MSI outputs depending on config

### macOS

- Requires Xcode CLT
- Universal builds need both targets configured

### Linux

- Install WebKitGTK and related Tauri deps for your distro
- AppImage / deb depending on bundle settings

## Environment

Do not bake secrets into the client. Signing credentials belong in CI secrets
only.
