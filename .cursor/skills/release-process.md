# Skill: release-process

Ship a versioned release.

## Steps

1. Bump versions in root package.json, apps/desktop/package.json, Cargo.toml, tauri.conf.json.
2. Update CHANGELOG.md.
3. Ensure CI is green.
4. Tag `vX.Y.Z` and push tags.
5. Verify Release workflow artifacts.
6. Publish GitHub release notes.

Never include AI co-author trailers in release commits.
