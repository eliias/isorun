use crate::js;
use crate::js::worker::WORKER;
use magnus::Error;
use std::cell::RefCell;
use std::ops::Deref;

#[magnus::wrap(class = "Isorun::Function")]
pub(crate) struct Function(pub(crate) RefCell<js::module_item::Function>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Function {}

impl Function {
    pub(crate) fn call(
        &self,
        args: &[magnus::Value],
    ) -> Result<magnus::Value, Error> {
        let args = WORKER.with(|worker| {
            let func = self.0.borrow();
            let realm = func.realm.borrow();
            let realm = realm.deref();

            let mut v8_args = vec![];
            for arg in args {
                v8_args.push(worker.to_v8(realm, *arg).unwrap());
            }
            v8_args
        });

        self.0.borrow().call(args.as_slice()).map_err(|error| {
            Error::runtime_error(format!("cannot call function: {}", error))
        })
    }
}
