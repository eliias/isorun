use std::cell::{RefCell};
use deno_core::{Extension, op};
use deno_core::error::AnyError;
use magnus::block::Proc;
use magnus::{Error, RString};
use magnus::gvl::{GVLContext, without_gvl};
use tokio::runtime::Runtime;
use crate::isorun::vm::VM;

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

        let extension_send = Extension::builder()
            .ops(vec![
                op_app_send::decl()
            ])
            .build();
        let extensions = vec![extension_send];

        let vm = runtime
            .block_on(VM::new(extensions));

        JsRuntime {
            context: RefCell::from(None),
            receiver: RefCell::from(None),
            runtime,
            vm: RefCell::from(vm),
        }
    }

    pub(crate) fn render(&self, bundle_path: &str, block: Proc) -> Result<String, Error> {
        self.receiver.borrow_mut().replace(block);

        let result = without_gvl(|context| {
            self.context.borrow_mut().replace(context);
            self.runtime
                .block_on(self.vm.borrow_mut().render(bundle_path))
                .map_err(|error| Error::runtime_error(
                    format!("cannot render app: {}\nerror: {}", bundle_path, error)))
        }, None::<fn()>);

        result.0.unwrap()
    }

    fn send(&self, message: String, data: String) -> Result<RString, Error> {
        if let (Some(ctx), Some(rec)) = (self.context.borrow_mut().as_mut(), self.receiver.borrow_mut().as_mut()) {
            ctx.with_gvl(|| {
                let args: (RString, RString, ) = (RString::from(message), RString::from(data), );
                rec.call::<(RString, RString, ), RString>(args)
            })?
        } else {
            Err(Error::runtime_error("cannot send message"))
        }
    }
}

thread_local! {
    pub(crate) static JS_RUNTIME: JsRuntime = JsRuntime::new();
}

#[op]
fn op_app_send(message: String, data: String) -> Result<String, AnyError> {
    JS_RUNTIME.with(|js_runtime| {
        js_runtime.send(message, data)
            .and_then(|value| value.to_string())
            .map_err(|error| AnyError::msg(format!("{}", error)))
    })
}
