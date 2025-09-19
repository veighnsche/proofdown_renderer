# Proofdown Renderer Authoring Guide

## Philosophy

- Spec-first; deterministic; security before features.
- Keep API minimal; avoid leaking internal details.

## Structure

- `src/lib.rs`: public API and error types.
- `src/html.rs`: emitters and helpers.
- `src/sanitize.rs`: escaping/sanitization utilities.
- `tests/`: unit & golden tests.
- `.specs/`: renderer contract and behavior.
- `book/`: mdBook developer docs.

## Contribution workflow

1. Update `.specs/` if behavior changes.
2. Add/adjust unit tests and goldens.
3. Run local CI script.
4. Open PR with rationale and spec diffs.

## Coding rules

- No I/O, threading, or randomness.
- Avoid `unsafe`.
- Keep imports at top; `clippy -D warnings` clean.

## Error messages

- Include component/attribute names and, when available, spans.
- Prefer actionable messages with remediation hints.
