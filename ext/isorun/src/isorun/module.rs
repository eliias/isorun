use crate::isorun::function::Function;
use crate::js;
use magnus::{exception, Error, IntoValue};
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
        let module_item =
            module.import(export_name.as_str()).map_err(|error| {
                Error::new(exception::runtime_error(), format!("{}", error))
            })?;

        match module_item {
            js::module_item::ModuleItem::Value(v) => Ok(v.to_ruby().unwrap()),
            js::module_item::ModuleItem::Function(f) => {
                Ok(Function(RefCell::from(f)).into_value())
            }
        }
    }
}
