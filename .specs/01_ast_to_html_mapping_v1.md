# AST → HTML Mapping — Renderer v1 (compatible with `proofdown_parser` v1)

Status: v1.0.0-draft (normative for renderer behavior)

This document specifies how a `proofdown_parser` v1 AST is transformed into a safe, deterministic HTML fragment. It complements the language grammar in `crates/proofdown_parser/.specs/01_proofdown_language_v1.md` and the artifact-first semantics in `crates/proofdown_parser/.specs/02_artifact_first_language_spec.md`.

References
- Parser grammar v1: `crates/proofdown_parser/.specs/01_proofdown_language_v1.md`
- Artifact-first semantics v1: `crates/proofdown_parser/.specs/02_artifact_first_language_spec.md`
- Renderer error model: `./05_error_model.md`
- Determinism and limits: `./04_determinism_and_limits.md`
- Sanitization policy: `./09_html_sanitization_policy.md`

## 1. Document root

Input: `Document { blocks: Vec<Block> }`

Output: A concatenation of rendered block nodes into a single HTML fragment using `\n` newlines. No outer `<html>` or `<body>` is emitted by this renderer; the SSG provides page chrome.

## 2. Blocks

### 2.1 Heading { level: 1..4, text }

- Rendering (top-level only, from parser contract):
  - Tag: `<h1>`..`<h4>`
  - Classes: `pml-heading pml-h{level}`
  - Text content is escaped per sanitization policy.
- Example:
  - AST: `Heading { level: 1, text: "QA Evidence" }`
  - HTML: `<h1 class="pml-heading pml-h1">QA Evidence</h1>`

### 2.2 Paragraph { text }

- Tag: `<p>`
- Classes: `pml-paragraph`
- Escaping: full text escaping.
- Whitespace: paragraph text comes pre-normalized by parser; renderer does not re-flow text.

### 2.3 Component(Component)

- Dispatch by `Component.name`. Unknown components MUST raise `RenderError::UnknownComponent { name }`.
- Common attributes are validated and emitted in stable order. Unknown attributes MUST error.

## 3. Components (v1 minimum)

### 3.1 grid(cols, gap)

- Purpose: layout container.
- Required/optional attributes (bounds per parser semantics):
  - `cols: 1..6` (required)
  - `gap: 0..64` (optional, default implementation-dependent constant, e.g., `16`)
- HTML:
  - Tag: `<div>`
  - Classes: `pml-grid pml-cols-{cols} pml-gap-{gap}`
  - Children: render all child blocks inside.
- Errors:
  - Missing/invalid attrs → `InvalidAttr`.
  - Depth/size handled globally (see §4 Determinism and limits).

### 3.2 card(title)

- Purpose: panel grouping.
- Attributes:
  - `title: string` (required)
- HTML:
  - Wrapper: `<section class="pml-card">`
  - Title: `<h3 class="pml-card__title">…</h3>` (escaped)
  - Body: `<div class="pml-card__body">{children}</div>`
- Children: render block children within body.

### 3.3 artifact.summary(id)

- Emits a placeholder container that the SSG fills using the verified summary viewer output.
- Attributes:
  - `id: string` (required)
- HTML (renderer submodule role):
  - `<div class="pml-artifact pml-artifact--summary" data-artifact-id="…"></div>`
  - No client scripting. SSG injects verified HTML for the summary viewer server-side.

### 3.4 artifact.table(id, columns?, limit?, sort_by?, sort_dir?, kind?)

- Attributes per artifact-first semantics; bounds validated.
- HTML shell:
  - `<div class="pml-artifact pml-artifact--table" data-artifact-id="…" data-columns="…" data-limit="…" data-sort-by="…" data-sort-dir="…" data-kind="…"></div>`
- Determinism: attribute emission order is stable; absent attrs omitted.

### 3.5 artifact.json(id, collapsed?, depth?)

- HTML shell:
  - `<div class="pml-artifact pml-artifact--json" data-artifact-id="…" data-collapsed="true|false" data-depth="N"></div>`

### 3.6 artifact.markdown(id)

- HTML shell:
  - `<div class="pml-artifact pml-artifact--markdown" data-artifact-id="…"></div>`

### 3.7 artifact.image(id, alt, max_height?)

- HTML:
  - `<figure class="pml-artifact pml-artifact--image">`
  - `<img src="{resolved_download_href}" alt="…" class="pml-image" style="max-height:{px}px" />`
  - Optional caption: `<figcaption class="pml-caption">…</figcaption>` (v2)
  - Note: `src` is a verified, content-addressed URL provided by SSG; renderer does not fetch.

### 3.8 artifact.link(id, title?, download?)

- HTML:
  - `<a class="pml-artifact pml-artifact--link" href="{verified_href}" download>Title</a>` (download attr when requested)
- Title default: use `id` if `title` absent.

## 4. Attribute policy

- Validation: name and type per component-specific table above and `02_artifact_first_renderer_semantics_v1.md`.
- Ordering: attributes are serialized in lexicographic key order when reflected as `data-*` or as CSS class suffixes.
- Escaping: attribute values are escaped according to `09_html_sanitization_policy.md`.

## 5. Classes and BEM

- All renderer-emitted classes are prefixed with `pml-`.
- Block elements: `pml-grid`, `pml-card`, `pml-artifact`.
- Elements use `__` (e.g., `pml-card__title`), modifiers use `--` (e.g., `pml-artifact--json`).

## 6. Examples

Input (PML):
```pml
# QA Evidence — {{ commit }}

<grid cols=3 gap=16>
  <card title="Tests">
    <artifact.summary id="tests-summary" />
  </card>
</grid>
```

Rendered HTML:
```html
<h1 class="pml-heading pml-h1">QA Evidence — {{ commit }}</h1>
<div class="pml-grid pml-cols-3 pml-gap-16">
  <section class="pml-card">
    <h3 class="pml-card__title">Tests</h3>
    <div class="pml-card__body">
      <div class="pml-artifact pml-artifact--summary" data-artifact-id="tests-summary"></div>
    </div>
  </section>
</div>
```
