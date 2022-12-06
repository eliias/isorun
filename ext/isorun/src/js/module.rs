use crate::js::context::Context;
use crate::js::module_item::{ModuleItem, ModuleItemFunction, ModuleItemValue};
use crate::js::worker::WORKER;
use deno_core::error::AnyError;
use deno_core::ModuleId;
use v8::{Global, Local, Object, Value};

pub(crate) struct Module<'context, 'worker> {
    pub(crate) id: ModuleId,
    pub(crate) context: &'context Context<'worker>,
    pub(crate) path: String,
}

impl<'context, 'worker> Module<'context, 'worker> {
    pub(crate) fn import(
        &self,
        export_name: &str,
    ) -> Result<ModuleItem, AnyError> {
        WORKER.with(|worker| {
            let js_runtime = worker.get_runtime();
            let mut scope = self.context.scope();

            let namespace = js_runtime.get_module_namespace(self.id).unwrap();
            let namespace = Local::<Object>::new(&mut scope, namespace);

            let export_name = v8::String::new(&mut scope, export_name).unwrap();

            let binding =
                namespace.get(&mut scope, export_name.into()).unwrap();
            let global_binding = Global::<Value>::new(&mut scope, binding);

            if binding.is_function() {
                Ok(ModuleItem::Function(ModuleItemFunction {
                    context: &self.context,
                    export_name: export_name.to_rust_string_lossy(&mut scope),
                    binding: global_binding,
                }))
            } else {
                Ok(ModuleItem::Value(ModuleItemValue {
                    context: &self.context,
                    export_name: export_name.to_rust_string_lossy(&mut scope),
                    binding: global_binding,
                }))
            }
        })
    }
}
