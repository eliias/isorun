use crate::js;
use crate::js::worker::WORKER;
use magnus::Error;
use std::cell::RefCell;
use std::ops::Deref;
use v8::{Global, Value};

#[magnus::wrap(class = "Isorun::Function")]
pub(crate) struct Function(pub(crate) RefCell<js::module_item::Function>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Function {}

impl Function {
    pub(crate) fn call(
        &self,
        args: &[magnus::Value],
    ) -> Result<magnus::Value, Error> {
        WORKER.with(|worker| {
            let func = self.0.borrow();
            let realm = func.realm.borrow();
            let realm = realm.deref();

            let v8_args: Vec<Global<Value>> = args
                .iter()
                .map(|arg| worker.to_v8(realm, *arg).unwrap())
                .collect();

            let result =
                self.0.borrow().call(v8_args.as_slice()).map_err(|error| {
                    Error::runtime_error(format!(
                        "cannot call function: {}",
                        error
                    ))
                });

            result
        })
    }
}
