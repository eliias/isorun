use self::isorun::utils::stats;
use crate::isorun::context::Context;
use crate::isorun::utils::low_memory_notification;
use isorun::function::Function;
use isorun::module::Module;
use magnus::{
    class, define_module, function, method, Error, Module as M, Object,
};

mod ext;
mod isorun;
mod js;

#[magnus::init]
fn init() -> Result<(), Error> {
    let root = define_module("Isorun").expect("cannot define module: Isorun");

    root.define_module_function("stats", function!(stats, 0))
        .expect("cannot define module function: stats");

    root.define_module_function(
        "low_memory_notification",
        function!(low_memory_notification, 0),
    )
    .expect("cannot define module function: low_memory_notification");

    let context = root
        .define_class("Context", class::object())
        .expect("cannot define class: Isorun::Context");
    context
        .define_singleton_method("new", function!(Context::new, 0))
        .expect("cannot define singelton method: new");
    context
        .define_method("load", method!(Context::load, 1))
        .expect("cannot define method: load");
    context
        .define_method("receiver=", method!(Context::set_receiver, 1))
        .expect("cannot define method: receiver=");

    let module = root
        .define_class("Module", class::object())
        .expect("cannot define class: Isorun::Module");
    module
        .define_private_method("id", method!(Module::id, 0))
        .expect("cannot define method: module_id");
    module
        .define_method("import", method!(Module::import, 1))
        .expect("cannot define method: import");

    let function = root
        .define_class("Function", class::object())
        .expect("cannot define class: Isorun::Function");
    function
        .define_method("call", method!(Function::call, -1))
        .expect("cannot define method: call");

    Ok(())
}
