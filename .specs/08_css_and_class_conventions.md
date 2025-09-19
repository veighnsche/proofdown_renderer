# CSS and Class Conventions — Renderer v1

Status: v1.0.0-draft (normative)

This document defines the class naming and structural conventions used by the renderer to enable stable, deterministic styling by SSG pages. All classes are considered part of the renderer’s public output contract.

## 1. Prefix and style scope

- All classes emitted by the renderer MUST start with `pml-`.
- The SSG MUST provide styles for these classes in a deterministic, self-contained stylesheet. No external fetches.

## 2. BEM-like naming

- Blocks: `pml-grid`, `pml-card`, `pml-artifact`, `pml-heading`, `pml-paragraph`, `pml-image`, `pml-caption`.
- Elements: `__` separator (e.g., `pml-card__title`, `pml-card__body`).
- Modifiers: `--` separator (e.g., `pml-artifact--json`, `pml-artifact--table`).

## 3. Attribute reflection

- Certain numeric attributes are reflected as class suffixes for stable layout:
  - `grid.cols = N` → `pml-cols-N` on the grid block.
  - `grid.gap = N` → `pml-gap-N` on the grid block.
- Renderer MUST validate bounds before emitting class suffixes.

## 4. Data attributes (`data-*`)

- The renderer emits data attributes to carry viewer options for SSG/server usage:
  - `data-artifact-id`
  - `data-columns`
  - `data-limit`
  - `data-sort-by`
  - `data-sort-dir`
  - `data-kind`
  - `data-collapsed`
  - `data-depth`
  - `data-json-pointer` (v2)
  - `data-caption` (v2)
  - `data-max-lines` (v2)
- Keys MUST be emitted in lexicographic order for determinism.
- Missing options MUST NOT produce empty attributes; omit the key entirely.

## 5. Accessibility classes

- `pml-caption` marks accessible captions associated with viewers.
- `pml-card__title` is a heading element inside `pml-card`.

## 6. Reserved classes (do not reuse)

- `pml-error` — reserved for SSG error banners.
- `pml-truncated` — reserved class to mark truncated content.

## 7. Example

```html
<div class="pml-grid pml-cols-3 pml-gap-16">
  <section class="pml-card">
    <h3 class="pml-card__title">Tests</h3>
    <div class="pml-card__body">
      <div class="pml-artifact pml-artifact--table" data-artifact-id="tests-summary.json" data-limit="100" data-sort-dir="desc"></div>
    </div>
  </section>
</div>
```
