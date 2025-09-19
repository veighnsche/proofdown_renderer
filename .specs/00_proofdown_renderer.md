# Proofdown Renderer — Spec (v1-draft)

This document specifies the Proofdown HTML renderer. It consumes a validated Proofdown AST and produces safe, deterministic HTML fragments for use in static site generation and WASM environments.

## 0. Goals

- Deterministic output (byte-identical given same inputs).
- Strict component allowlist and attribute validation.
- No raw HTML execution; all text is escaped.
- Stable id/slug generation and attribute ordering.
- No network or filesystem I/O.

## 1. Inputs

- AST: `proofdown_parser::ast::Document` (or equivalent JSON when used via WASM).
- RenderContext: struct carrying limits, allowlist, slug policy, base path constraints.

## 2. Outputs

- UTF-8 HTML fragment string using `\n` newlines.
- Error on unknown component/attribute, depth/limits exceeded, or invalid attribute type.

## 3. Component Allowlist (minimum)

- Structural: `grid(cols=1..6, gap=0..64)`, `card(title: string)`
- Artifact: `artifact.summary(id)`, `artifact.table(id)`, `artifact.json(id)`, `artifact.markdown(id)`, `artifact.image(id, alt)`, `artifact.link(id, download=true|false, title)`

Unknown components/attributes MUST be rejected.

## 4. Determinism

- Attribute keys sorted lexicographically at emit time.
- Stable id/slug algorithm (ASCII, lower, `-` separators). Collisions get `-2`, `-3`, ... suffixes deterministically.
- Whitespace normalized to single spaces in attributes where applicable.
- Newlines normalized to `\n`.

## 5. Safety

- Escape all text nodes; sanitize attributes by type/bounds.
- No scripts or event handler attributes emitted.
- No style injection; styling limited to typed attrs only.

## 6. Limits

- Depth ≤ 16 (configurable), nodes ≤ 100k (configurable).
- Limit overruns produce a structured error; partial output MUST NOT be emitted.

## 7. Public API (Rust)

```rust
pub struct RenderContext { /* limits, allowlist, slugger, etc. */ }
#[derive(Debug)]
pub enum RenderError { UnknownComponent{ name: String }, InvalidAttr{ name: String }, DepthExceeded, /* ... */ }

pub fn to_html(doc: &proofdown_parser::ast::Document, ctx: &RenderContext) -> Result<String, RenderError>;
```

## 8. WASM Surface (future)

- `wasm_to_html(input_json_ast: &str, options: &str) -> String` returning JSON `{ ok, html|err }`.
- Deterministic JSON stringification; no Date/time.

## 9. Testing Requirements

- Unit tests per component and attribute validation.
- Golden HTML tests: byte-equal snapshot files.
- Determinism test: two renders equal; map orders stable.
- Sanitization tests: escaping and unsafe attributes removed.

## 10. Integration Contract

- SSG uses only verified artifacts; ids resolved upstream; renderer receives only ids and display context.
- Renderer MUST not traverse paths or fetch resources.
