use magnus::{define_module, method, Error, Module, Object, function};
use crate::isorun::vm::VM;

mod isorun;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Isorun")
        .expect("cannot define module: Isorun");

    let vm = module
        .define_class("VM", Default::default())
        .expect("cannot define class: Isorun::VM");

    vm.define_singleton_method("new", function!(VM::new, 0))
        .expect("cannot define singleton method: VM::new");
    vm.define_method("load", method!(VM::load, 2))
        .expect("cannot define method: VM::load");
    vm.define_method("render", method!(VM::render, 1))
        .expect("cannot define method: VM::render");

    Ok(())
}
