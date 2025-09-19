# Renderer v2 Extensions (additive over v1)

Status: v2.0.0-draft (compatible with all valid v1 ASTs)

This document specifies additive renderer features aligned with `crates/proofdown_parser/.specs/03_proofdown_language_v2.md`. It MUST NOT break v1 behavior.

References
- Parser v2 spec: `crates/proofdown_parser/.specs/03_proofdown_language_v2.md`
- Renderer v1 mapping: `./01_ast_to_html_mapping_v1.md`
- Artifact-first semantics v1: `./02_artifact_first_renderer_semantics_v1.md`

## 1. New/extended viewers

### 1.1 `artifact.text`

- Purpose: display short plaintext excerpts (logs, SQL, configs) as escaped, preformatted blocks.
- Required:
  - `id: string`
- Optional (bounded):
  - `max_lines: 1..500` (default `100`)
  - `caption: string`
- HTML shell (SSG injects content deterministically):
  - `<div class="pml-artifact pml-artifact--text" data-artifact-id data-max-lines></div>`
  - When content is injected by SSG, it MUST be `<pre class="pml-pre">…</pre>` with lines truncated to `max_lines` and a truncation banner.

### 1.2 `artifact.json` additions

- `json_pointer: string` — RFC 6901 pointer to a subtree.
- `caption: string` — informative caption under the widget.
- Shell adds data attributes: `data-json-pointer`, `data-caption` (when present).

### 1.3 `artifact.table` additions

- `caption: string`
- `columns` may include JSON Pointer selectors (RFC 6901) for nested values.
- Shell adds `data-caption`.

### 1.4 `artifact.image` additions

- `caption: string` — emit `<figcaption class="pml-caption">…</figcaption>` escaped.

### 1.5 `artifact.link` additions

- `caption: string` — optional small `<div class="pml-caption">…</div>` after the link.

## 2. Accessibility refinements

- Captions improve context for screen readers; when present, they are rendered in `<figcaption>` for images and in a visually-muted `<div class="pml-caption">` for other viewers.
- Table captions SHOULD be used for `artifact.table` via the attribute or the surrounding card title.

## 3. Determinism & safety (unchanged)

- All new attributes are validated with deterministic emission order.
- No scripts; all text is escaped.

## 4. Error handling

- Unknown v2 attributes in v1 mode MUST error.
- In v2 mode, attributes out of bounds (e.g., `max_lines`) MUST raise `BoundsExceeded`.
