use crate::isorun::utils::convert_v8_to_ruby;
use crate::js::context::Context;
use crate::js::worker::WORKER;
use magnus::QNIL;
use v8::{Global, Value};

pub(crate) enum ModuleItem<'context, 'worker> {
    Value(ModuleItemValue<'context, 'worker>),
    Function(ModuleItemFunction<'context, 'worker>),
    Undefined(ModuleItemValue<'context, 'worker>),
}

pub(crate) struct ModuleItemValue<'context, 'worker> {
    pub(crate) context: &'context Context<'worker>,
    pub(crate) export_name: String,
    pub(crate) binding: Global<Value>,
}

impl<'context, 'worker> ModuleItemValue<'context, 'worker> {
    pub(crate) fn to_ruby(&self) -> Option<magnus::Value> {
        WORKER.with(|worker| {
            let mut scope = self.context.scope();
            let value =
                v8::Local::<Value>::new(&mut scope, self.binding.clone());

            let result = convert_v8_to_ruby(value, &mut scope);
            match result {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        })
    }
}

pub(crate) struct ModuleItemFunction<'context, 'worker> {
    pub(crate) context: &'context Context<'worker>,
    pub(crate) export_name: String,
    pub(crate) binding: Global<Value>,
}

impl<'context, 'worker> ModuleItemFunction<'context, 'worker> {
    pub(crate) fn call(
        &self,
        args: &[Global<Value>],
    ) -> Result<magnus::Value, magnus::Error> {
        Ok(magnus::Value::from(QNIL))
    }
}

pub(crate) struct ModuleItemUndefined<'context, 'worker> {
    pub(crate) context: &'context Context<'worker>,
    pub(crate) export_name: String,
    pub(crate) binding: Global<Value>,
}

impl<'context, 'worker> ModuleItemUndefined<'context, 'worker> {
    pub(crate) fn to_ruby(&self) -> Option<magnus::Value> {
        Some(magnus::Value::from(QNIL))
    }
}
