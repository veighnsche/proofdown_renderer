# Proofdown Renderer — SPEC List (compatible with `crates/proofdown_parser`)

This index enumerates all renderer specifications and maps them to the parser’s specs to ensure compatibility and clear ownership boundaries.

Compatibility targets
- Language v1 (grammar): `crates/proofdown_parser/.specs/01_proofdown_language_v1.md`
- Artifact-first semantics v1: `crates/proofdown_parser/.specs/02_artifact_first_language_spec.md`
- Language v2 additions: `crates/proofdown_parser/.specs/03_proofdown_language_v2.md`
- Parser↔SSG contract: `crates/proofdown_parser/.specs/10_contract_with_provenance_ssg.md`

Renderer spec set
- 00 — Overview and goals
  - `.specs/00_proofdown_renderer.md`

- 01 — AST → HTML mapping (v1)
  - `.specs/01_ast_to_html_mapping_v1.md`
  - Maps `Document`, `Block`, `Component`, and attributes to deterministic HTML structure and classes.

- 02 — Viewer semantics (artifact-first v1)
  - `.specs/02_artifact_first_renderer_semantics_v1.md`
  - Normative rendering behavior for `artifact.*` and structural components (`grid`, `card`), aligned with parser v1 semantics.

- 03 — Additive v2 features
  - `.specs/03_renderer_v2_extensions.md`
  - Adds `artifact.text`, captions, JSON Pointer columns for tables, and accessibility refinements. Backward compatible with v1.

- 04 — Determinism and limits
  - `.specs/04_determinism_and_limits.md`
  - Stable ordering, id/slug policy, newline normalization, node/depth limits, and failure behavior.

- 05 — Error model
  - `.specs/05_error_model.md`
  - Structured `RenderError` with categories, messages, and span notes when available.

- 06 — Accessibility
  - `.specs/06_accessibility.md`
  - Alt text rules, table captions, keyboard/landmark expectations when embedded in SSG pages.

- 07 — Security posture
  - `.specs/07_security_posture.md`
  - No raw HTML execution, attribute sanitization, no I/O.

- 08 — CSS/class conventions
  - `.specs/08_css_and_class_conventions.md`
  - BEM-like class naming (`pml-*`), stable attribute-to-class mapping, and reserved data attributes.

- 09 — Sanitization policy
  - `.specs/09_html_sanitization_policy.md`
  - Escaping rules, attribute type enforcement, and forbidden attributes.

- 10 — Contract with SSG
  - `.specs/10_contract_with_provenance_ssg.md`
  - API boundary, inputs/outputs, and determinism guarantees.

- 11 — WASM surface (future)
  - `.specs/11_wasm_surface.md`
  - JSON interface and determinism requirements in browser/Worker contexts.

- schemas/
  - `.specs/schemas/README.md`
  - Houses small schemas/conventions used by the renderer (e.g., class naming, viewer option enums).

Change control
- Any change to 01–06 and 10 requires a contract update and SSG alignment.
- v2 additions are additive and MUST not break v1 behaviors.
