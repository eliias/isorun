use crate::js::worker::WORKER;
use deno_core::error::AnyError;
use deno_core::op;
use deno_core::serde_v8;

deno_core::extension!(send_to_ruby, ops = [op_send_to_ruby]);

#[allow(clippy::extra_unused_lifetimes)]
#[op(v8)]
fn op_send_to_ruby<'a>(
    // do not remove the v8:: prefix, otherwise the macro complains
    scope: &mut v8::HandleScope,
    data: serde_v8::Value<'a>,
) -> Result<serde_v8::Value<'a>, AnyError> {
    WORKER.with(|worker| worker.send(scope, data))
}
