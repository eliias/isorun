use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use deno_core::{Extension, FsModuleLoader, JsRuntime, located_script_name, ModuleId, op};
use deno_core::anyhow::{Error as AnyhowError};
use deno_core::error::AnyError;
use deno_runtime::{BootstrapOptions, js};
use deno_runtime::permissions::Permissions;
use deno_web::BlobStore;
use magnus::{Error};
use v8::{Promise, PromiseResolver};
use tokio::runtime::Runtime;

#[magnus::wrap(class = "Isorun::VM")]
pub struct VM {
    js_runtime: RefCell<JsRuntime>,
    registry: RefCell<HashMap<String, ModuleId>>,
    runtime: RefCell<Runtime>,
}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for VM {}

impl VM {
    pub fn new() -> VM {
        // create extensions
        let fetch_intercept_extension = Extension::builder()
            .ops(vec![op_fetch::decl()])
            .build();

        let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            startup_snapshot: Some(js::deno_isolate_init()),
            extensions: vec![
                // Web APIs
                deno_webidl::init(),
                deno_console::init(),
                deno_url::init(),
                deno_web::init::<Permissions>(
                    BlobStore::default(),
                    None,
                ),
                deno_node::init::<Permissions>(true, None),

                // custom fetch extension
                fetch_intercept_extension,
            ],
            ..Default::default()
        });

        // load runtime API
        js_runtime.execute_script(
            "[isorun:runtime.js]",
            include_str!("../runtime.js"),
        ).unwrap();

        //
        let bootstrap_options = BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            user_agent: "hello_runtime".to_string(),
            inspect: false,
        };
        let script = format!("bootstrap.mainRuntime({})", bootstrap_options.as_json());
        js_runtime
            .execute_script(&located_script_name!(), &script)
            .expect("Failed to execute bootstrap script");

        let registry: HashMap<String, ModuleId> = HashMap::new();

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        VM {
            js_runtime: RefCell::from(js_runtime),
            registry: RefCell::from(registry),
            runtime: RefCell::from(runtime),
        }
    }

    pub fn load(&self, app_id: String, app_path: String) -> Result<(), Error> {
        let id = app_id.as_str();
        let mut registry = self.registry.borrow_mut();
        let runtime = self.runtime.borrow();

        if registry.contains_key(id) {
            return Err(Error::runtime_error(
                format!("cannot register app: {}. an app with the same ID already exists.", id)));
        }

        runtime
            .block_on(self.load_module(app_path.as_str()))
            .map(|mod_id| { registry.insert(app_id, mod_id); })
            .map_err(|err| Error::runtime_error(format!("cannot load app: {}.\n\tMake sure app is available at \"{}\"", err, app_path)))?;

        Ok(())
    }

    async fn load_module(&self, module_path: &str) -> Result<ModuleId, AnyhowError> {
        let mut js_runtime = self.js_runtime.borrow_mut();

        let main_module = deno_core::resolve_path(module_path).unwrap();
        let mod_id = js_runtime.load_main_module(&main_module, None).await?;
        let result = js_runtime.mod_evaluate(mod_id);
        js_runtime.run_event_loop(false).await?;
        result.await??;

        Ok(mod_id)
    }

    pub fn render(&self, app_id: String) -> Result<String, Error> {
        let id = app_id.as_str();
        let mut js_runtime = self.js_runtime.borrow_mut();
        let registry = self.registry.borrow();

        if !registry.contains_key(id) {
            return Err(Error::runtime_error(
                format!("cannot render app without loading it: {}. have you called vm#load(app_id, app_path) before.", id)));
        }

        // get module_id
        let module_id = registry.get(id).unwrap();

        // find function
        let module_namespace = js_runtime.get_module_namespace(*module_id).unwrap();
        let scope = &mut js_runtime.handle_scope();
        let global = scope.get_current_context().global(scope);
        let module_namespace =
            v8::Local::<v8::Object>::new(scope, module_namespace);
        let render_export_name = v8::String::new(scope, "render").unwrap();
        let binding = module_namespace.get(scope, render_export_name.into()).unwrap();
        let func = v8::Local::<v8::Function>::try_from(binding).unwrap();

        // call function
        let args = &[];
        let result = func.call(scope, global.into(), args).unwrap();

        // return value
        let return_value = v8::Local::<Promise>::try_from(result).unwrap();
        let resolver = PromiseResolver::new(scope).unwrap();
        let promise = resolver.get_promise(scope);
        resolver.resolve(scope, return_value.into());
        let result = promise.result(scope);

        Ok(result.to_rust_string_lossy(scope))
    }
}

#[op]
async fn op_fetch(url: String) -> Result<String, AnyError> {
    let response = format!("{{\"Hello\": \"World\", \"url\":\"{}\"}}", url);
    Ok(response)
}
