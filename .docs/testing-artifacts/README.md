# Testing Artifacts Catalog (Renderer)

- `fixtures/` directory (in tests) will hold input AST JSON or PML (via parser) and expected HTML.
- `golden/` directory will hold byte-equal HTML outputs.
- Use `UPDATE_GOLDEN=1` to regenerate goldens in harness.
