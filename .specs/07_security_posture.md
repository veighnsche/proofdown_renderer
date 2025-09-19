# Security Posture — Renderer v1

Status: v1.0.0-draft (normative)

This document defines the renderer’s security model. It is compatible with the parser language specs and the SSG’s security requirements.

References
- Parser language v1: `crates/proofdown_parser/.specs/01_proofdown_language_v1.md`
- Artifact-first semantics: `crates/proofdown_parser/.specs/02_artifact_first_language_spec.md`
- SSG spec (security): `/.specs/00_provenance.md` and `/.specs/16_worker.md`

## 1. Threat model (non-exhaustive)

- Untrusted input: PML source and artifacts (JSON/markdown/images) are untrusted.
- Injection: attempts to inject HTML/JS/CSS via PML attributes or artifact content.
- Resource abuse: deeply nested components, huge text nodes, or large outputs.

## 2. Non-negotiable guarantees

- No network or filesystem I/O.
- No script emission; no event handler attributes; no inline JS.
- No raw HTML passthrough from PML or artifacts.
- Deterministic outputs with a fixed class/attribute scheme.

## 3. Allowed outputs and attributes

- Tags: standard structural tags (`div`, `section`, `h1`..`h4`, `h3` for card title, `figure`, `img`, `figcaption`, `a`, `p`).
- Classes: prefixed with `pml-` and enumerated in `08_css_and_class_conventions.md`.
- Data attributes: `data-*` only for renderer/SSG context, with keys in a fixed allowlist per viewer.
- Links: `href` values supplied by SSG; renderer does not construct URLs.
- Images: `src` values supplied by SSG; renderer does not construct URLs. `alt` text is escaped.
- Styles: Inline `style` is forbidden except for constant numeric properties explicitly allowed:
  - `max-height: <int>px` for `artifact.image` where `<int>` is validated against bounds (§02 semantics).

## 4. Sanitization policy (summary)

- Escape text nodes and attribute values (see `09_html_sanitization_policy.md`).
- Reject attributes not in the allowlist for a component.
- Enforce bounds and types for all attributes.

## 5. Limits and fail-closed behavior

- Enforce `max_depth`, `max_nodes`, and attribute size limits (see `04_determinism_and_limits.md`).
- On any violation or unknown component/attribute, return `Err(RenderError)`; do not emit partial HTML.

## 6. Protocol and URL handling

- Renderer does not accept or manipulate URLs. All URLs come from SSG after verification.
- SSG MUST set appropriate `rel` and `target` sandboxing attributes on `<a>` when embedding downloads or external pages.

## 7. Logging and diagnostics

- Renderer returns structured errors; no logging side effects.

## 8. WASM considerations

- Same guarantees apply in WASM; no access to DOM, network, or time APIs.
