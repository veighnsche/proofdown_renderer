# Error Model — Renderer v1

Status: v1.0.0-draft (normative)

This document specifies the structured `RenderError` type and required messages. It is compatible with parser error conventions (line/col), though the renderer operates on ASTs and often lacks precise spans.

## 1. Error type

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    // Component/name issues
    UnknownComponent { name: String },
    MissingAttr { name: String, key: String },
    UnknownAttr { name: String, key: String },
    InvalidAttr { name: String, key: String, value: String, expected: &'static str },

    // Bounds/limits
    BoundsExceeded { key: String, value: String, min: i64, max: i64 },
    DepthExceeded { limit: usize, actual: usize },
    NodeLimitExceeded { limit: usize, actual: usize },
    AttrSizeExceeded { key: String, limit: usize, actual: usize },
    OutputSizeExceeded { limit: usize, actual: usize },

    // Sanitization
    SanitizationError { detail: String },

    // Generic/implementation placeholder
    Unimplemented,
}
```

Notes
- `name` is the component name (e.g., `"artifact.table"` or `"grid"`).
- `key` is the attribute key as authored in the AST.
- `expected` is a short human-readable description (e.g., `"integer 1..6"`, `"true|false"`).
- `value` is the literal (unescaped) attribute value.

## 2. Message conventions

- Messages MUST be actionable and consistent. Recommended templates:
  - `unknown component: <name>`
  - `missing attribute <key> in <name>`
  - `unknown attribute <key> in <name>`
  - `invalid <key> in <name>: expected <expected>, got "<value>"`
  - `bounds exceeded for <key>: <value> is outside [<min>, <max>]`
  - `depth limit exceeded: <actual> > <limit>`
  - `node limit exceeded: <actual> > <limit>`
  - `attribute <key> too large: <actual> > <limit>`

## 3. Determinism of errors

- Given the same AST and options, error categories and messages MUST be identical across runs.
- Attribute validation order MUST be deterministic (lexicographic by key) so the first error is stable.

## 4. Error return policy

- On any error, the renderer MUST return `Err(RenderError::...)` and emit no partial HTML.
- The SSG transforms errors into visible error cards.

## 5. Optional span metadata

- If the AST provides span metadata in future versions, the renderer MAY include span notes (line/col) as non-breaking extensions to `RenderError` variants or a separate `Span { line, col }` field.
