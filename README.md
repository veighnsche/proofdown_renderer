# proofdown_renderer — Proofdown AST → Safe HTML

A deterministic HTML renderer for the Proofdown language. This crate converts a verified Proofdown AST into safe, sanitized HTML fragments suitable for server-side static site generation and WASM runtimes.

Scope (v0 scaffold)
- Deterministic, pure rendering — no I/O, no clocks, no randomness.
- Stable attribute ordering and id generation.
- Strict component allowlist per spec; unknown components are errors.
- Sanitization and escaping for all text nodes; no raw HTML execution.
- No network calls; all resource references are validated by the caller.

Out of scope for the scaffold
- AST definitions and parsing (handled by `proofdown_parser`).
- WASM bindings (will be added in a future phase).

Integration
- Parse `.pml` into AST using `proofdown_parser`.
- Call `proofdown_renderer::to_html(ast, ctx)` to obtain a safe HTML fragment.
- Compose with other viewers (markdown/json/table:coverage/summary:test/image) at the SSG layer.

Determinism rules
- Stable ordering and formatting.
- Stable id/slug generation with deterministic collision suffixing.
- Consistent `\n` newlines.
- Reject path traversal; only repo-relative references allowed (enforced by the caller prior to render).

Safety rules
- No raw HTML in Proofdown; any such content must be escaped as text.
- All text nodes are HTML-escaped; attribute keys are emitted in stable sorted order.
- Component whitelist enforced with clear error messages.

Status
- This is a scaffold. The API currently returns `Err(RenderError::Unimplemented)` until the AST integration is wired.

License
- Dual-licensed under MIT or Apache-2.0.
