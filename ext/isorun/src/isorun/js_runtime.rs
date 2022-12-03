use crate::isorun::utils;
use crate::isorun::vm::VM;
use deno_core::error::AnyError;
use deno_core::serde_v8::from_v8;
use deno_core::{op, serde_v8, Extension};
use magnus::block::Proc;
use magnus::gvl::{without_gvl, GVLContext};
use magnus::{Error, RArray, RHash, RString, Value};
use std::cell::RefCell;
use tokio::runtime::Runtime;

pub(crate) struct JsRuntime {
    context: RefCell<Option<GVLContext>>,
    receiver: RefCell<Option<Proc>>,
    runtime: Runtime,
    vm: RefCell<VM>,
}

impl JsRuntime {
    fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let extension_send =
            Extension::builder().ops(vec![op_app_send::decl()]).build();
        let extensions = vec![extension_send];

        let vm = runtime.block_on(VM::new(extensions));

        JsRuntime {
            context: RefCell::from(None),
            receiver: RefCell::from(None),
            runtime,
            vm: RefCell::from(vm),
        }
    }

    pub(crate) fn call(
        &self,
        bundle_path: &str,
        entrypoint: &str,
        receiver: Proc,
        args: RArray,
        kwargs: RHash,
        block: Option<Proc>,
    ) -> Result<Value, Error> {
        self.receiver.borrow_mut().replace(receiver);

        let result = without_gvl(
            |context| {
                self.context.borrow_mut().replace(context);
                self
                    .runtime
                    .block_on(self.vm.borrow_mut().call(
                        bundle_path,
                        entrypoint,
                        args,
                        kwargs,
                    ))
                    .map_err(|error| {
                        Error::runtime_error(format!(
                            "cannot call function `{}` in module `{}`\nerror: {}",
                            entrypoint, bundle_path, error
                        ))
                    })
            },
            None::<fn()>,
        );

        if let Some(block) = block {
            let args: (Value,) = (result.0.unwrap().unwrap(),);
            block.call::<(Value,), Value>(args)
        } else {
            result.0.unwrap()
        }
    }

    fn send(&self, message: RString, value: Value) -> Result<Value, Error> {
        if let (Some(ctx), Some(rec)) = (
            self.context.borrow_mut().as_mut(),
            self.receiver.borrow_mut().as_mut(),
        ) {
            ctx.with_gvl(|| {
                let args: (RString, Value) = (message, value);
                rec.call::<(RString, Value), Value>(args)
            })?
            .map_err(|_error| Error::runtime_error("cannot map type"))
        } else {
            Err(Error::runtime_error("cannot send message: "))
        }
    }
}

thread_local! {
    pub(crate) static JS_RUNTIME: JsRuntime = JsRuntime::new();
}

#[allow(clippy::extra_unused_lifetimes)]
#[op(v8)]
fn op_app_send<'a>(
    // do not remove the v8:: prefix, otherwise the macro complains
    scope: &mut v8::HandleScope,
    message: String,
    data: serde_v8::Value<'a>,
) -> Result<serde_v8::Value<'a>, AnyError> {
    let message = RString::from(message);
    let value = utils::convert_v8_to_ruby(data.v8_value, scope)?;

    JS_RUNTIME.with(|js_runtime| {
        js_runtime
            .send(message, value)
            .map(|v| {
                let v = utils::convert_ruby_to_v8(v, scope).unwrap();
                from_v8(scope, v).unwrap()
            })
            .map_err(|error| {
                println!("{:?}", error);
                AnyError::msg(format!("{}", error))
            })
    })
}
