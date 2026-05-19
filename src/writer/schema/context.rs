use std::collections::HashSet;

use oas3::OpenApiV3Spec;

/// State threaded through schema rendering. `visiting` tracks ref names currently
/// on the recursion stack so cyclic `$ref`s render as links rather than overflowing.
pub(super) struct RenderCtx<'a> {
    pub(super) spec: &'a OpenApiV3Spec,
    pub(super) multi_use: &'a HashSet<String>,
    pub(super) visiting: HashSet<String>,
}
