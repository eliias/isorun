use crate::isorun::js_runtime::JS_RUNTIME;
use magnus::block::Proc;
use magnus::{Error, RArray, RHash, RString, Value};

#[magnus::wrap(class = "Isorun::Module")]
pub(crate) struct Module {
    id: String,
    entrypoint: String,
}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Module {}

impl Module {
    pub(crate) fn new(id: String, entrypoint: String) -> Self {
        Module { id, entrypoint }
    }

    pub(crate) fn id(&self) -> String {
        self.id.clone()
    }

    pub(crate) fn entrypoint(&self) -> String {
        self.entrypoint.clone()
    }

    pub(crate) fn module_call(
        &self,
        all_args: &[Value],
    ) -> Result<Value, Error> {
        let bundle_path = RString::from_value(all_args[0])
            .unwrap()
            .to_string()
            .unwrap();
        let entrypoint = RString::from_value(all_args[1])
            .unwrap()
            .to_string()
            .unwrap();

        let receiver = Proc::from_value(all_args[2]).unwrap();
        let args = RArray::from_value(all_args[3]).unwrap();
        let kwargs = RHash::from_value(all_args[4]).unwrap();
        let mut block: Option<Proc> = None;
        if all_args.len() == 6 {
            block = Proc::from_value(all_args[5]);
        }

        JS_RUNTIME.with(|js_runtime| {
            js_runtime.call(
                bundle_path.as_str(),
                entrypoint.as_str(),
                receiver,
                args,
                kwargs,
                block,
            )
        })
    }
}
