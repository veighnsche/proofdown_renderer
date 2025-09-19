# Artifact‑First Renderer Semantics — v1 (compatible with parser v1)

Status: v1.0.0‑draft (normative)

This document defines normative rendering behavior for the v1 component set using the AST produced by `crates/proofdown_parser`. It complements the parser language specification and preserves the artifact‑first posture.

References
- Parser language v1: `crates/proofdown_parser/.specs/01_proofdown_language_v1.md`
- Artifact‑first semantics (language): `crates/proofdown_parser/.specs/02_artifact_first_language_spec.md`
- v2 additions: `crates/proofdown_parser/.specs/03_proofdown_language_v2.md`

Compatibility notes
- The renderer MUST accept any AST conforming to parser v1.
- Headings are only recognized at the document root per v1; inside components, `#` lines are paragraph text.

## 1. Structural components

### 1.1 grid(cols, gap)

- Required: `cols: 1..6`
- Optional: `gap: 0..64` (default `16`)
- Emits:
  - `<div class="pml-grid pml-cols-{cols} pml-gap-{gap}">…</div>`
- Children: render in order.
- Errors:
  - Missing `cols` → `InvalidAttr{ key: "cols" }`
  - Out‑of‑range values → `BoundsExceeded{ key, value }`

### 1.2 card(title)

- Required: `title: string`
- Emits:
  - `<section class="pml-card">`
  - `<h3 class="pml-card__title">{escaped(title)}</h3>`
  - `<div class="pml-card__body">{children}</div>`
  - `</section>`
- Errors: missing `title` → `MissingAttr{ key: "title" }`

## 2. Artifact viewers (shells)

Renderer emits deterministic shells; SSG injects/supplies verified viewer bodies. All `id` values refer to the signed Index; verification occurs upstream. Unknown attributes MUST error.

### 2.1 artifact.summary(id)

- Required: `id: string`
- Emits:
  - `<div class="pml-artifact pml-artifact--summary" data-artifact-id="{id}"></div>`

### 2.2 artifact.table(id, columns?, limit?, sort_by?, sort_dir?, kind?)

- Required: `id: string`
- Optional/bounded:
  - `columns: string` (comma‑separated keys)
  - `limit: 1..5000` (default `100`)
  - `sort_by: string`
  - `sort_dir: "asc"|"desc"` (default `"desc"`)
  - `kind: string` (enum hints per language v2 §6; accepted in v1 as a hint)
- Emits data attributes in stable lexicographic key order for determinism:
  - `<div class="pml-artifact pml-artifact--table" data-artifact-id data-columns data-kind data-limit data-sort-by data-sort-dir></div>`
- Errors:
  - Out‑of‑range `limit` or invalid `sort_dir` → `BoundsExceeded` / `InvalidAttr`.

### 2.3 artifact.json(id, collapsed?, depth?)

- Required: `id: string`
- Optional/bounded:
  - `collapsed: true|false` (default `true`)
  - `depth: 0..8` (default `3`)
- Emits:
  - `<div class="pml-artifact pml-artifact--json" data-artifact-id data-collapsed data-depth></div>`
- Errors: out‑of‑range `depth` or invalid `collapsed` → `InvalidAttr`/`BoundsExceeded`.

### 2.4 artifact.markdown(id)

- Required: `id: string`
- Emits:
  - `<div class="pml-artifact pml-artifact--markdown" data-artifact-id></div>`
- Sanitization of artifact HTML happens in non‑Proofdown markdown viewer, not here.

### 2.5 artifact.image(id, alt, max_height?)

- Required: `id: string`, `alt: string`
- Optional/bounded: `max_height: 128..2048` (default `640`)
- Emits:
  - `<figure class="pml-artifact pml-artifact--image">`
  - `<img class="pml-image" src="{verified_href}" alt="{escaped(alt)}" style="max-height:{px}px" />`
  - Optional v2 caption slot under `<figcaption>` (see v2 spec)
  - `</figure>`
- Errors: missing `alt` → `MissingAttr{ key: "alt" }`; out‑of‑range `max_height` → `BoundsExceeded`.

### 2.6 artifact.link(id, title?, download?)

- Required: `id: string`
- Optional: `title: string`, `download: true|false` (default `false`)
- Emits:
  - `<a class="pml-artifact pml-artifact--link" href="{verified_href}"{download?}> {escaped(title_or_id)} </a>`

## 3. Placeholder emission policy

- Data attributes are omitted when value is absent (no empty strings).
- Attribute emission order is lexicographic by key for determinism.
- Boolean data attributes use `true|false` string values.

## 4. Failure presentation (shell level)

- When the renderer encounters a semantic error (unknown component/attr, bounds), it MUST return `RenderError` and emit no partial HTML.
- The SSG is responsible for turning `RenderError` into a visible error card on the page.

## 5. Accessibility

- `artifact.image` requires `alt`; `card` titles are headings (`<h3>`), ensuring landmark semantics.
- Tables SHOULD have a caption via attribute or surrounding card title.

## 6. Determinism

- Class names and data attributes are emitted in a stable order.
- Newlines between block nodes are single `\n`.
- No timestamps, UUIDs, or random suffixes are permitted; id/slug generation is deterministic (see Determinism spec).
