# API

```rust
pub struct RenderContext { /* limits, allowlist, slugger, etc. */ }
#[derive(Debug)]
pub enum RenderError { /* ... */ }

pub fn to_html(doc: &proofdown_parser::ast::Document, ctx: &RenderContext) -> Result<String, RenderError>;
```

The API is deterministic and side-effect free.
