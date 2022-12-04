use isorun::module::Module;
use magnus::{define_module, function, method, Error, Module as M, Object};

mod isorun;

#[magnus::init]
fn init() -> Result<(), Error> {
    let root = define_module("Isorun").expect("cannot define module: Isorun");

    let module = root
        .define_class("Module", Default::default())
        .expect("cannot define class: Isorun::Module");
    module
        .define_singleton_method("new", function!(Module::new, -1))
        .expect("cannot define singelton method: new");
    module
        .define_method("id", method!(Module::id, 0))
        .expect("cannot define method: id");
    module
        .define_private_method("module_call", method!(Module::module_call, -1))
        .expect("cannot define method: module_call");
    Ok(())
}
