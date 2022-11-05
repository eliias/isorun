use std::cell::{RefCell};
use deno_core::{Extension, op};
use deno_core::error::AnyError;
use magnus::block::Proc;
use magnus::{Error, RString};
use magnus::gvl::{GVLContext, without_gvl};
use crate::isorun::vm::VM;

struct App {
    context: RefCell<Option<GVLContext>>,
    receiver: RefCell<Option<Proc>>,
}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for App {}

unsafe impl Sync for App {}

impl App {
    fn new() -> Self {
        App {
            context: RefCell::from(None),
            receiver: RefCell::from(None),
        }
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

lazy_static! {
    static ref APP: App = {
        App::new()
    };
}

#[magnus::wrap(class = "Isorun::Renderer")]
pub(crate) struct Renderer {
    app_path: String,
}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for Renderer {}

impl Renderer {
    pub(crate) fn new(app_path: String) -> Self {
        Self { app_path }
    }

    pub(crate) fn renderer_render(&self, block: Proc) -> Result<String, Error> {
        APP.receiver.borrow_mut().replace(block);

        let result = without_gvl(|context| {
            APP.context.borrow_mut().replace(context);

            let extension_send = Extension::builder()
                .ops(vec![
                    op_app_send::decl()
                ])
                .build();
            let extensions = vec![extension_send];
            let vm = VM::new();
            vm.render(self.app_path.clone(), extensions)
        }, None::<fn()>);

        result.0.unwrap()
    }
}

#[op]
fn op_app_send(message: String, data: String) -> Result<String, AnyError> {
    APP.send(message, data)
        .and_then(|value| value.to_string())
        .map_err(|error| AnyError::msg(format!("{}", error)))
}
