use crate::js::worker::WORKER;
use deno_core::error::AnyError;
use deno_core::serde_v8;
use deno_core::{op, ExtensionBuilder};

fn force_op_registration(ext: &mut ExtensionBuilder) {
    ext.force_op_registration();
}

deno_core::extension!(
    send_to_ruby,
    ops = [op_send_to_ruby],
    customizer = force_op_registration
);

#[allow(clippy::extra_unused_lifetimes)]
#[op(v8)]
fn op_send_to_ruby<'a>(
    // do not remove the v8:: prefix, otherwise the macro complains
    scope: &mut v8::HandleScope,
    data: serde_v8::Value<'a>,
) -> Result<serde_v8::Value<'a>, AnyError> {
    WORKER.with(|worker| worker.send(scope, data))
}
