//! proofdown_renderer — Deterministic Proofdown AST → safe HTML
//!
//! This is a scaffold. The public API will accept the Proofdown AST once the parser integration is wired.

mod html;
pub mod sanitize;

use std::fmt;

/// Rendering context (placeholder for scaffold).
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Maximum allowed include or node depth (if applicable).
    pub max_depth: usize,
    /// Component allowlist (names) enforced during rendering.
    pub component_allowlist: Vec<String>,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self { max_depth: 64, component_allowlist: Vec::new() }
    }
}

/// Rendering errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// Renderer not implemented yet (scaffold).
    Unimplemented,
    /// An unknown or disallowed component was encountered.
    UnknownComponent { name: String },
    /// Depth limit exceeded while rendering.
    DepthExceeded,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::Unimplemented => write!(f, "renderer not implemented"),
            RenderError::UnknownComponent { name } => write!(f, "unknown component: {}", name),
            RenderError::DepthExceeded => write!(f, "depth limit exceeded"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Render a Proofdown document into a safe HTML fragment.
///
/// For the scaffold, this returns `Err(RenderError::Unimplemented)`.
/// The final API will accept an AST from `proofdown_parser`.
pub fn to_html(_doc_placeholder: &str, _ctx: &RenderContext) -> Result<String, RenderError> {
    Err(RenderError::Unimplemented)
}
