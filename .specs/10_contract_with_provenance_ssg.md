# Contract with provenance_ssg

This document defines how `provenance_ssg` calls `proofdown_renderer` to produce the front page HTML fragment.

## Responsibilities

- provenance_ssg:
  - Loads & verifies manifest, resolves artifact ids, and parses `.pml` via `proofdown_parser`.
  - Provides `RenderContext` with limits, allowlist, and base URL policy.
  - Embeds returned HTML fragment into the page template.

- proofdown_renderer:
  - Validates component names and attribute bounds.
  - Emits safe, sanitized HTML deterministically.
  - Returns structured `RenderError` for unknown components/attrs or limits.

## API

```rust
pub fn to_html(doc: &proofdown_parser::ast::Document, ctx: &RenderContext) -> Result<String, RenderError>;
```

## Determinism & Safety

- No timestamps, RNG, or env access.
- Attribute order sorted; ids stable.
- All text escaped; no raw HTML/script/style.

## Error Handling

- Errors include component/attribute names and (when available) span info.
- SSG surfaces errors as a clear error card in the page.
