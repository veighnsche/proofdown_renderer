# Workspace Plan (renderer)

Initial layout is a single crate. Optionally evolve into a nested workspace mirroring the parser:

- `crates/proofdown_renderer` — HTML renderer (this crate)
- `crates/proofdown_renderer_wasm` — wasm-bindgen bindings (future)

Docs live under `.specs/`, `.docs/`, and `book/`.
