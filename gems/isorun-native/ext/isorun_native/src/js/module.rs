use crate::js::module_item::{Function, ModuleItem, Value as JsValue};
use crate::js::worker::WORKER;
use deno_core::error::AnyError;
use deno_core::{JsRealm, ModuleId};
use std::cell::RefCell;
use std::rc::Rc;
use v8::{Global, Local, Value};

pub(crate) struct Module {
    pub(crate) id: ModuleId,
    pub(crate) realm: Rc<RefCell<JsRealm>>,
}

impl Module {
    pub(crate) fn import(
        &self,
        export_name: &str,
    ) -> Result<ModuleItem, AnyError> {
        WORKER.with(|worker| {
            let namespace = {
                let mut worker = worker.worker.borrow_mut();
                worker.js_runtime.get_module_namespace(self.id).unwrap()
            };

            let realm = self.realm.borrow();

            let mut worker = worker.worker.borrow_mut();
            let mut scope = realm.handle_scope(worker.js_runtime.v8_isolate());

            let namespace = Local::new(&mut scope, namespace);

            let export_name = v8::String::new(&mut scope, export_name).unwrap();

            let binding =
                namespace.get(&mut scope, export_name.into()).unwrap();
            let global_binding = Global::<Value>::new(&mut scope, binding);

            if binding.is_function() {
                Ok(ModuleItem::Function(Function {
                    binding: global_binding,
                    realm: self.realm.clone(),
                }))
            } else {
                Ok(ModuleItem::Value(JsValue {
                    binding: global_binding,
                    realm: self.realm.clone(),
                }))
            }
        })
    }
}
