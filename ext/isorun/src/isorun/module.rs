use crate::isorun::js_runtime::JS_RUNTIME;
use magnus::block::Proc;
use magnus::{Error, RArray, RHash, RString, RStruct, Value};

#[magnus::wrap(class = "Isorun::Module")]
pub(crate) struct Module {
    id: String,
}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Module {}

impl Module {
    pub(crate) fn new(options: &[Value]) -> Result<Self, Error> {
        if options.is_empty() {
            return Err(Error::runtime_error("Provide a valid asset id"));
        }

        let id = RString::from_value(options[0]).unwrap();

        Ok(Module {
            id: id.to_string().unwrap(),
        })
    }

    pub(crate) fn id(&self) -> String {
        self.id.clone()
    }

    pub(crate) fn module_call(
        &self,
        all_args: &[Value],
    ) -> Result<Value, Error> {
        let call_options = RStruct::from_value(all_args[0]).unwrap();
        let args = RArray::from_value(all_args[1]).unwrap();
        let kwargs = RHash::from_value(all_args[2]).unwrap();
        let mut block: Option<Proc> = None;
        if all_args.len() == 6 {
            block = Proc::from_value(all_args[3]);
        }

        JS_RUNTIME.with(|js_runtime| {
            js_runtime.call(call_options, args, kwargs, block)
        })
    }
}
