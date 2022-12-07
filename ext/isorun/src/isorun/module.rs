use crate::isorun::module_item::{Function, Value};
use crate::js;
use magnus::Error;
use std::cell::RefCell;

#[magnus::wrap(class = "Isorun::Module")]
pub(crate) struct Module(pub(crate) RefCell<js::module::Module>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Module {}

impl Module {
    pub(crate) fn id(&self) -> String {
        self.0.borrow().id.to_string()
    }

    pub(crate) fn import(
        &self,
        export_name: String,
    ) -> Result<magnus::Value, Error> {
        let module = self.0.borrow();
        let module_item = module
            .import(export_name.as_str())
            .map_err(|error| Error::runtime_error(format!("{}", error)))?;

        match module_item {
            js::module_item::ModuleItem::Value(v) => {
                Ok(magnus::Value::from(Value(RefCell::from(v))))
            }
            js::module_item::ModuleItem::Function(f) => {
                Ok(magnus::Value::from(Function(RefCell::from(f))))
            }
        }
    }
}
