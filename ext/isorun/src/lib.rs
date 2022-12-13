use crate::isorun::context::Context;
use isorun::configure::set_receiver;
use isorun::function::Function;
use isorun::module::Module;
use magnus::{define_module, function, method, Error, Module as M, Object};

mod isorun;
mod js;

#[magnus::init]
fn init() -> Result<(), Error> {
    let root = define_module("Isorun").expect("cannot define module: Isorun");

    root.define_module_function("receiver=", function!(set_receiver, 1))
        .expect("cannot define module function: receiver=");

    let context = root
        .define_class("Context", Default::default())
        .expect("cannot define class: Isorun::Context");
    context
        .define_singleton_method("new", function!(Context::new, 0))
        .expect("cannot define singelton method: new");
    context
        .define_method("load", method!(Context::load, 1))
        .expect("cannot load module");

    let module = root
        .define_class("Module", Default::default())
        .expect("cannot define class: Isorun::Module");
    module
        .define_private_method("id", method!(Module::id, 0))
        .expect("cannot define method: module_id");
    module
        .define_method("import", method!(Module::import, 1))
        .expect("cannot define method: import");

    let function = root
        .define_class("Function", Default::default())
        .expect("cannot define class: Isorun::Function");
    function
        .define_method("call", method!(Function::call, -1))
        .expect("cannot define method: call");
    function
        .define_method(
            "call_without_gvl",
            method!(Function::call_without_gvl, -1),
        )
        .expect("cannot define method: call_without_gvl");

    Ok(())
}