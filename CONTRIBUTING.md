# Contributing to proofdown_renderer

- Discuss significant changes with maintainers before implementation.
- Update `.specs/` when changing behavior; tests first.
- Keep code deterministic and `clippy -D warnings` clean.
- Add unit tests and goldens for new components/attrs.

## Development

```bash
cargo fmt --all
cargo clippy --all -- -D warnings
cargo test
```
