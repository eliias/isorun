use crate::js;
use std::cell::RefCell;

#[magnus::wrap(class = "Isorun::Module")]
pub(crate) struct Module(RefCell<js::module::Context>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Module {}

impl Module {
    pub(crate) fn new(options: &[Value]) -> Result<Self, Error> {
        if options.is_empty() {
            return Err(Error::runtime_error("Provide a valid asset id"));
        }

        let id = RString::from_value(options[0])
            .unwrap()
            .to_string()
            .unwrap();

        let path = RString::from_value(options[1])
            .unwrap()
            .to_string()
            .unwrap();

        let module = Module(RefCell::from(js::module::Module { id, path }));

        Ok(module)
    }

    pub(crate) fn module_id(&self) -> String {
        self.0.borrow().id.clone()
    }

    pub(crate) fn module_load(&self, path: String) -> Result<(), Error> {
        Ok(())
    }

    pub(crate) fn module_import(
        &self,
        export_name: String,
    ) -> Result<ModuleItem, Error> {
        let module = self.0.borrow();
        module
            .import(export_name.as_str())
            .map(|module_item| ModuleItem(RefCell::from(module_item)))
            .map_err(|error| Error::runtime_error(format!("{}", error)))
    }
}
