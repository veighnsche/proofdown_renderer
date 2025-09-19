# Releasing

1. Ensure `cargo fmt`, `clippy -D warnings`, and tests pass.
2. Update `CHANGELOG.md` (if present) and bump version in `Cargo.toml` if releasing the crate independently.
3. Tag release `vX.Y.Z` and push.
4. If publishing to crates.io (optional), run `cargo publish`.
