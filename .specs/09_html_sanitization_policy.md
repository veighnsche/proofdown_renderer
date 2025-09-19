# HTML Sanitization Policy — Renderer v1

Status: v1.0.0-draft (normative)

This document defines the mandatory escaping and sanitization rules for all text and attribute content emitted by the renderer. It complements the security posture and determinism specs.

References
- Security posture: `./07_security_posture.md`
- Determinism: `./04_determinism_and_limits.md`
- CSS/class conventions: `./08_css_and_class_conventions.md`

## 1. Text escaping (nodes)

- All text inserted into element bodies (e.g., heading text, paragraph text, captions) MUST be escaped by replacing:
  - `&` → `&amp;`
  - `<` → `&lt;`
  - `>` → `&gt;`
  - `"` → `&quot;`
  - `'` → `&#39;`
- Escaping MUST be applied exactly once (idempotent with respect to untrusted input).

## 2. Attribute escaping

- Attribute values MUST be escaped using the same table as text nodes.
- Numeric attributes are validated and emitted as canonical decimal integers without quotes in CSS fragments (e.g., `style="max-height:640px"`); otherwise, attribute values SHOULD be quoted.

## 3. Forbidden constructs

- The renderer MUST NOT emit:
  - Any `<script>` tag or event handler attributes (e.g., `onclick`).
  - Arbitrary `style` attributes; only whitelisted numeric CSS properties are allowed (e.g., `max-height` for images).
  - `srcdoc`, `src`, or `href` values derived from PML; such URLs MUST be provided by the SSG as verified and content-addressed.

## 4. Allowed attributes per element

- `div.pml-artifact`: `data-*` keys only from the per-viewer allowlist.
- `section.pml-card`: no data attributes.
- `h3.pml-card__title`: no attributes beyond `class`.
- `img.pml-image`: `src` (provided by SSG), `alt` (escaped), `style` (only `max-height:<int>px`).
- `a.pml-artifact--link`: `href` (provided by SSG), optional `download` (boolean attribute), `rel`, `target` provided by SSG.

## 5. Whitespace normalization

- The renderer MUST NOT emit leading/trailing spaces in attributes.
- Newlines in text nodes are preserved as `\n` (if present from AST) but SHOULD be avoided in headings and titles.

## 6. Fallback and errors

- If escaping fails (should not happen in safe Rust), the renderer MUST return `RenderError::SanitizationError`.
- Unknown attributes MUST be rejected with `RenderError::UnknownAttr`.
