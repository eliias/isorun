use crate::js;
use magnus::{Error, QNIL};
use std::cell::RefCell;

#[magnus::wrap(class = "Isorun::Function")]
pub(crate) struct Function(pub(crate) RefCell<js::module_item::Function>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Function {}

impl Function {
    pub(crate) fn call(
        &self,
        all_args: &[magnus::Value],
    ) -> Result<magnus::Value, Error> {
        let args = &[];
        self.0.borrow().call(args).map_err(|error| {
            Error::runtime_error(format!("cannot call function: {}", error))
        })
    }
}

#[magnus::wrap(class = "Isorun::Value")]
pub(crate) struct Value(pub(crate) RefCell<js::module_item::Value>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Value {}

impl Value {
    pub(crate) fn to_ruby(&self) -> Option<magnus::Value> {
        self.0.borrow().to_ruby()
    }
}
