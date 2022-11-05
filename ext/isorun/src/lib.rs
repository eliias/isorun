use magnus::{define_module, method, Error, Module, Object, function};
use crate::isorun::runtime::{Renderer};

#[macro_use]
extern crate lazy_static;

mod isorun;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Isorun")
        .expect("cannot define module: Isorun");

    let runtime = module
        .define_class("Renderer", Default::default())
        .expect("cannot define class: Isorun::Renderer");
    runtime.define_singleton_method("new", function!(Renderer::new, 1))
        .expect("cannot define singleton method: new");
    runtime.define_method("renderer_render", method!(Renderer::renderer_render, 1))
        .expect("cannot defined method: renderer_render");

    Ok(())
}
