# Renderer Schemas and Conventions

This folder documents small JSON or textual schemas and enumerations used by the renderer. These are not networked and exist only to make behavior deterministic and reviewable.

Contents
- `viewer_options.json` (informative): enumerates allowed data-* keys per viewer and their types.
- `classes.md` (informative): lists BEM-style class names with their intended semantics.

Notes
- Schemas here are descriptive aids for contributors and for golden tests that validate option handling.
- They are not used at runtime; the renderer validates options via compiled code.
