use crate::isorun::js_runtime::JS_RUNTIME;
use magnus::block::Proc;
use magnus::Error;

#[magnus::wrap(class = "Isorun::Renderer")]
pub(crate) struct Renderer;

impl Renderer {
    pub(crate) fn new() -> Renderer {
        Renderer
    }

    pub(crate) fn render(
        &self,
        bundle_path: String,
        block: Proc,
    ) -> Result<String, Error> {
        JS_RUNTIME
            .with(|js_runtime| js_runtime.render(bundle_path.as_str(), block))
    }
}
