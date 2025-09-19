#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all
cargo clippy --all -- -D warnings
cargo test --all

# Optional: wasm and mdbook steps to be added later
