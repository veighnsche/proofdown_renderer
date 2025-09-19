# WASM Surface — Renderer (future, v1.x)

Status: v1.x-draft (additive; optional target)

This document specifies the JSON-based interface for using the renderer in WebAssembly contexts (browsers/Workers) with deterministic outputs. It mirrors the native API and preserves determinism and safety.

## 1. Goals

- Provide a minimal, stable JSON interface for AST→HTML rendering.
- Avoid any host interaction: no DOM, no network, no time, no RNG.
- Deterministic byte output given the same inputs.

## 2. Function surface

```ts
// input_json_ast: serde-JSON Document produced by parser
// options_json: JSON object with render options (limits, flags)
// Returns a JSON string with one of:
//  - { ok: true, html: string }
//  - { ok: false, err: { kind: string, msg: string, ... } }
export function wasm_to_html(input_json_ast: string, options_json: string): string;
```

Notes
- `input_json_ast` MUST be validated as a parser-conformant AST. Unknown component names/attrs cause a renderer error.
- `options_json` mirrors a subset of `RenderContext`:
  - `{ "max_depth": 16, "max_nodes": 100000, "attr_size_limit": 8192, "mode": "v1|v2" }`

## 3. Determinism

- The returned `html` uses `\n` newlines and stable attribute order.
- Error categories and messages are stable for identical inputs.

## 4. Error payload

```json
{
  "ok": false,
  "err": {
    "kind": "UnknownComponent|MissingAttr|UnknownAttr|InvalidAttr|BoundsExceeded|DepthExceeded|NodeLimitExceeded|AttrSizeExceeded|OutputSizeExceeded|SanitizationError|Unimplemented",
    "msg": "human-readable detail",
    "name?": "component.name",
    "key?": "attr_key"
  }
}
```

## 5. Compatibility

- The WASM surface is additive and MUST NOT break native Rust API guarantees.
- JSON encoding/field order is stable (serde defaults). No timestamps.
