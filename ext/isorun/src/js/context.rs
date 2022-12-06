use crate::js::module::Module;
use crate::js::worker::Worker;
use deno_core::error::AnyError;
use deno_core::JsRealm;
use v8::HandleScope;

pub(crate) struct Context<'worker> {
    realm: JsRealm,
    worker: &'worker Worker,
}

impl<'worker> Context<'worker> {
    pub(crate) fn create(realm: JsRealm, worker: &'worker Worker) -> Self {
        Self { realm, worker }
    }

    pub(crate) fn scope<'realm>(&self) -> HandleScope<'realm> {
        self.realm.handle_scope(self.worker.get_isolate())
    }

    pub(crate) fn load(&self, path: &str) -> Result<Module, AnyError> {
        self.worker.load_module(path).map(|id| Module {
            id,
            context: self,
            path: path.to_string(),
        })
    }
}
