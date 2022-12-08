use crate::isorun;
use crate::js::module::Module;
use crate::js::worker::WORKER;
use magnus::Error;
use std::cell::RefCell;

#[magnus::wrap(class = "Isorun::Context")]
pub(crate) struct Context();

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Context {}

impl Context {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(Context())
    }

    pub(crate) fn load(
        &self,
        path: String,
    ) -> Result<isorun::module::Module, Error> {
        WORKER
            .with(|worker| {
                worker.load_module(path.as_str()).map(|id| Module { id })
            })
            .map(|module| isorun::Module(RefCell::from(module)))
            .map_err(|error| {
                Error::runtime_error(format!(
                    "cannot load module: `{}`: {}",
                    path.as_str(),
                    error
                ))
            })
    }
}
