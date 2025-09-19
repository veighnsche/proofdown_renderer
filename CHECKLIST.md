# Production Readiness Checklist — proofdown_renderer

API
- [ ] `RenderContext` documented, stable, and minimal
- [ ] `to_html(ast, ctx)` returns `Result<String, RenderError>`
- [ ] `RenderError` spans and messages are actionable

Determinism
- [ ] Stable attribute ordering and whitespace policy
- [ ] Stable id/slug generation and collision handling
- [ ] Newlines normalized to `\n`

Safety
- [ ] No network calls or ambient I/O
- [ ] All text is HTML-escaped; no raw HTML execution
- [ ] Component whitelist enforced; unknown attributes rejected

Correctness
- [ ] Spec conformance with `.specs/10_proofdown.md`
- [ ] Golden HTML tests for representative inputs
- [ ] Large content truncation policy delegated to caller; renderer is pure

Tooling
- [ ] `cargo fmt` / `clippy -D warnings` clean
- [ ] Minimal dependencies; MSRV documented if needed

Docs
- [ ] README explains scope, determinism, and safety rules
- [ ] CHANGELOG started (semantic, or minimal)
- [ ] WASM binding notes if feature enabled
