# Determinism and Limits — Renderer v1

Status: v1.0.0-draft (normative)

This document defines hard requirements for deterministic output and resource limits for the Proofdown HTML renderer. It complements grammar and semantics in the parser specs and the renderer’s mapping/semantics docs.

References
- Parser limits: `crates/proofdown_parser/.specs/01_proofdown_language_v1.md` §10
- Renderer mapping: `./01_ast_to_html_mapping_v1.md`
- Renderer semantics: `./02_artifact_first_renderer_semantics_v1.md`

## 1. Determinism

A conforming renderer MUST produce byte-identical HTML for identical inputs (AST + options) across runs and platforms.

- Pure functions:
  - No filesystem or network I/O.
  - No environment access (time, RNG, locale, env vars).
- Stable ordering:
  - Iterate collections in deterministic order; avoid HashMap iteration without sorting.
  - When reflecting attributes as `data-*` or class suffixes, emit keys in lexicographic order.
- Stable id/slug policy:
  - Slug algorithm: lower-case ASCII, spaces → `-`, non `[a-z0-9-]` removed, squeeze repeated `-` to single.
  - Collision policy: first occurrence uses base slug; subsequent collisions append `-2`, `-3`, … in encounter order.
- Newline normalization:
  - Output uses `\n` (LF) exclusively. No trailing whitespace.
- String formatting:
  - No locale-sensitive formatting. Numbers (when present in attributes) are emitted as canonical decimal integers.
- Class naming:
  - All classes prefixed `pml-` (see `08_css_and_class_conventions.md`).
- Error determinism:
  - `RenderError` messages and categories are stable for a given invalid input.

## 2. Limits (renderer context)

The renderer enforces limits to prevent pathological inputs from consuming excessive resources. Limits are exposed in `RenderContext` and are independent of (but compatible with) parser limits.

- Depth: `max_depth` (default 16)
  - Counts nested components; root document depth is 0.
  - Exceeding emits `RenderError::DepthExceeded { limit, actual }`.
- Node count: `max_nodes` (default 100_000)
  - Counts total blocks visited.
  - Exceeding emits `RenderError::NodeLimitExceeded { limit, actual }`.
- Attribute size:
  - Individual attribute values MAY be capped (default 8 KiB). Exceeding emits `RenderError::AttrSizeExceeded { key, limit, actual }`.
- Output size guards (defensive):
  - Implementations MAY limit HTML output length (e.g., 5 MiB) and return `RenderError::OutputSizeExceeded` (SSG SHOULD truncate at a higher layer instead).

## 3. Failure behavior

- On any error, the renderer MUST return `Err(RenderError::…)` and emit no partial HTML.
- The SSG is responsible for presenting an error card or page.

## 4. Performance expectations

- Linear time with respect to number of nodes and attributes in the common case.
- Avoid quadratic string concatenation; prefer capacity reservation and push.
- WASM target: single-threaded, bounded memory; avoid large intermediate allocations.

## 5. Compatibility with parser limits

- The renderer SHOULD accept any AST within parser limits (§10 in parser v1 spec).
- Render-time `max_depth` and `max_nodes` SHOULD be set at least as high as parser defaults unless the SSG deliberately lowers them.
