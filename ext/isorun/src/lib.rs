use magnus::{define_module, method, Error, Module, Object, function};
use isorun::renderer::Renderer;

mod isorun;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Isorun")
        .expect("cannot define module: Isorun");

    let app = module
        .define_class("Renderer", Default::default())
        .expect("cannot define class: Isorun::Renderer");
    app.define_singleton_method("new", function!(Renderer::new, 0))
        .expect("cannot define singleton method: new");
    app.define_method("render", method!(Renderer::render, 2))
        .expect("cannot defined method: render");

    Ok(())
}
