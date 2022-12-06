use crate::js;
use crate::js::context::CONTEXT;
use magnus::{Error, Value};
use std::cell::RefCell;

#[magnus::wrap(class = "Isorun::ModuleItem")]
pub(crate) struct ModuleItem(pub(crate) RefCell<js::module_item::ModuleItem>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for ModuleItem {}

impl ModuleItem {
    pub(crate) fn module_item_export_name(&self) -> String {
        self.0.borrow().export_name.to_string()
    }

    pub(crate) fn module_item_call(
        &self,
        all_args: &[Value],
    ) -> Result<Value, Error> {
        CONTEXT.with(|context| {
            let args = context.convert_to_v8(all_args[0]).unwrap();
            let kwargs = context.convert_to_v8(all_args[1]).unwrap();

            let value = self.0.borrow().call(args, kwargs);
            context
                .convert_to_ruby(value)
                .map_err(|error| Error::runtime_error(format!("{}", error)))
        })
    }

    pub(crate) fn module_item_to_value(&self) -> Result<Value, Error> {
        CONTEXT.with(|context| {
            let value = self.0.borrow().to_value();
            context
                .convert_to_ruby(value.unwrap())
                .map_err(|error| Error::runtime_error(format!("{}", error)))
        })
    }
}
