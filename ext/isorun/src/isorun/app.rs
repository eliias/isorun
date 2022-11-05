// use std::cell::RefCell;
// use std::rc::Rc;
// use deno_core::error::AnyError;
// use deno_core::{Extension, op, OpState, Resource, ResourceId};
// use magnus::block::Proc;
// use magnus::gvl::{GVLContext, without_gvl};
// use magnus::{Error, RString};
// use crate::isorun::VM;
//
// struct Callback(RefCell<Proc>);
//
// impl Resource for Callback {}
//
// #[magnus::wrap(class = "Isorun::Container")]
// pub struct Container(pub(crate) Rc<App>);
//
// /// SAFETY: This is safe because we only access this data when the GVL is held.
// unsafe impl Send for Container {}
//
// impl Container {
//     pub(crate) fn new(cb: Proc) -> Self {
//         let mut resource_id: ResourceId = 0;
//         let callback = Rc::from(Callback(RefCell::from(cb)));
//
//         let extension = Extension::builder()
//             .ops(vec![
//                 op_app_send::decl()
//             ])
//             .state(|state| {
//                 resource_id = state.resource_table.add_rc(callback.clone());
//                 Ok(())
//             })
//             .build();
//
//         let extensions = vec![extension];
//         let vm = VM::new();
//
//         let app = App {
//             vm,
//             extensions,
//             context: Rc::from(RefCell::from(None)),
//             resource_id,
//         };
//
//         Self(Rc::from(app))
//     }
//
//     pub(crate) fn render(&self, app_path: String,) -> Result<String, Error> {
//         self.0
//             .clone()
//             .render(app_path)
//     }
// }
//
// pub struct App {
//     vm: VM,
//     extensions: Vec<Extension>,
//     context: Rc<RefCell<Option<GVLContext>>>,
//     resource_id: ResourceId
// }
//
// /// Make app a shared resource
// impl Resource for App {}
//
// impl App {
//     pub(crate) fn render(self: Rc<Self>, app_path: String) -> Result<String, Error> {
//         let result = without_gvl(|context| {
//             self.context.replace(Some(context));
//
//             let extensions = vec![];
//             let vm = VM::new();
//             vm.render(app_path, extensions)
//         }, None::<fn()>);
//
//         result.0.unwrap()
//     }
// }
//
// #[op]
// fn op_app_send(state: Rc<RefCell<OpState>>, resource_id: ResourceId) -> Result<(), AnyError> {
//     let cb = state.borrow().resource_table.get::<Callback>(resource_id)?;
//     let args:(magnus::Value,) = (RString::from("Hello, World!").into(),);
//     cb.0.borrow().call::<(magnus::Value, ), magnus::Value>(args).expect("cannot invoke callback");
//
//     Ok(())
// }
