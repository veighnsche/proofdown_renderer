# Accessibility — Renderer v1

Status: v1.0.0-draft (normative)

This document defines accessibility requirements for the HTML emitted by the renderer. It aligns with the parser language specs and the SSG’s accessibility baseline in `/.specs/40_accessibility.md`.

References
- Parser language v1: `crates/proofdown_parser/.specs/01_proofdown_language_v1.md`
- Renderer mapping: `./01_ast_to_html_mapping_v1.md`
- Renderer semantics: `./02_artifact_first_renderer_semantics_v1.md`

## 1. Headings and structure

- Top-level headings render as `<h1>`..`<h4>` with classes `pml-heading pml-h{level}`.
- Cards render a nested heading `<h3 class="pml-card__title">` to establish section structure.
- The SSG page template provides landmarks (`<header>`, `<nav>`, `<main>`, `<footer>`), not the renderer.

## 2. Images

- `artifact.image` requires `alt`. The renderer MUST escape alt text and emit it on the `<img>` tag.
- Optional captions render in `<figcaption class="pml-caption">` (v2).

## 3. Tables

- `artifact.table` SHOULD have an accessible caption:
  - via `caption` attribute (v2), rendered as `<caption>` or a visually-associated element; or
  - derived from surrounding `card(title=...)` when present.
- Table semantics and keyboard interaction are provided by the SSG viewer for non‑Proofdown tables; the renderer emits only shells.

## 4. Links

- `artifact.link` MUST use a descriptive `title` when provided; otherwise the `id` is used.
- SSG MUST set `rel="noopener noreferrer"` and appropriate `target`/sandboxing policies when linking to downloads or external views.

## 5. Keyboard and focus

- Renderer does not emit focus-managing scripts. All interactive behavior (if any) is handled by SSG pages without JavaScript requirements.

## 6. Contrast and theming

- Renderer emits classes only; CSS tokens and contrast compliance are enforced by SSG styles. The default theme must meet WCAG AA where applicable.
