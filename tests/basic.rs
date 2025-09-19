use proofdown_renderer::{to_html, RenderContext, RenderError};

#[test]
fn scaffold_compiles_and_returns_unimplemented() {
    let ctx = RenderContext::default();
    let res = to_html("", &ctx);
    assert!(matches!(res, Err(RenderError::Unimplemented)));
}
