use crate::{isorun, js};
use magnus::Error;
use std::cell::RefCell;
use std::rc::Rc;

#[magnus::wrap(class = "Isorun::Context")]
pub(crate) struct Context(Rc<RefCell<js::context::Context>>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Context {}

impl Context {
    pub(crate) fn create() -> Result<Self, Error> {
        let context = js::context::Context::create();

        Ok(Context(Rc::from(RefCell::from(context))))
    }

    pub(crate) fn load(
        &self,
        path: String,
    ) -> Result<isorun::module::Module, Error> {
        let module =
            js::context::Context::load(path.as_str()).map_err(|error| {
                Error::runtime_error(format!(
                    "cannot load module: `{:?}`",
                    error
                ))
            })?;

        Ok(isorun::module::Module(RefCell::from(module)))
    }
}
