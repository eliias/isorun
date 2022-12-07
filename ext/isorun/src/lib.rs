use crate::isorun::context::Context;
use isorun::module::Module;
use isorun::module_item::{Function, Value};
use magnus::{define_module, function, method, Error, Module as M, Object};

mod isorun;
mod js;

#[magnus::init]
fn init() -> Result<(), Error> {
    let root = define_module("Isorun").expect("cannot define module: Isorun");

    let context = root
        .define_class("Context", Default::default())
        .expect("cannot define class: Isorun::Context");
    context
        .define_singleton_method("create", function!(Context::create, 0))
        .expect("cannot define singelton method: create");
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

    let value = root
        .define_class("Value", Default::default())
        .expect("cannot define class: Isorun::Value");
    value
        .define_method("to_ruby", method!(Value::to_ruby, 0))
        .expect("cannot define method: to_ruby");

    Ok(())
}
