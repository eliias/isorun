use crate::js::worker::WORKER;
use v8::Global;

pub(crate) enum ModuleItem {
    Value(Value),
    Function(Function),
}

pub(crate) struct Function {
    pub(crate) binding: Global<v8::Value>,
}

impl Function {
    pub(crate) fn call(
        &self,
        args: &[Global<v8::Value>],
    ) -> Result<magnus::Value, magnus::Error> {
        WORKER.with(|worker| worker.call(&self.binding, args))
    }
}

pub(crate) struct Value {
    pub(crate) binding: Global<v8::Value>,
}

impl Value {
    pub(crate) fn to_ruby(&self) -> Option<magnus::Value> {
        WORKER.with(|worker| worker.to_ruby(&self.binding))
    }
}
