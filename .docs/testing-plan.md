# Testing Plan — proofdown_renderer (v1)

This plan defines comprehensive tests and procedures to ensure the renderer is deterministic, safe, and correct.

## Test Tiers

- **[Unit tests (emitters)]**
  - Per-node suites for blocks/inlines/components.
  - Structural components: `grid`, `card`.
  - Artifact components: `artifact.summary`, `artifact.table`, `artifact.json`, `artifact.markdown`, `artifact.image`, `artifact.link`.
  - Attribute validation: known keys only; types/ranges; stable attribute ordering.

- **[Sanitization tests]**
  - Escaping of special characters `&<>"'` in text.
  - Attribute value escaping.
  - Negative cases: attempts to inject HTML/script; verify escape.

- **[Golden HTML tests]**
  - For representative AST inputs, render → compare byte-equal to files in `tests/golden/`.
  - Deterministic newlines `\n` and attribute order.
  - Harness supports `UPDATE_GOLDEN=1` to regenerate outputs.

- **[Determinism checks]**
  - Render the same AST twice; compare bytes.
  - Traverse maps/sets with fixed order; ensure stable ids.

- **[Limits and error-path tests]**
  - Depth limit exceeded → `RenderError::DepthExceeded`.
  - Unknown component/attribute → `RenderError::UnknownComponent/InvalidAttr`.
  - Oversized input (node count) → error.

- **[Integration tests (with parser, offline)]**
  - Parse sample `.pml` via `proofdown_parser` → `Document` → render via this crate.
  - Stub out artifact viewers via a trait injection (see below) for deterministic outputs.

- **[WASM tests (future)]**
  - `wasm-bindgen-test` for `wasm_to_html` JSON surface.
  - Byte-equal outputs vs native tests.

- **[Fuzzing (optional)]**
  - `cargo-fuzz` targets for attribute normalization and renderer functions (guarded with strict limits).

## Test Harness Conventions

- **[Directories]**
  - `tests/fixtures/` — input AST JSON files or `.pml` (when parser used).
  - `tests/golden/` — expected HTML outputs.

- **[Case format]**
  - Case folder: `tests/cases/<name>/` containing:
    - `input.pml` (preferred) or `input.json` (AST JSON)
    - `expected.html` on success or `expected_error.json` on failure.

- **[Harness behavior]**
  - Discover cases at runtime.
  - If `UPDATE_GOLDEN=1`, overwrite `expected.html` when rendering succeeds; otherwise compare.
  - Normalize newlines to `\n` and compare byte-equal.

## Artifact Viewer Injection (for tests)

- **[Trait]**
  - Define `ArtifactViewProvider` trait for test-time substitution:
    ```rust
    pub trait ArtifactViewProvider {
        fn render(&self, kind: &str, id: &str) -> Result<String, String>;
    }
    ```
  - Production SSG provides an implementation that calls non‑Proofdown viewers.
  - Tests provide a fake that emits deterministic placeholders like `<div data-kind="summary:test" data-id="tests-summary"></div>`.

- **[Renderer API hook]**
  - `RenderContext` holds an optional `Arc<dyn ArtifactViewProvider>` for integration tests; unit tests omit it.

## CI Gates

- **[CI workflow]**
  - `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test -- --nocapture`.
  - Optional wasm build and mdBook publish steps (future).

- **[Determinism gate]**
  - Run golden tests twice to ensure identical outputs.

## Coverage & MSRV

- **[Coverage]**
  - Target >80% for core emitters once implemented.
- **[MSRV]**
  - Match parser’s MSRV and document in `Cargo.toml`.
