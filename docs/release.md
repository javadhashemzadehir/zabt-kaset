# Release process

## Versioning

Semantic Versioning. Bump together:

- `package.json` (root)
- `apps/desktop/package.json`
- `src-tauri/tauri.conf.json` (`version`)
- `src-tauri/Cargo.toml` (`version`)
- `CHANGELOG.md`

## Checklist

1. Update changelog under `[Unreleased]` → version section
2. Ensure CI is green on `main`
3. Tag: `git tag -a v0.x.y -m "v0.x.y"`
4. Push tag to trigger the Release workflow
5. Verify Windows / macOS / Linux artifacts
6. Publish GitHub Release notes from changelog

## Signing

Configure platform signing secrets in GitHub Actions before public releases:

- Windows: certificate + password
- macOS: Developer ID + notarization
- Linux: optional GPG

Until signing is configured, mark builds as **unsigned preview**.

## Hotfix

Branch from the release tag, bump patch version, cherry-pick fix, retag.
