use crate::isorun;
use crate::js::module::Module;
use crate::js::worker::WORKER;
use deno_core::JsRealm;
use magnus::block::Proc;
use magnus::{exception, Error};
use std::cell::RefCell;
use std::rc::Rc;

#[magnus::wrap(class = "Isorun::Context")]
pub(crate) struct Context(pub(crate) Rc<RefCell<JsRealm>>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Context {}

impl Context {
    pub(crate) fn new() -> Result<Self, Error> {
        WORKER
            .with(|worker| worker.create_realm())
            .map(|realm| Context(Rc::new(RefCell::from(realm))))
            .map_err(|_error| {
                Error::new(
                    exception::runtime_error(),
                    "cannot create JavaScript context",
                )
            })
    }

    pub(crate) fn set_receiver(&self, receiver: Option<Proc>) {
        WORKER.with(|worker| worker.ruby_receiver.replace(receiver));
    }

    pub(crate) fn load(
        &self,
        path: String,
    ) -> Result<isorun::module::Module, Error> {
        let realm = self.0.clone();
        WORKER
            .with(|worker| {
                worker
                    .load_module(path.as_str())
                    .map(|id| Module { id, realm })
            })
            .map(|module| isorun::Module(RefCell::from(module)))
            .map_err(|error| {
                Error::new(
                    exception::runtime_error(),
                    format!(
                        "cannot load module: `{}`: {}",
                        path.as_str(),
                        error
                    ),
                )
            })
    }
}
