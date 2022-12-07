use crate::js::module::Module;
use crate::js::worker::WORKER;
use deno_core::error::AnyError;

pub(crate) struct Context;

impl Context {
    pub(crate) fn create() -> Self {
        Self {}
    }

    pub(crate) fn load(path: &str) -> Result<Module, AnyError> {
        WORKER.with(|worker| worker.load_module(path).map(|id| Module { id }))
    }
}
