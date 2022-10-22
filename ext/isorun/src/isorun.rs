use std::borrow::Borrow;
use std::rc::Rc;
use deno_core::futures;
use magnus::Error;
use v8::{Global, Handle, Promise, PromiseResolver, Value};

#[magnus::wrap(class = "Isorun::VM")]
pub(crate) struct VM {}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for VM {}

impl VM {
    pub fn new() -> VM {
        VM {}
    }

    async fn run_js(&self, source: &str) -> Result<String, Error> {
        let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
            ..Default::default()
        });

        // load module and wait for evaluation
        let main_module = deno_core::resolve_path("examples/deno/renderer.js").unwrap();
        let mod_id = runtime.load_main_module(&main_module, None).await.unwrap();
        let result = runtime.mod_evaluate(mod_id);
        runtime.run_event_loop(false).await;
        result.await;

        // access module namespace, find exported "render" method and invoke "render" function
        let module_namespace = runtime.get_module_namespace(mod_id).unwrap();
        let scope = &mut runtime.handle_scope();
        let global = scope.get_current_context().global(scope);
        let module_namespace =
            v8::Local::<v8::Object>::new(scope, module_namespace);

        let render_export_name = v8::String::new(scope, "render").unwrap();
        let binding = module_namespace.get(scope, render_export_name.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(binding).unwrap();
        let args = &[];
        let result = func.call(scope, global.into(), args).unwrap();

        let return_value = v8::Local::<Promise>::try_from(result).unwrap();

        let resolver = PromiseResolver::new(scope).unwrap();
        let promise = resolver.get_promise(scope);
        resolver.resolve(scope, return_value.into());
        let result = promise.result(scope);

        Ok(result.to_rust_string_lossy(scope))
    }

    pub fn run(&self, source: String) -> Result<String, Error> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        match runtime.block_on(self.run_js(source.as_str())) {
            Ok(value) => Ok(value),
            Err(err) => Err(
                Error::runtime_error(format!("render did not return result: {}", err))
            )
        }
    }
}
