use crate::js::worker::WORKER;
use deno_core::JsRealm;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use v8::Global;

pub(crate) enum ModuleItem {
    Value(Value),
    Function(Function),
}

pub(crate) struct Function {
    pub(crate) binding: Global<v8::Value>,
    pub(crate) realm: Rc<RefCell<JsRealm>>,
}

impl Function {
    pub(crate) fn call(
        &self,
        args: &[Global<v8::Value>],
    ) -> Result<magnus::Value, magnus::Error> {
        WORKER.with(|worker| {
            let realm = self.realm.borrow();
            let realm = realm.deref();
            worker
                .runtime
                // we block here instead of the worker, due to a refcell issue
                // when borrowing within an await
                .block_on(worker.call(realm, &self.binding, args))
        })
    }
}

pub(crate) struct Value {
    pub(crate) binding: Global<v8::Value>,
    pub(crate) realm: Rc<RefCell<JsRealm>>,
}

impl Value {
    pub(crate) fn to_ruby(&self) -> Option<magnus::Value> {
        let realm = self.realm.borrow();
        let realm = realm.deref();
        WORKER.with(|worker| worker.to_ruby(realm, &self.binding))
    }
}
