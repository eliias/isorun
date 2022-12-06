use isorun::module::Module;
use isorun::module_item::ModuleItem;
use magnus::{define_module, function, method, Error, Module as M, Object};

mod isorun;
mod js;

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
        .define_private_method("module_id", method!(Module::module_id, 0))
        .expect("cannot define method: module_id");
    module
        .define_private_method(
            "module_import",
            method!(Module::module_import, 1),
        )
        .expect("cannot define method: module_import");

    let module_item = root
        .define_class("ModuleItem", Default::default())
        .expect("cannot define class: Isorun::ModuleItem");
    module_item
        .define_private_method(
            "module_item_call",
            method!(ModuleItem::module_item_call, -1),
        )
        .expect("cannot define method: module_item_call");
    module_item
        .define_private_method(
            "module_item_export_name",
            method!(ModuleItem::module_item_export_name, 0),
        )
        .expect("cannot define method: module_item_export_name");
    module_item
        .define_private_method(
            "module_item_to_value",
            method!(ModuleItem::module_item_to_value, 0),
        )
        .expect("cannot define method: module_item_to_value");

    Ok(())
}
